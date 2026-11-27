use clap::Parser;

#[derive(Parser)]
#[command(disable_version_flag = true)]
pub struct Args {
    #[arg(short = 'v', long = "version")]
    version: bool,
}

pub fn run() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.version {
        println!(
            "tldr {} (Client Specification 2.3)",
            env!("CARGO_PKG_VERSION")
        );
    }
    Ok(())
}
