use std::{
    fs::File,
    io::{self, BufRead, Seek, Write},
};

use eyre::{eyre, WrapErr};

use crate::{
    cli::PatchArgs,
    parse::{for_parsed_data, recognize_line, DumpLine},
};

pub fn patch(args: PatchArgs) -> eyre::Result<()> {
    let target = File::options()
        .write(true)
        .read(false)
        .open(&args.target)
        .wrap_err_with(|| {
            eyre!(
                "failed to open target file at `{}`",
                args.target.to_string_lossy()
            )
        })?;

    let input = io::stdin().lock();

    patch_impl(input, target)
}

pub(crate) fn patch_impl<R: BufRead, W: Write + Seek>(
    reader: R,
    mut writer: W,
) -> eyre::Result<()> {
    for line in reader.lines() {
        let line = line?;
        let DumpLine { offset, data, .. } = recognize_line(&line);
        if !offset.is_empty() {
            let offset = u64::from_str_radix(offset, 16)
                .wrap_err_with(|| eyre!("invalid offset `{offset}`"))?;
            writer.seek(io::SeekFrom::Start(offset))?;
        }
        for_parsed_data(data, |b| {
            writer
                .write_all(&[b])
                .wrap_err("failed to write to target file")
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    fn patch(i: &str, o: impl Into<Vec<u8>>) -> eyre::Result<Vec<u8>> {
        let mut o = o.into();
        patch_impl(i.as_bytes(), Cursor::new(&mut o))?;
        Ok(o)
    }

    fn hex(s: &str) -> Vec<u8> {
        let mut result = Vec::new();
        for_parsed_data(s, |b| {
            result.push(b);
            Ok(())
        })
        .unwrap();
        result
    }

    #[test]
    fn test_patch() {
        // assert!(dbg!(patch("aa", [])).is_err());
        assert_eq!(
            patch("0: aabbcc", hex("ddccbbaa")).unwrap(),
            hex("aabbccaa")
        );
        assert_eq!(patch("0: aabbcc", hex("dd")).unwrap(), hex("aabbcc"));
        assert_eq!(patch("1: aabbcc", hex("ddeeff")).unwrap(), hex("ddaabbcc"));
        assert_eq!(
            patch("0: aabb\ncc", hex("ddccbbaa")).unwrap(),
            hex("aabbccaa")
        );
        assert_eq!(
            patch("0: aa\nbb\ncc", hex("ddccbbaa")).unwrap(),
            hex("aabbccaa")
        );
        assert_eq!(
            patch("0: aabb\n2:cc", hex("ddccbbaa")).unwrap(),
            hex("aabbccaa")
        );
        assert_eq!(
            patch("0: aabb\n1:cc", hex("ddccbbaa")).unwrap(),
            hex("aaccbbaa")
        );
        assert_eq!(patch("", hex("ddccbbaa")).unwrap(), hex("ddccbbaa"));
        assert_eq!(patch("0:", hex("ddccbbaa")).unwrap(), hex("ddccbbaa"));
        assert_eq!(patch("2500:", hex("ddccbbaa")).unwrap(), hex("ddccbbaa"));
        assert_eq!(patch("0:\naabb", hex("ddccbbaa")).unwrap(), hex("aabbbbaa"));
        assert_eq!(
            patch("5:aabb", hex("ddccbbaa")).unwrap(),
            hex("ddccbbaa00aabb")
        );
    }
}
