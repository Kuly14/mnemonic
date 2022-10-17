pub fn check_code(line: &str) -> &str {
    if line == "stop" {
        return "00";
    }

    if line == "add" {
        return "01";
    }

    if line == "mul" {
        return "02";
    }

    if line == "sub" {
        return "03";
    }

    if line == "div" {
        return "04";
    }

    if line == "sdiv" {
        return "05";
    }

    if line == "mod" {
        return "06";
    }

    if line == "smod" {
        return "07";
    }

    if line == "addmod" {
        return "08";
    }

    if line == "mulmod" {
        return "09";
    }

    if line == "exp" {
        return "0a";
    }

    if line == "signextend" {
        return "0b";
    }

    if line == "lt" {
        return "10";
    }

    if line == "gt" {
        return "11";
    }

    if line == "slt" {
        return "12";
    }

    if line == "sgt" {
        return "13";
    }

    if line == "eq" {
        return "14";
    }

    if line == "iszero" {
        return "15";
    }

    if line == "and" {
        return "16";
    }

    if line == "or" {
        return "17";
    }

    if line == "xor" {
        return "18";
    }

    if line == "not" {
        return "19";
    }

    if line == "byte" {
        return "1a";
    }

    if line == "shl" {
        return "1b";
    }

    if line == "shr" {
        return "1c";
    }

    if line == "sar" {
        return "1d";
    }

    if line == "sha3" {
        return "20";
    }

    if line == "address" {
        return "30";
    }

    if line == "balance" {
        return "31";
    }

    if line == "origin" {
        return "32";
    }

    if line == "caller" {
        return "33";
    }

    if line == "callvalue" {
        return "34";
    }

    if line == "calldataload" {
        return "35";
    }

    if line == "calldatasize" {
        return "36";
    }

    if line == "calldatacopy" {
        return "37";
    }

    if line == "codesize" {
        return "38";
    }

    if line == "codecopy" {
        return "39";
    }

    if line == "gasprice" {
        return "3a";
    }

    if line == "extcodesize" {
        return "3b";
    }

    if line == "extcodecopy" {
        return "3c";
    }

    if line == "returndatasize" {
        return "3d";
    }

    if line == "returndatacopy" {
        return "3e";
    }

    if line == "extcodehash" {
        return "3f";
    }

    if line == "blockhash" {
        return "40";
    }

    if line == "coinbase" {
        return "41";
    }

    if line == "timestamp" {
        return "42";
    }

    if line == "number" {
        return "43";
    }

    if line == "difficulty" {
        return "44";
    }

    if line == "gaslimit" {
        return "45";
    }

    if line == "chainid" {
        return "46";
    }

    if line == "selfbalance" {
        return "47";
    }

    if line == "basefee" {
        return "48";
    }

    if line == "pop" {
        return "50";
    }

    if line == "mload" {
        return "51";
    }

    if line == "mstore" {
        return "52";
    }

    if line == "mstore8" {
        return "53";
    }

    if line == "sload" {
        return "54";
    }

    if line == "sstore" {
        return "55";
    }

    if line == "jump" {
        return "56";
    }

    if line == "jumpi" {
        return "57";
    }

    if line == "pc" {
        return "58";
    }

    if line == "msize" {
        return "59";
    }

    if line == "gas" {
        return "5a";
    }

    if line == "jumpdest" {
        return "5b";
    }

    if line == "dup1" {
        return "80";
    }

    if line == "dup2" {
        return "81";
    }

    if line == "dup3" {
        return "82";
    }

    if line == "dup4" {
        return "83";
    }

    if line == "dup5" {
        return "84";
    }

    if line == "dup6" {
        return "85";
    }

    if line == "dup7" {
        return "86";
    }

    if line == "dup8" {
        return "87";
    }

    if line == "dup9" {
        return "88";
    }

    if line == "dup10" {
        return "89";
    }

    if line == "dup11" {
        return "8a";
    }

    if line == "dup12" {
        return "8b";
    }

    if line == "dup13" {
        return "8c";
    }

    if line == "dup14" {
        return "8d";
    }

    if line == "dup15" {
        return "8e";
    }

    if line == "dup16" {
        return "8f";
    }

    if line == "swap1" {
        return "90";
    }

    if line == "swap2" {
        return "91";
    }

    if line == "swap3" {
        return "92";
    }

    if line == "swap4" {
        return "93";
    }

    if line == "swap5" {
        return "94";
    }

    if line == "swap6" {
        return "95";
    }

    if line == "swap7" {
        return "96";
    }

    if line == "swap8" {
        return "97";
    }

    if line == "swap9" {
        return "98";
    }

    if line == "swap10" {
        return "99";
    }

    if line == "swap11" {
        return "9a";
    }

    if line == "swap12" {
        return "9b";
    }

    if line == "swap13" {
        return "9c";
    }

    if line == "swap14" {
        return "9d";
    }

    if line == "swap15" {
        return "9e";
    }

    if line == "swap16" {
        return "9f";
    }

    if line == "log0" {
        return "a1";
    }

    if line == "log1" {
        return "a2";
    }

    if line == "log2" {
        return "a3";
    }

    if line == "log3" {
        return "a4";
    }

    if line == "create" {
        return "f0";
    }

    if line == "call" {
        return "f1";
    }

    if line == "callcode" {
        return "f2";
    }

    if line == "return" {
        return "f3";
    }

    if line == "delegatecall" {
        return "f4";
    }

    if line == "create2" {
        return "f5";
    }

    if line == "staticall" {
        return "fa";
    }

    if line == "revert" {
        return "fd";
    }

    if line == "invalid" {
        return "fe";
    }

    if line == "selfdestruct" {
        return "ff";
    }

    line
}

pub fn add_push(byte: &String) -> String {
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

    String::from("Wrong length")
}
