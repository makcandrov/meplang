pub type OpCode = u8;

// pub const PUSH_GAS: usize = 3;

// 0x30 range - closure state.
pub const ADDRESS: OpCode = 0x30;
pub const BALANCE: OpCode = 0x31;
pub const ORIGIN: OpCode = 0x32;
pub const CALLER: OpCode = 0x33;
pub const CALLVALUE: OpCode = 0x34;
pub const CALLDATALOAD: OpCode = 0x35;
pub const CALLDATASIZE: OpCode = 0x36;
pub const CALLDATACOPY: OpCode = 0x37;
pub const CODESIZE: OpCode = 0x38;
pub const CODECOPY: OpCode = 0x39;
pub const GASPRICE: OpCode = 0x3a;
pub const EXTCODESIZE: OpCode = 0x3b;
pub const EXTCODECOPY: OpCode = 0x3c;
pub const RETURNDATASIZE: OpCode = 0x3d;
pub const RETURNDATACOPY: OpCode = 0x3e;
pub const EXTCODEHASH: OpCode = 0x3f;

// 0x40 range - block operations.
pub const BLOCKHASH: OpCode = 0x40;
pub const COINBASE: OpCode = 0x41;
pub const TIMESTAMP: OpCode = 0x42;
pub const NUMBER: OpCode = 0x43;
pub const DIFFICULTY: OpCode = 0x44;
pub const RANDOM: OpCode = 0x44;
pub const PREVRANDAO: OpCode = 0x44;
pub const GASLIMIT: OpCode = 0x45;
pub const CHAINID: OpCode = 0x46;
pub const SELFBALANCE: OpCode = 0x47;
pub const BASEFEE: OpCode = 0x48;

// 0x50 range - 'storage' and execution.
pub const POP: OpCode = 0x50;
pub const MLOAD: OpCode = 0x51;
pub const MSTORE: OpCode = 0x52;
pub const MSTORE8: OpCode = 0x53;
pub const SLOAD: OpCode = 0x54;
pub const SSTORE: OpCode = 0x55;
pub const JUMP: OpCode = 0x56;
pub const JUMPI: OpCode = 0x57;
pub const PC: OpCode = 0x58;
pub const MSIZE: OpCode = 0x59;
pub const GAS: OpCode = 0x5a;
pub const JUMPDEST: OpCode = 0x5b;
// pub const PUSH0: OpCode = 0x5f;

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
        "address" => ADDRESS,
        "balance" => BALANCE,
        "origin" => ORIGIN,
        "caller" => CALLER,
        "callvalue" => CALLVALUE,
        "calldataload" => CALLDATALOAD,
        "calldatasize" => CALLDATASIZE,
        "calldatacopy" => CALLDATACOPY,
        "codesize" => CODESIZE,
        "codecopy" => CODECOPY,
        "gasprice" => GASPRICE,
        "extcodesize" => EXTCODESIZE,
        "extcodecopy" => EXTCODECOPY,
        "returndatasize" => RETURNDATASIZE,
        "returndatacopy" => RETURNDATACOPY,
        "extcodehash" => EXTCODEHASH,

        // 0x40 range - block operations.
        "blockhash" => BLOCKHASH,
        "coinbase" => COINBASE,
        "timestamp" => TIMESTAMP,
        "number" => NUMBER,
        "difficulty" => DIFFICULTY,
        "random" => RANDOM,
        "prevrandao" => PREVRANDAO,
        "gaslimit" => GASLIMIT,
        "chainid" => CHAINID,
        "selfbalance" => SELFBALANCE,
        "basefee" => BASEFEE,

        // 0x50 range - 'storage' and execution.
        "pop" => POP,
        "mload" => MLOAD,
        "mstore" => MSTORE,
        "mstore8" => MSTORE8,
        "sload" => SLOAD,
        "sstore" => SSTORE,
        "jump" => JUMP,
        "jumpi" => JUMPI,
        "pc" => PC,
        "msize" => MSIZE,
        "gas" => GAS,
        "jumpdest" => JUMPDEST,
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
