//! A pure-Rust library to work with GNOME Keyfiles.

#[deny(missing_docs)]
#[deny(missing_debug_implementations)]
mod errors;
mod keyfile;
mod parser;
mod value;
mod writer;

pub use errors::{Error, Result};
pub use keyfile::Keyfile;
pub use value::Value;

/// Deserialize a Keyfile from an IO reader.
pub fn from_reader<R>(reader: R) -> errors::Result<Keyfile>
where
    R: std::io::Read,
{
    let bufrd = std::io::BufReader::new(reader);
    Keyfile::parse_buf(bufrd)
}

/// Serialize a Keyfile to an IO reader.
pub fn to_writer<W>(writer: &mut W, value: &Keyfile) -> errors::Result<()>
where
    W: std::io::Write,
{
    let mut bufwr = std::io::BufWriter::new(writer);
    value.write_buf(&mut bufwr)
}

#[cfg(test)]
mod tests {
    use super::Keyfile;

    #[test]
    fn test_basic_from_reader() {
        let input = r#"
# Random comment
[single-group]
mykey = value 1;value 2;value 3
"#;
        let buf = std::io::Cursor::new(input);
        let kf = super::from_reader(buf).unwrap();

        let groups: Vec<_> = kf.groups_labels().collect();
        assert_eq!(groups, vec!["single-group"]);
    }

    #[test]
    fn test_basic_to_writer() {
        let expected = r#"[single-group]
mykey = value 1
"#;
        let mut kf = Keyfile::new();
        kf.insert(
            "single-group".to_string(),
            "mykey".to_string(),
            "value 1".into(),
        );

        let buf = Vec::<u8>::new();
        let mut bufwr = std::io::BufWriter::new(buf);
        kf.write_buf(&mut bufwr).unwrap();
        assert_eq!(bufwr.get_ref().as_slice(), expected.as_bytes());
    }
}
