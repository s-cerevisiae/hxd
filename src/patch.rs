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

pub(crate) fn patch_impl<R: BufRead, W: Write + Seek>(reader: R, mut writer: W) -> eyre::Result<()> {
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
