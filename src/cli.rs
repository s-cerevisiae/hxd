use std::ffi::OsString;

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
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "dump")]
/// dumps a file into hexdump format
pub struct DumpArgs {
    #[argh(option, short = 'c', default = "16")]
    /// number of octets shown on each line, defaults to 16
    pub columns: usize,
    #[argh(option, short = 'g', default = "4")]
    /// octets per group (separated by a single space), defaults to 4
    pub groupsize: usize,
    #[argh(positional)]
    /// input file, defaults to stdin
    pub input: Option<OsString>,
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "load")]
/// parse the format from `dump` and output the original binary, ignoring offsets and comments
pub struct LoadArgs {
    #[argh(positional)]
    /// input file, defaults to stdin
    pub input: Option<OsString>,
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "edit")]
/// open an editor to edit the binary, original file will be replaced if no error has occurred
pub struct EditArgs {
    #[argh(option, short = 'c', default = "16")]
    /// number of octets shown on each line, defaults to 16
    pub columns: usize,
    #[argh(option, short = 'g', default = "4")]
    /// octets per group (separated by a single space), defaults to 4
    pub groupsize: usize,
    #[argh(positional)]
    /// the file to edit
    pub input: OsString,
}
