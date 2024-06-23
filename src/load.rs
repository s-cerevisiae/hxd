use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

use eyre::{bail, eyre, WrapErr};

use crate::cli::LoadArgs;

pub fn load(options: LoadArgs) -> eyre::Result<()> {
    let writer = io::stdout().lock();
    if let Some(path) = options.input {
        let reader = BufReader::new(
            File::open(&path)
                .wrap_err_with(|| eyre!("failed to open file `{}`", path.to_string_lossy()))?,
        );
        load_impl(reader, writer)
    } else {
        let reader = io::stdin().lock();
        load_impl(reader, writer)
    }
}

pub(crate) fn load_impl<R: BufRead, W: Write>(reader: R, mut writer: W) -> eyre::Result<()> {
    for line in reader.lines() {
        let line = line?;
        let dump = extract_dump(line.as_str());
        for mut group in dump.split_ascii_whitespace() {
            if group.len() % 2 != 0 {
                bail!("incomplete octet in input:\n{line}")
            }
            while let Some(b) = group.get(..2) {
                group = &group[2..];
                let b = u8::from_str_radix(b, 16)
                    .wrap_err_with(|| eyre!("failed to parse octet `{b}` in\n{line}"))?;
                writer.write_all(&[b])?;
            }
        }
    }

    Ok(())
}

fn extract_dump(line: &str) -> &str {
    let rest = line.split_once('|').map_or(line, |(rest, _comments)| rest);
    let dump = rest.split_once(':').map_or(rest, |(_offset, dump)| dump);
    dump.trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_dump() {
        assert_eq!(extract_dump(""), "");
        assert_eq!(extract_dump("abcd"), "abcd");
        assert_eq!(extract_dump("0: abcd"), "abcd");
        assert_eq!(extract_dump(": abcd"), "abcd");
        assert_eq!(extract_dump(":abcd"), "abcd");
        assert_eq!(extract_dump("0: abcd|????"), "abcd");
        assert_eq!(extract_dump("0: abcd |????"), "abcd");
        assert_eq!(extract_dump("0: abcd| ????"), "abcd");
        assert_eq!(extract_dump("0: abcd | ????"), "abcd");
        assert_eq!(extract_dump("0: abcd "), "abcd");
        assert_eq!(extract_dump("0:    abcd "), "abcd");
        assert_eq!(extract_dump(" abcd "), "abcd");
        assert_eq!(extract_dump("  abcd "), "abcd");
        assert_eq!(extract_dump(" abcd | ????"), "abcd");
        assert_eq!(extract_dump("00 abcd |  ????"), "00 abcd");
        assert_eq!(extract_dump("abcd |0:  ????"), "abcd");
    }
}
