//! Keyfile parser.

use crate::errors;
use crate::Keyfile;
use crate::Value;
use std::collections::HashMap;

pub(crate) fn parse_buf<T>(bufrd: T) -> errors::Result<Keyfile>
where
    T: std::io::BufRead,
{
    let mut current_group: Option<String> = None;
    let mut groups = HashMap::<String, HashMap<String, Value>>::new();

    for (index, entry) in bufrd.lines().enumerate() {
        let line_num = index.saturating_add(1);
        let line = entry.map_err(|source| errors::Error::FailedRead {
            line: line_num,
            source,
        })?;

        // Skip silent lines (comments and blank content).
        if is_silent(&line) {
            continue;
        }

        // Check and parse group declaration.
        if is_new_group(&line) {
            let group = parse_group(line_num, line)?;
            groups.entry(group.clone()).or_default();
            current_group = Some(group);
            continue;
        }

        // Ensure content is in a group context.
        let group = match current_group {
            Some(ref g) => g,
            None => {
                return Err(errors::Error::Malformed {
                    line: line_num,
                    reason: "content outside of group context".to_string(),
                })
            }
        };

        let (key, val) = parse_kv(line_num, line)?;
        groups.entry(group.clone()).or_default().insert(key, val);
    }

    let kf = Keyfile { groups };
    Ok(kf)
}

/// Return whether a line has no meaningful content.
fn is_silent(line: &str) -> bool {
    line.is_empty() || line.starts_with('#')
}

/// Return whether a line declares a new group.
fn is_new_group(line: &str) -> bool {
    line.starts_with('[')
}

/// Parse a group identifier.
fn parse_group(line_num: usize, line: String) -> errors::Result<String> {
    if !line.starts_with('[') {
        return Err(errors::Error::Malformed {
            line: line_num,
            reason: "missing group start marker".to_string(),
        });
    }

    if !line.ends_with(']') {
        return Err(errors::Error::Malformed {
            line: line_num,
            reason: "missing group end marker".to_string(),
        });
    }

    let trimmed = line.trim_start_matches('[').trim_end_matches(']').trim();
    Ok(trimmed.to_string())
}

/// Parse a key-value pair.
fn parse_kv(line_num: usize, line: String) -> errors::Result<(String, Value)> {
    let parts: Vec<_> = line.splitn(2, '=').collect();
    if parts.len() < 2 {
        return Err(errors::Error::Malformed {
            line: line_num,
            reason: "invalid key-value format".to_string(),
        });
    };

    let (key, value_str) = (parts[0].trim_end(), parts[1].trim_start());
    let value = Value::parse(value_str.to_string())?;
    Ok((key.to_string(), value))
}
