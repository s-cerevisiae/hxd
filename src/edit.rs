use std::{
    env,
    fs::File,
    io::{self, BufReader, BufWriter},
    path::Path,
    process::Command,
};

use tempfile::NamedTempFile;

use crate::{
    cli::{DumpArgs, EditArgs},
    dump::dump_impl,
    load::load_impl,
};

pub fn edit(options: EditArgs) -> io::Result<()> {
    let EditArgs {
        columns,
        groupsize,
        input,
    } = options;
    let input_path = Path::new(&input);
    let Some((file_name, dir)) = input_path
        .is_file()
        .then(|| input_path.file_name().zip(input_path.parent()))
        .flatten()
    else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "file to edit does not exist or is a directory",
        ));
    };
    let mut dump_tmp = NamedTempFile::with_prefix_in(file_name, dir)?;
    dump_impl(
        DumpArgs {
            columns,
            groupsize,
            input: Some(input.clone()),
        },
        BufReader::new(File::open(input_path)?),
        BufWriter::new(&mut dump_tmp),
    )?;
    let file_to_edit = dump_tmp.into_temp_path();

    let editor = env::var_os("EDITOR")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "$EDITOR is not set"))?;
    Command::new(editor).arg(&file_to_edit).spawn()?.wait()?;

    let mut target = NamedTempFile::with_prefix_in(file_name, dir)?;
    load_impl(
        BufReader::new(File::open(&file_to_edit)?),
        BufWriter::new(&mut target),
    )?;
    target.persist(input_path)?;
    Ok(())
}
