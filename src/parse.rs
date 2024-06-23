use eyre::{bail, eyre, WrapErr};

#[derive(Debug, PartialEq, Eq)]
pub struct DumpLine<'src> {
    pub offset: &'src str,
    pub data: &'src str,
    pub comment: &'src str,
}

/// Recognizes parts from a single line of the dump format, does not verify.
pub fn recognize_line(line: &str) -> DumpLine<'_> {
    let (rest, comment) = line.split_once('|').unwrap_or((line, ""));
    let (offset, data) = rest.split_once(':').unwrap_or(("", rest));
    DumpLine {
        offset: offset.trim(),
        data: data.trim(),
        comment,
    }
}

/// Iterates over a line of data and feeds resulting bytes into `f`. Stops when encounting any
/// error.
pub fn for_parsed_data(data: &str, mut f: impl FnMut(u8) -> eyre::Result<()>) -> eyre::Result<()> {
    for mut group in data.split_ascii_whitespace() {
        if group.len() % 2 != 0 {
            bail!("incomplete octet in input:\n{data}")
        }
        while let Some(b) = group.get(..2) {
            group = &group[2..];
            let b = u8::from_str_radix(b, 16)
                .wrap_err_with(|| eyre!("failed to parse octet `{b}` in\n{data}"))?;
            f(b)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recognize_line() {
        let p = |offset, data, comment| DumpLine {
            offset,
            data,
            comment,
        };
        assert_eq!(recognize_line(""), p("", "", ""));
        assert_eq!(recognize_line("abcd"), p("", "abcd", ""));
        assert_eq!(recognize_line("0: abcd"), p("0", "abcd", ""));
        assert_eq!(recognize_line("00000000: abcd"), p("00000000", "abcd", ""));
        assert_eq!(
            recognize_line("  deadbeef: abcd"),
            p("deadbeef", "abcd", "")
        );
        assert_eq!(recognize_line(": abcd"), p("", "abcd", ""));
        assert_eq!(recognize_line(":abcd"), p("", "abcd", ""));
        assert_eq!(recognize_line("0: abcd|????"), p("0", "abcd", "????"));
        assert_eq!(recognize_line("0: abcd |????"), p("0", "abcd", "????"));
        assert_eq!(recognize_line("0: abcd| ????"), p("0", "abcd", " ????"));
        assert_eq!(recognize_line("0: abcd | ????"), p("0", "abcd", " ????"));
        assert_eq!(recognize_line("0: abcd "), p("0", "abcd", ""));
        assert_eq!(recognize_line("0:    abcd "), p("0", "abcd", ""));
        assert_eq!(recognize_line(" abcd "), p("", "abcd", ""));
        assert_eq!(recognize_line("  abcd "), p("", "abcd", ""));
        assert_eq!(recognize_line(" abcd | ????"), p("", "abcd", " ????"));
        assert_eq!(
            recognize_line("00 abcd |  ????"),
            p("", "00 abcd", "  ????")
        );
        assert_eq!(recognize_line("abcd |0:  ????"), p("", "abcd", "0:  ????"));
    }
}
