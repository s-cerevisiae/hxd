use std::{
    fs::File,
    io::{self, BufRead, Seek, Write},
};

use eyre::{eyre, OptionExt, WrapErr};

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
    let mut patch_set: PatchSet = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let DumpLine { offset, data, .. } = recognize_line(&line);
        let mut parsed_data = Vec::new();
        for_parsed_data(data, |b| {
            parsed_data.push(b);
            Ok(())
        })?;
        if offset.is_empty() {
            patch_set
                .last_mut()
                .ok_or_eyre("patch must start with a valid offset")?
                .data
                .extend(parsed_data);
        } else {
            let offset = u64::from_str_radix(offset, 16)
                .wrap_err_with(|| eyre!("invalid offset `{offset}`"))?;
            patch_set.push(Patch {
                offset,
                data: parsed_data,
            });
        }
    }
    Ok(patch_set)
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
