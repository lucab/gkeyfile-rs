//! Keyfile writer.

use crate::errors;
use crate::Keyfile;
use snafu::ResultExt;
use std::io::Write;

/// Write a keyfile to a buffered writer.
pub(crate) fn write_buf<T>(kf: &Keyfile, bufwr: &mut std::io::BufWriter<T>) -> errors::Result<()>
where
    T: std::io::Write,
{
    for (group, content) in &kf.groups {
        // Write group header.
        bufwr
            .write_fmt(format_args!("[{}]\n", group))
            .context(errors::FailedWrite)?;

        // Write group content.
        for (key, val) in content {
            bufwr
                .write_fmt(format_args!("{} = {}\n", key, val))
                .context(errors::FailedWrite)?;
        }

        // Flush whole group data.
        bufwr.flush().context(errors::FailedWrite)?;
    }
    Ok(())
}
