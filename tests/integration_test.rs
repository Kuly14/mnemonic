use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

#[test]
pub fn test_workflow() {
    let mut file = File::create("test.mn").unwrap();
    file.write_all(
        b"
        0x01 // storage slot
        0x00 mstore // store the storage slot
        0x4675C7e5BaAFBFFbca748158bEcBA61ef3b0a263 // random address
        0x20 mstore // store the address at 0x20
        0x40 0x00
        sha3 // hash them together to get storage slot for the mapping
        0xb5d85CBf7cB3EE0D56b3bB207D5Fc4B82f43F511 //another random address
        sstore
        ",
    )
    .unwrap();

    let bytecode = Command::new("cargo")
        .args(["r", "--", "test.mn"])
        .output()
        .unwrap();

    let manual_bytecode = String::from("6001600052734675C7e5BaAFBFFbca748158bEcBA61ef3b0a263602052604060002073b5d85CBf7cB3EE0D56b3bB207D5Fc4B82f43F51155").to_lowercase();

    assert_eq!(
        String::from_utf8(bytecode.stdout).unwrap().trim(),
        manual_bytecode
    );
}
