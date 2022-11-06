use std::collections::HashMap;

pub struct Opcodes<'a> {
    pub opcodes: HashMap<&'a str, &'a str>,
}

impl Opcodes<'_> {
    pub fn new() -> Self {
        let mut opcodes: HashMap<&str, &str> = HashMap::new();
        opcodes.insert("stop", "00");
        opcodes.insert("add", "01");
        opcodes.insert("mul", "02");
        opcodes.insert("sub", "03");
        opcodes.insert("div", "04");
        opcodes.insert("sdiv", "05");
        opcodes.insert("mod", "06");
        opcodes.insert("smod", "07");
        opcodes.insert("addmod", "08");
        opcodes.insert("mulmod", "09");
        opcodes.insert("exp", "0a");
        opcodes.insert("signextend", "0b");
        opcodes.insert("lt", "10");
        opcodes.insert("gt", "11");
        opcodes.insert("slt", "12");
        opcodes.insert("sgt", "13");
        opcodes.insert("eq", "14");
        opcodes.insert("iszero", "15");
        opcodes.insert("and", "16");
        opcodes.insert("or", "17");
        opcodes.insert("xor", "18");
        opcodes.insert("not", "19");
        opcodes.insert("byte", "1a");
        opcodes.insert("shl", "1b");
        opcodes.insert("shr", "1c");
        opcodes.insert("sar", "1d");
        opcodes.insert("sha3", "20");
        opcodes.insert("address", "30");
        opcodes.insert("balance", "31");
        opcodes.insert("origin", "32");
        opcodes.insert("caller", "33");
        opcodes.insert("callvalue", "34");
        opcodes.insert("calldataload", "35");
        opcodes.insert("calldatasize", "36");
        opcodes.insert("calldatacopy", "37");
        opcodes.insert("codesize", "38");
        opcodes.insert("codecopy", "39");
        opcodes.insert("gasprice", "3a");
        opcodes.insert("extcodesize", "3b");
        opcodes.insert("extcodecopy", "3c");
        opcodes.insert("returndatasize", "3d");
        opcodes.insert("returndatacopy", "3e");
        opcodes.insert("extcodehash", "3f");
        opcodes.insert("blockhash", "40");
        opcodes.insert("coinbase", "41");
        opcodes.insert("timestamp", "42");
        opcodes.insert("number", "43");
        opcodes.insert("difficulty", "44");
        opcodes.insert("gaslimit", "45");
        opcodes.insert("chainid", "46");
        opcodes.insert("selfbalance", "47");
        opcodes.insert("basefee", "48");
        opcodes.insert("pop", "50");
        opcodes.insert("mload", "51");
        opcodes.insert("mstore", "52");
        opcodes.insert("mstore8", "53");
        opcodes.insert("sload", "54");
        opcodes.insert("sstore", "55");
        opcodes.insert("jump", "56");
        opcodes.insert("jumpi", "57");
        opcodes.insert("pc", "58");
        opcodes.insert("msize", "59");
        opcodes.insert("gas", "5a");
        opcodes.insert("jumpdest", "5b");
        opcodes.insert("dup1", "80");
        opcodes.insert("dup2", "81");
        opcodes.insert("dup3", "82");
        opcodes.insert("dup4", "83");
        opcodes.insert("dup5", "84");
        opcodes.insert("dup6", "85");
        opcodes.insert("dup7", "86");
        opcodes.insert("dup8", "87");
        opcodes.insert("dup9", "88");
        opcodes.insert("dup10", "89");
        opcodes.insert("dup11", "8a");
        opcodes.insert("dup12", "8b");
        opcodes.insert("dup13", "8c");
        opcodes.insert("dup14", "8d");
        opcodes.insert("dup15", "8e");
        opcodes.insert("dup16", "8f");
        opcodes.insert("swap1", "90");
        opcodes.insert("swap2", "91");
        opcodes.insert("swap3", "92");
        opcodes.insert("swap4", "93");
        opcodes.insert("swap5", "94");
        opcodes.insert("swap6", "95");
        opcodes.insert("swap7", "96");
        opcodes.insert("swap8", "97");
        opcodes.insert("swap9", "98");
        opcodes.insert("swap10", "99");
        opcodes.insert("swap11", "9a");
        opcodes.insert("swap12", "9b");
        opcodes.insert("swap13", "9c");
        opcodes.insert("swap14", "9d");
        opcodes.insert("swap15", "9e");
        opcodes.insert("swap16", "9f");
        opcodes.insert("log0", "a0");
        opcodes.insert("log1", "a1");
        opcodes.insert("log2", "a2");
        opcodes.insert("log3", "a3");
        opcodes.insert("log4", "a4");
        opcodes.insert("create", "f0");
        opcodes.insert("call", "f1");
        opcodes.insert("callcode", "f2");
        opcodes.insert("return", "f3");
        opcodes.insert("delegatecall", "f4");
        opcodes.insert("create2", "f5");
        opcodes.insert("staticall", "fa");
        opcodes.insert("revert", "fd");
        opcodes.insert("invalid", "fe");
        opcodes.insert("selfdestruct", "ff");

        Self { opcodes }
    }
}

pub fn add_push(byte: &str) -> String {
    let fixed_byte = byte.replace("0x", "");
    let length = fixed_byte.len();

    if length == 2 {
        return format!("60{}", fixed_byte);
    }

    if length == 4 {
        return format!("61{}", fixed_byte);
    }

    if length == 6 {
        return format!("62{}", fixed_byte);
    }

    if length == 8 {
        return format!("63{}", fixed_byte);
    }

    if length == 10 {
        return format!("64{}", fixed_byte);
    }

    if length == 12 {
        return format!("65{}", fixed_byte);
    }

    if length == 14 {
        return format!("66{}", fixed_byte);
    }

    if length == 16 {
        return format!("67{}", fixed_byte);
    }

    if length == 18 {
        return format!("68{}", fixed_byte);
    }

    if length == 20 {
        return format!("69{}", fixed_byte);
    }

    if length == 22 {
        return format!("6a{}", fixed_byte);
    }

    if length == 24 {
        return format!("6b{}", fixed_byte);
    }

    if length == 26 {
        return format!("6c{}", fixed_byte);
    }

    if length == 28 {
        return format!("6d{}", fixed_byte);
    }

    if length == 30 {
        return format!("6e{}", fixed_byte);
    }

    if length == 32 {
        return format!("6f{}", fixed_byte);
    }

    if length == 34 {
        return format!("70{}", fixed_byte);
    }

    if length == 36 {
        return format!("71{}", fixed_byte);
    }
    if length == 38 {
        return format!("72{}", fixed_byte);
    }
    if length == 40 {
        return format!("73{}", fixed_byte);
    }
    if length == 42 {
        return format!("74{}", fixed_byte);
    }
    if length == 44 {
        return format!("75{}", fixed_byte);
    }

    if length == 46 {
        return format!("76{}", fixed_byte);
    }

    if length == 48 {
        return format!("77{}", fixed_byte);
    }
    if length == 50 {
        return format!("78{}", fixed_byte);
    }
    if length == 52 {
        return format!("79{}", fixed_byte);
    }

    if length == 54 {
        return format!("7a{}", fixed_byte);
    }
    if length == 56 {
        return format!("7b{}", fixed_byte);
    }

    if length == 58 {
        return format!("7c{}", fixed_byte);
    }

    if length == 60 {
        return format!("7d{}", fixed_byte);
    }
    if length == 62 {
        return format!("7e{}", fixed_byte);
    }
    if length == 64 {
        return format!("7f{}", fixed_byte);
    }

    String::from("404")
}
