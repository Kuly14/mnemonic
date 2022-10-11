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

    if line == "push1" {
        return "60";
    }

    if line == "push2" {
        return "61";
    }

    if line == "push3" {
        return "62";
    }

    if line == "push4" {
        return "63";
    }

    if line == "push5" {
        return "64";
    }

    if line == "push6" {
        return "65";
    }

    if line == "push7" {
        return "66";
    }

    if line == "push8" {
        return "67";
    }

    if line == "push9" {
        return "68";
    }

    if line == "push10" {
        return "69";
    }

    if line == "push11" {
        return "6a";
    }

    if line == "push12" {
        return "6b";
    }

    if line == "push13" {
        return "6c";
    }

    if line == "push14" {
        return "6d";
    }

    if line == "push15" {
        return "6e";
    }

    if line == "push16" {
        return "6f";
    }

    if line == "push17" {
        return "70";
    }

    if line == "push18" {
        return "71";
    }

    if line == "push19" {
        return "72";
    }

    if line == "push20" {
        return "73";
    }

    if line == "push21" {
        return "74";
    }

    if line == "push22" {
        return "75";
    }

    if line == "push23" {
        return "76";
    }

    if line == "push24" {
        return "77";
    }

    if line == "push25" {
        return "78";
    }

    if line == "push26" {
        return "79";
    }

    if line == "push27" {
        return "7a";
    }

    if line == "push28" {
        return "7b";
    }

    if line == "push29" {
        return "7c";
    }

    if line == "push30" {
        return "7d";
    }

    if line == "push31" {
        return "7e";
    }

    if line == "push32" {
        return "7f";
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
