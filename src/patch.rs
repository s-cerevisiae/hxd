use std::{
    fs::File,
    io::{self, BufRead, Seek, Write},
};

use eyre::{eyre, WrapErr};

use crate::{
    cli::PatchArgs,
    parse::{for_parsed_data, recognize_line, DumpLine},
};

struct Patch {
    offset: u64,
    data: Vec<u8>,
}

type PatchSet = Vec<Patch>;

fn parse_patch<R: BufRead>(reader: R) -> eyre::Result<PatchSet> {
    reader
        .lines()
        .map(|line| -> eyre::Result<Patch> {
            let line = line?;
            let DumpLine { offset, data, .. } = recognize_line(&line);
            let offset = u64::from_str_radix(offset, 16)
                .wrap_err_with(|| eyre!("invalid offset `{offset}`"))?;
            let mut parsed_data = Vec::new();
            for_parsed_data(data, |b| {
                parsed_data.push(b);
                Ok(())
            })?;
            Ok(Patch {
                offset,
                data: parsed_data,
            })
        })
        .collect()
}

pub fn patch(args: PatchArgs) -> eyre::Result<()> {
    let mut target = File::options()
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

    let patch_set = parse_patch(input)?;

    for Patch { offset, data } in patch_set {
        target.seek(io::SeekFrom::Start(offset))?;
        target.write_all(&data)?;
    }

    Ok(())
}
