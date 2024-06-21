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
        let Some((groups, _)) = line
            .split_once(": ")
            .and_then(|(_, line)| line.split_once("  "))
        else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid line format:\n{line}"),
            ));
        };
        for mut group in groups.split(' ') {
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
