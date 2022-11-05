use anyhow::{Context, Result};
use clap::Parser;
use dict::Opcodes;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    fs,
    fs::metadata,
    path::PathBuf,
};

mod dict;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to your .mn file
    pub path: PathBuf,
    /// Use -p if you want to print the bytecode to the terminal
    #[clap(long, short, action)]
    pub print: bool,
    /// Use -d to specify where do you want to save the bytecode. If the file exists it will error
    #[clap(long, short)]
    pub destination: Option<PathBuf>,
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

    let file_name = args.path.file_name().unwrap().to_str().unwrap().to_string();

    let extension = match args.path.extension() {
        Some(ext) => ext,
        None => {
            return Err(anyhow::Error::new(InputError::WrongFile(file_name)));
        }
    };

    if extension != "mn" {
        return Err(anyhow::Error::new(InputError::WrongFile(file_name)));
    }

    Ok(())
}

pub fn read_file(path: &PathBuf) -> Result<String> {
    let contents = fs::read_to_string(path).context("Couldn't read file")?;
    Ok(contents)
}

pub fn get_bytecode(content: String) -> Result<String> {
    let mut lines = content.lines();

    let opcodes_struct = Opcodes::new();

    let mut bytecode_final = String::new();

    'outer: while let Some(line) = lines.next() {
        let words = line.split_whitespace().map(|word| word.to_lowercase());

        for word in words {
            if word.contains("//") {
                continue 'outer;
            }

            let fixed_byte: String;

            let bytecode = match opcodes_struct.opcodes.get(&word[..]) {
                Some(opcode) => opcode,
                None => {
                    if word.contains("0x") {
                        fixed_byte = dict::add_push(&word);
                        if fixed_byte.trim() == String::from("404") {
                            return Err(anyhow::Error::new(InputError::WrongHexLength(
                                word.to_string(),
                            )));
                        }
                    } else {
                        return Err(anyhow::Error::new(InputError::UnknownOpcode(word)));
                    }

                    &fixed_byte[..]
                }
            };

            bytecode_final.push_str(bytecode);
        }
    }

    Ok(bytecode_final)
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
            print: true,
            destination: None,
        };
        check_extension(&cli).unwrap()
    }

    #[test]
    #[should_panic(expected = "File has to be a valid `.mn` file, You provided `box.sol`")]
    pub fn test_different_extension() {
        let cli = Cli {
            path: PathBuf::from("./box.sol"),
            print: true,
            destination: None,
        };
        check_extension(&cli).unwrap()
    }
}
