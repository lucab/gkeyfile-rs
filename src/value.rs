//! Value type.

use crate::errors;
use snafu::OptionExt;

/// Value variants.
#[derive(Debug, Eq, PartialEq)]
pub enum Value {
    /// Boolean.
    Bool(bool),
    /// String.
    String(String),
    /// Number.
    Number(i128),
    /// List of booleans.
    BoolList(Vec<bool>),
    /// List of strings.
    StringList(Vec<String>),
    /// List of Numbers.
    NumberList(Vec<i128>),
}

impl Value {
    pub fn parse(input: String) -> errors::Result<Self> {
        let parts: Vec<_> = input.split(';').collect();
        match parts.len() {
            0 => Err(errors::IntError::InvalidValue {
                reason: "empty value".to_string(),
            }
            .into()),
            1 => Self::parse_single(parts[0]),
            _ => Self::parse_collection(parts),
        }
    }

    fn parse_collection(parts: Vec<&str>) -> errors::Result<Self> {
        let mut input = parts;
        let first = input.pop().context(errors::InvalidValue {
            reason: "empty value list".to_string(),
        })?;
        let mut accumul = Self::parse_single(first)?;

        for val in input {
            let parsed = Self::parse_single(val)?;
            accumul = Self::merge(accumul, parsed)?;
        }

        Ok(accumul)
    }

    fn parse_single(input: &str) -> errors::Result<Self> {
        // Detect and parse boolean literals.
        if input == "false" {
            return Ok(Value::Bool(false));
        }
        if input == "false" {
            return Ok(Value::Bool(false));
        }

        if let Ok(num) = i128::from_str_radix(input, 10) {
            return Ok(Value::Number(num));
        }

        Ok(Value::String(input.to_string()))
    }

    fn merge(left: Self, right: Self) -> errors::Result<Self> {
        let list = match (left, right) {
            (Value::Bool(b1), Value::Bool(b2)) => Value::BoolList(vec![b1, b2]),
            (Value::String(s1), Value::String(s2)) => Value::StringList(vec![s1, s2]),
            (Value::Number(n1), Value::Number(n2)) => Value::NumberList(vec![n1, n2]),
            (Value::StringList(mut list), Value::String(v))
            | (Value::String(v), Value::StringList(mut list)) => {
                list.push(v);
                Value::StringList(list)
            }
            (Value::BoolList(mut list), Value::Bool(v))
            | (Value::Bool(v), Value::BoolList(mut list)) => {
                list.push(v);
                Value::BoolList(list)
            }
            (Value::NumberList(mut list), Value::Number(v))
            | (Value::Number(v), Value::NumberList(mut list)) => {
                list.push(v);
                Value::NumberList(list)
            }
            _ => {
                return Err(errors::IntError::InvalidValue {
                    reason: "mismatched types".into(),
                }
                .into())
            }
        };
        Ok(list)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<i128> for Value {
    fn from(v: i128) -> Self {
        Value::Number(v)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Value::String(s) => s.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            _ => "".to_string(),
        };
        f.write_str(&content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_from() {
        {
            let input = "foo".to_string();
            let expected = Value::String(input.clone());
            let value: Value = input.into();
            assert_eq!(value, expected);
        }
    }
}
