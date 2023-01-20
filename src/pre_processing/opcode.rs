pub type OpCode = u8;

// pub const PUSH_GAS: usize = 3;

pub fn str_to_op(name: &str) -> Option<OpCode> {
    Some(match name.to_lowercase().as_str() {
        // 0x0 range - arithmetic ops.
        "stop" => 0x00,
        "add" => 0x01,
        "mul" => 0x02,
        "sub" => 0x03,
        "div" => 0x04,
        "sdiv" => 0x05,
        "mod" => 0x06,
        "smod" => 0x07,
        "addmod" => 0x08,
        "mulmod" => 0x09,
        "exp" => 0x0a,
        "signextend" => 0x0b,

        // 0x10 range - comparison ops.
        "lt" => 0x10,
        "gt" => 0x11,
        "slt" => 0x12,
        "sgt" => 0x13,
        "eq" => 0x14,
        "iszero" => 0x15,
        "and" => 0x16,
        "or" => 0x17,
        "xor" => 0x18,
        "not" => 0x19,
        "byte" => 0x1a,
        "shl" => 0x1b,
        "shr" => 0x1c,
        "sar" => 0x1d,

        // 0x20 range - crypto.
        "keccak256" => 0x20,

        // 0x30 range - closure state.
        "address" => 0x30,
        "balance" => 0x31,
        "origin" => 0x32,
        "caller" => 0x33,
        "callvalue" => 0x34,
        "calldataload" => 0x35,
        "calldatasize" => 0x36,
        "calldatacopy" => 0x37,
        "codesize" => 0x38,
        "codecopy" => 0x39,
        "gasprice" => 0x3a,
        "extcodesize" => 0x3b,
        "extcodecopy" => 0x3c,
        "returndatasize" => 0x3d,
        "returndatacopy" => 0x3e,
        "extcodehash" => 0x3f,

        // 0x40 range - block operations.
        "blockhash" => 0x40,
        "coinbase" => 0x41,
        "timestamp" => 0x42,
        "number" => 0x43,
        "difficulty" => 0x44,
        "random" => 0x44,
        "prevrandao" => 0x44,
        "gaslimit" => 0x45,
        "chainid" => 0x46,
        "selfbalance" => 0x47,
        "basefee" => 0x48,

        // 0x50 range - 'storage' and execution.
        "pop" => 0x50,
        "mload" => 0x51,
        "mstore" => 0x52,
        "mstore8" => 0x53,
        "sload" => 0x54,
        "sstore" => 0x55,
        "jump" => 0x56,
        "jumpi" => 0x57,
        "pc" => 0x58,
        "msize" => 0x59,
        "gas" => 0x5a,
        "jumpdest" => 0x5b,
        // "push0" => 0x57, // invalid

        // 0x60 range - pushes. // all pushes instructions are invalid
        
        // 0x80 range - dups.
        "dup1" => 0x80,
        "dup2" => 0x81,
        "dup3" => 0x82,
        "dup4" => 0x83,
        "dup5" => 0x84,
        "dup6" => 0x85,
        "dup7" => 0x86,
        "dup8" => 0x87,
        "dup9" => 0x88,
        "dup10" => 0x89,
        "dup11" => 0x8a,
        "dup12" => 0x8b,
        "dup13" => 0x8c,
        "dup14" => 0x8d,
        "dup15" => 0x8e,
        "dup16" => 0x8f,

        // 0x90 range - swaps.
        "swap1" => 0x90,
        "swap2" => 0x91,
        "swap3" => 0x92,
        "swap4" => 0x93,
        "swap5" => 0x94,
        "swap6" => 0x95,
        "swap7" => 0x96,
        "swap8" => 0x97,
        "swap9" => 0x98,
        "swap10" => 0x99,
        "swap11" => 0x9a,
        "swap12" => 0x9b,
        "swap13" => 0x9c,
        "swap14" => 0x9d,
        "swap15" => 0x9e,
        "swap16" => 0x9f,

        // 0xa0 range - logging ops.
        "log0" => 0xa0,
        "log1" => 0xa1,
        "log2" => 0xa2,
        "log3" => 0xa3,
        "log4" => 0xa4,

        // 0xb0 range.
        "tload" => 0xb3,
        "tstore" => 0xb4,

        // 0xf0 range - closures.
        "create" => 0xf0,
        "call" => 0xf1,
        "callcode" => 0xf2,
        "return" => 0xf3,
        "delegatecall" => 0xf4,
        "create2" => 0xf5,

        "staticcall" => 0xfa,
        "revert" => 0xfd,
    
        "selfdestruct" => 0xff,

        _ => return None,
    })
}
