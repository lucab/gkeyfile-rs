//! Value type.

use crate::errors;
use snafu::OptionExt;

/// Value variants.
#[derive(Debug, PartialEq)]
pub enum Value {
    /// Boolean.
    Bool(bool),
    /// String.
    String(String),
    /// Integer.
    Integer(i128),
    // Double.
    Double(f64),
    /// List of booleans.
    BoolList(Vec<bool>),
    /// List of strings.
    StringList(Vec<String>),
    /// List of Integers.
    IntegerList(Vec<i128>),
    /// List of Doubles.
    DoubleList(Vec<f64>),
}

impl Value {
    /// Parse an input string.
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
        use std::str::FromStr;

        // Detect and parse boolean literals.
        if input == "false" {
            return Ok(Value::Bool(false));
        }
        if input == "true" {
            return Ok(Value::Bool(true));
        }

        // Detect and parse integers.
        if let Ok(num) = i128::from_str_radix(input, 10) {
            return Ok(Value::Integer(num));
        }

        // Detect and parse floating-point numbers.
        if let Ok(double) = f64::from_str(input) {
            return Ok(Value::Double(double));
        }

        Ok(Value::String(input.to_string()))
    }

    fn merge(left: Self, right: Self) -> errors::Result<Self> {
        let list = match (left, right) {
            (Value::Bool(b1), Value::Bool(b2)) => Value::BoolList(vec![b1, b2]),
            (Value::String(s1), Value::String(s2)) => Value::StringList(vec![s1, s2]),
            (Value::Integer(n1), Value::Integer(n2)) => Value::IntegerList(vec![n1, n2]),
            (Value::Double(f1), Value::Double(f2)) => Value::DoubleList(vec![f1, f2]),
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
            (Value::IntegerList(mut list), Value::Integer(v))
            | (Value::Integer(v), Value::IntegerList(mut list)) => {
                list.push(v);
                Value::IntegerList(list)
            }
            (Value::DoubleList(mut list), Value::Double(v))
            | (Value::Double(v), Value::DoubleList(mut list)) => {
                list.push(v);
                Value::DoubleList(list)
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
        Value::Integer(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Double(v)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Value::String(s) => s.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Integer(n) => n.to_string(),
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

    #[test]
    fn test_value_parse() {
        {
            let input = "true".to_string();
            let expected = Value::Bool(true);
            let value = Value::parse(input).unwrap();
            assert_eq!(value, expected);
        }
        {
            let input = "false".to_string();
            let expected = Value::Bool(false);
            let value = Value::parse(input).unwrap();
            assert_eq!(value, expected);
        }
        {
            let input = "-10".to_string();
            let expected = Value::Integer(-10);
            let value = Value::parse(input).unwrap();
            assert_eq!(value, expected);
        }
        {
            let input = "4.2".to_string();
            let expected = Value::Double(4.2);
            let value = Value::parse(input).unwrap();
            assert_eq!(value, expected);
        }
        {
            let input = "foo".to_string();
            let expected = Value::String(input.clone());
            let value = Value::parse(input).unwrap();
            assert_eq!(value, expected);
        }
    }
}
