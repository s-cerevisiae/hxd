use std::{
    env,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
    process::Command,
};

use eyre::{bail, eyre, WrapErr};

use crate::{cli::EditArgs, dump::dump_impl, load::load_impl};

pub fn edit(options: EditArgs) -> eyre::Result<()> {
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
        bail!(
            "`{}` does not exist or is a directory",
            input_path.display()
        );
    };
    let mut dump_tmp = tempfile::Builder::new()
        .prefix(file_name)
        .suffix(".nxd")
        .tempfile_in(dir)
        .wrap_err_with(|| {
            eyre!(
                "failed to create temporary file for editing in `{}`",
                dir.display()
            )
        })?;
    let input_file = File::open(input_path)
        .wrap_err_with(|| eyre!("failed to open file `{}`", input_path.display()))?;
    let input_file_perm = input_file.metadata()?.permissions();
    dump_impl(
        BufReader::new(input_file),
        BufWriter::new(&mut dump_tmp),
        columns,
        groupsize,
        0,
    )?;
    let file_to_edit = dump_tmp.into_temp_path();

    let editor = env::var_os("EDITOR").ok_or_else(|| eyre!("$EDITOR is not set"))?;
    Command::new(editor)
        .arg(&file_to_edit)
        .spawn()
        .wrap_err("failed to start editor")?
        .wait()?;

    let mut target = tempfile::Builder::new()
        .prefix(file_name)
        .permissions(input_file_perm)
        .tempfile_in(dir)?;
    load_impl(
        BufReader::new(File::open(&file_to_edit)?),
        BufWriter::new(&mut target),
    )?;
    target
        .persist(input_path)
        .wrap_err_with(|| eyre!("failed to save file at `{}`", input_path.display()))?;
    Ok(())
}
