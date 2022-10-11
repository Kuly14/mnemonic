use anyhow::Result;
use clap::Parser;
use mnemonic::Cli;

fn main() -> Result<()> {
    let args = Cli::parse();
    mnemonic::check_extension(&args)?;
    let content = mnemonic::read_file(&args.path)?;
    let bytecode = mnemonic::get_bytecode(content)?;

    println!("{}", bytecode);
    Ok(())
}
