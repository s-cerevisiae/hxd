use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

use crate::cli::LoadArgs;

pub fn load(options: LoadArgs) -> Result<(), io::Error> {
    let writer = io::stdout().lock();
    if let Some(path) = options.input {
        let reader = BufReader::new(File::open(path)?);
        load_impl(reader, writer)
    } else {
        let reader = io::stdin().lock();
        load_impl(reader, writer)
    }
}

pub(crate) fn load_impl<R: BufRead, W: Write>(reader: R, mut writer: W) -> Result<(), io::Error> {
    for line in reader.lines() {
        let line = line?;
        let dump = extract_dump(line.as_str());
        for mut group in dump.split_ascii_whitespace() {
            if group.len() % 2 != 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("incomplete octet:\n{line}"),
                ));
            }
            while let Some(b) = group.get(..2) {
                group = &group[2..];
                let b = u8::from_str_radix(b, 16).map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidInput, format!("{e}:\n{line}"))
                })?;
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
