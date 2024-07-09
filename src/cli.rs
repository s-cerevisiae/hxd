use std::{num::NonZeroUsize, path::PathBuf};

/// A non-interactive hexdump processor.
#[derive(argh::FromArgs)]
pub struct CliArgs {
    #[argh(subcommand)]
    pub subcmd: SubCmd,
}
#[derive(argh::FromArgs)]
#[argh(subcommand)]
pub enum SubCmd {
    Dump(DumpArgs),
    Load(LoadArgs),
    Edit(EditArgs),
    Patch(PatchArgs),
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "dump")]
/// dumps a file into hexdump format
pub struct DumpArgs {
    #[argh(option, short = 'c', default = "NonZeroUsize::new(16).unwrap()")]
    /// number of octets shown on each line, defaults to 16
    pub columns: NonZeroUsize,
    #[argh(option, short = 'g', default = "4")]
    /// octets per group (separated by a single space). defaults to 4, use 0 to disable grouping
    pub groupsize: usize,
    #[argh(option, short = 'o', default = "0")]
    /// add given amount to all the offset in output, defaults to 0
    pub offset: u64,
    #[argh(positional)]
    /// input file, defaults to stdin
    pub input: Option<PathBuf>,
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "load")]
/// parse the format from `dump` and output the original binary, ignoring offsets and comments
pub struct LoadArgs {
    #[argh(positional)]
    /// input file, defaults to stdin
    pub input: Option<PathBuf>,
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "edit")]
/// open an editor to edit the binary, original file will be replaced if no error has occurred
pub struct EditArgs {
    #[argh(option, short = 'c', default = "NonZeroUsize::new(16).unwrap()")]
    /// number of octets shown on each line, defaults to 16
    pub columns: NonZeroUsize,
    #[argh(option, short = 'g', default = "4")]
    /// octets per group (separated by a single space). defaults to 4, use 0 to disable grouping
    pub groupsize: usize,
    #[argh(positional)]
    /// the file to edit
    pub input: PathBuf,
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "patch")]
/// parse the input as the dump format into a patch, and overwrite portions in the target file
/// accordingly
pub struct PatchArgs {
    #[argh(option, short = 'o', default = "0")]
    /// add given amount to all the offset in patch before applying, can be negative. defaults to 0
    pub offset: i64,
    #[argh(positional)]
    // the file to apply the patch
    pub target: PathBuf,
}
