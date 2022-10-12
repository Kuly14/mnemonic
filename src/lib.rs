use anyhow::{Context, Result};
use clap::Parser;
use std::fs::metadata;
use std::process;
use std::{fs, path::PathBuf};

mod dict;

#[derive(Parser)]
pub struct Cli {
    pub path: PathBuf,
}

pub fn check_extension(args: &Cli) -> Result<()> {
    match metadata(&args.path)?.is_dir() {
        true => {
            println!("Path leads to a directory!");
            process::exit(1);
        }
        false => (),
    };

    let extension = match args.path.extension() {
        Some(ext) => ext,
        None => {
            println!("Path doesn't lead to a file with extension");
            process::exit(1);
        }
    };

    if extension != "mn" {
        println!("File has to have `.mn` extension");
        process::exit(1);
    }

    Ok(())
}

pub fn read_file(path: &PathBuf) -> Result<String> {
    let contents = fs::read_to_string(path).context("Couldn't read file")?;
    Ok(contents)
}

pub fn get_bytecode(content: String) -> Result<String> {
    let bytes = content
        .split_whitespace()
        .map(|line| line.to_lowercase())
        .map(|line| {
            return dict::check_code(line.trim()).to_string();
        })
        .collect::<Vec<String>>();

    let mut bytecode = String::new();

    bytes.iter().for_each(|byte| {
        let mut fixed_byte = byte.to_string();

        if byte.contains("0x") {
            fixed_byte = dict::add_push(byte);
            if fixed_byte.trim() == String::from("Wrong length") {
                println!("Wrong hex input");
                process::exit(1);
            }
        }
        bytecode.push_str(&fixed_byte);
    });

    Ok(bytecode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_bytecode() {
        let content = String::from(
            "
            0x01 push1 \n 
             stop 0x01 \n 
        ",
        );
        let bytecode = get_bytecode(content).unwrap();

        assert_eq!(bytecode, String::from("0x01600001"));
    }
}
