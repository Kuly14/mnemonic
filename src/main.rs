use anyhow::Result;
use clap::Parser;
use mnemonic::Cli;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<()> {
    let args = Cli::parse();
    mnemonic::check_extension(&args)?;
    let content = mnemonic::read_file(&args.path)?;
    let bytecode = mnemonic::get_bytecode(content)?;

    if args.print == true {
        println!("{}", bytecode);
    } else if !args.destination.is_some() {
        let file_name = args.path.file_name().unwrap().to_str().unwrap();
        let new_file_name = file_name.replace(".mn", ".txt");
        let mut file = File::create(&new_file_name)?;
        file.write_all(bytecode.as_bytes())?;

        println!("Done...");
        println!("Bytecode is saved at {}", new_file_name);
    } else {
        let file_path = args.destination.unwrap();
        let mut file = File::create(&file_path)?;
        file.write_all(bytecode.as_bytes())?;

        println!("Done...");
        println!("Bytecode is saved at {:#?}", file_path);
    }
    Ok(())
}
