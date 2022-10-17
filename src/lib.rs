use anyhow::{Context, Result};
use clap::Parser;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    fs,
    fs::metadata,
    path::PathBuf,
};

mod dict;

#[derive(Parser)]
pub struct Cli {
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum InputError {
    UnknownOpcode(String),
    WrongHexLength(String),
    WrongFile(String),
    Directory,
}

impl Display for InputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use InputError::*;
        match self {
            UnknownOpcode(opcode) => write!(f, "Unknown Opcode: {}", opcode),
            WrongHexLength(line) => write!(
                f,
                "This hex number: {}\n Has a wrong length, each byte has to be 2 hex characters long",
                line
            ),
            WrongFile(file) => write!(f, "File has to be a valid `.mn` file, You provided `{}`", file),
            Directory => write!(f, "Path leads to a directory"),
        }
    }
}

impl Error for InputError {}

pub fn check_extension(args: &Cli) -> Result<()> {
    match metadata(&args.path)?.is_dir() {
        true => {
            return Err(anyhow::Error::new(InputError::Directory));
        }
        false => (),
    };

    let extension = match args.path.extension() {
        Some(ext) => ext,
        None => {
            return Err(anyhow::Error::new(InputError::WrongFile(
                args.path.file_name().unwrap().to_str().unwrap().to_string(),
            )))
        }
    };

    if extension != "mn" {
        return Err(anyhow::Error::new(InputError::WrongFile(
            args.path.file_name().unwrap().to_str().unwrap().to_string(),
        )));
    }

    Ok(())
}

pub fn read_file(path: &PathBuf) -> Result<String> {
    let contents = fs::read_to_string(path).context("Couldn't read file")?;
    Ok(contents)
}

pub fn get_bytecode(content: String) -> Result<String> {
    let mut lines = content.lines();

    let mut bytes: Vec<String> = Vec::new();

    'outer: while let Some(line) = lines.next() {
        let words = line.split_whitespace().map(|word| word.to_lowercase());

        for word in words {
            if word.contains("//") {
                continue 'outer;
            }

            let bytecode = dict::check_code(&word);

            if !word.contains("0x") && bytecode.trim() == word.trim() {
                return Err(anyhow::Error::new(InputError::UnknownOpcode(word)));
            }

            bytes.push(bytecode.to_string());
        }
    }

    let mut bytecode = String::new();

    for byte in bytes {
        let mut fixed_byte = byte.to_string();
        if byte.contains("0x") {
            fixed_byte = dict::add_push(&byte);
            if fixed_byte.trim() == String::from("Wrong length") {
                return Err(anyhow::Error::new(InputError::WrongHexLength(
                    byte.to_string(),
                )));
            }
        }
        bytecode.push_str(&fixed_byte);
    }
    Ok(bytecode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    pub fn test_get_bytecode() {
        let content = String::from(
            "
            0x01 0x40\n 
             stop 0x01 \n 
        ",
        );
        let bytecode = get_bytecode(content).unwrap();
        assert_eq!(bytecode, String::from("60016040006001"));
    }

    #[test]
    pub fn test_get_bytecode_complex() {
        let content = String::from(
            "
            0x04 calldataload
            0x00 mstore
            0x20 0x00
            return
        ",
        );
        let bytecode = get_bytecode(content).unwrap();
        assert_eq!(bytecode, String::from("60043560005260206000f3"));
    }

    #[test]
    pub fn test_get_bytecode_more_complex() {
        // This bytecode is essentialy idnetical to:
        // mapping(address => address) map;
        // function store() public {
        //     map[0x4675C7e5BaAFBFFbca748158bEcBA61ef3b0a263] = 0xb5d85CBf7cB3EE0D56b3bB207D5Fc4B82f43F511;
        // }

        let content = String::from(
            "
        0x01 // storage slot
        0x00 mstore // store the storage slot
        0x4675C7e5BaAFBFFbca748158bEcBA61ef3b0a263 // random address
        0x20 mstore // store the address at 0x20
        0x40 0x00
        sha3 // hash them together to get storage slot for the mapping
        0xb5d85CBf7cB3EE0D56b3bB207D5Fc4B82f43F511 //another random address
        sstore
        ",
        );

        let bytecode = get_bytecode(content).unwrap();
        let manual_bytecode = String::from("6001600052734675C7e5BaAFBFFbca748158bEcBA61ef3b0a263602052604060002073b5d85CBf7cB3EE0D56b3bB207D5Fc4B82f43F51155").to_lowercase();

        assert_eq!(bytecode, manual_bytecode);
    }

    #[test]
    #[should_panic(
        expected = "This hex number: 0x000\n Has a wrong length, each byte has to be 2 hex characters long"
    )]
    pub fn test_get_bytecode_error() {
        let content = String::from("0x000 mstore");
        get_bytecode(content).unwrap();
    }

    #[test]
    #[should_panic(expected = "Unknown Opcode: unknown")]
    pub fn test_unknown_opcode() {
        let content = String::from("0x00 unknown");
        get_bytecode(content).unwrap();
    }

    #[test]
    #[should_panic(expected = "Path leads to a directory")]
    pub fn test_is_directory() {
        let cli = Cli {
            path: PathBuf::from("./src"),
        };
        check_extension(&cli).unwrap()
    }

    #[test]
    #[should_panic(expected = "File has to be a valid `.mn` file, You provided `box.sol`")]
    pub fn test_different_extension() {
        let cli = Cli {
            path: PathBuf::from("./box.sol"),
        };
        check_extension(&cli).unwrap()
    }
}
