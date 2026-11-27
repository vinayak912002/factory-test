use clap::builder::PossibleValuesParser;
use clap::{CommandFactory, Parser};

use crate::error::TldrError;
use crate::platform::VALID_PLATFORMS;

#[derive(Parser)]
#[command(disable_version_flag = true)]
pub struct Args {
    #[arg(short = 'v', long = "version")]
    version: bool,

    #[arg(long = "search")]
    search: Option<String>,

    #[arg(short = 'u', long = "update")]
    update: bool,

    #[arg(short = 'k', long = "clear-cache")]
    clear_cache: bool,

    #[arg(short = 'p', long = "platform", value_parser = PossibleValuesParser::new(VALID_PLATFORMS))]
    platform: Option<String>,

    #[arg(short = 'l', long = "list")]
    list: bool,

    #[arg(short = 's', long = "source")]
    source: Option<String>,

    #[arg(short = 'c', long = "color")]
    color: bool,

    #[arg(short = 'r', long = "render")]
    render: Option<String>,

    #[arg(short = 'L', long = "language")]
    language: Option<String>,

    #[arg(short = 'm', long = "markdown")]
    markdown: bool,

    #[arg(long = "short-options")]
    short_options: bool,

    #[arg(long = "long-options")]
    long_options: bool,

    #[arg(long = "print-completion")]
    print_completion: bool,

    command: Vec<String>,
}

pub fn run() -> anyhow::Result<()> {
    if std::env::args_os().len() == 1 {
        eprintln!("{}", Args::command().render_help());
        return Err(TldrError::NoArguments.into());
    }

    let args = Args::parse();
    if args.version {
        println!(
            "tldr {} (Client Specification 2.3)",
            env!("CARGO_PKG_VERSION")
        );
    }
    if args.print_completion {
        println!("--print-completion is not supported");
    }
    Ok(())
}
