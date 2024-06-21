use std::process::ExitCode;

use hxd::{
    cli::{CliArgs, SubCmd},
    dump::dump,
    edit::edit,
    load::load,
};

fn main() -> ExitCode {
    let cli_args: CliArgs = argh::from_env();

    let result = match cli_args.subcmd {
        SubCmd::Dump(d) => dump(d),
        SubCmd::Load(l) => load(l),
        SubCmd::Edit(e) => edit(e),
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
