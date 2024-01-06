# Meplang - An EVM low-level language

Meplang is a low-level programming language that produces EVM bytecode. It is designed for developers who need full control over the control flow in their smart contracts.

Meplang is a low-level language and is not meant for complex smart contract development. It is recommended to use more high-level languages like Solidity or Yul for that.

**Please note that the work on Meplang is still in progress, and users should always verify that the output bytecode is as expected before deployment.**

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install) on your machine.

2. Run the following to build the Meplang compiler from source:

```sh
cargo install --git https://github.com/meppent/meplang.git
```

To update from source, run the same command again.

## Hello World

Here is an example of a simple Meplang contract, that returns "Hello World!" as bytes:

```rust,ignore
contract HelloWorld {
    block main {
        // copy the bytes into memory
        push(hello_world.size) push(hello_world.pc) push(0x) codecopy

        // return them
        push(hello_world.size) push(0x) return 
    }

    block hello_world {
        // "Hello World!" as bytes
        0x48656c6c6f20576f726c6421
    }
}
```

To compile this contract saved as `hello_world.mep`, run the following command: 

```sh
meplang compile -contract HelloWorld -input hello_world.mep
```

Or the shortened version: 

```sh
meplang compile -c HelloWorld -i hello_world.mep
```

This will print the runtime bytecode in the terminal. To export the compilation artifacts (including the runtime bytecode), use the argument `-o` or `-output`:

```sh
meplang compile -c HelloWorld -i hello_world.mep -o hello_world.json
```

## Deployment bytecode

The compilation gives the runtime bytecode of the smart contract. To get the deployment contract, use an auxiliary contract, and compile it:

```rust,ignore
contract Constructor {
    block main {
        // copy the bytes into memory
        push(deployed.size) push(deployed.pc) push(0x) codecopy

        // return them
        push(deployed.size) push(0x) return 
    }

    block deployed {
        &Deployed.code
    }
}

// the contract that will be deployed
contract Deployed {
    block main {
        // ...
    }
}
```

Compile the contract `Constructor` to get the deployment bytecode of the contract `Deployed`.

## Basic syntax

- A **contract** is declared with the keyword `contract`. Many contracts can be defined in a single file. A contract can copy the runtime bytecode of another contract using `&Contract.code` inside a block.
- A **block** is declared inside a contract using the keyword `block`. A block can be defined **abstract** (see later) using the keyword `abstract` before `block`. The first opcodes of the contract are from the necessary block named `main` (or a block surrounded by the attribute `#[main]`).
- A **constant** is declared inside a contract using the keyword `const`. Constants can only be used inside a function `push` inside a block.

```rust,ignore
contract BalanceGetter {
    const BALANCE_OF_SELECTOR = 0x70a08231;
    const WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;

    #[assume(msize = 0x00)]
    #[assume(returndatasize = 0x00)]
    block main {
        push(BALANCE_OF_SELECTOR) push(0x) mstore // mem[0x1c..0x20] = 0x70a08231
        #[assume(msize = 0x20)]
        address push(0x20) mstore // mem[0x20..0x40] = address(this)
        #[assume(msize = 0x40)]
    
        // mem[0x00..0x20] = WETH.call{value: 0, gas: gas()}(mem[0x1c..0x20])
        //                 = WETH.balanceOf(address(this))
        push(0x20) push(0x) push(0x24) push(0x1c) push(0x) push(WETH) gas call
        #[assume(returndatasize = 0x20)]

        // the contract's balance in WETH is stored at mem[0x00..0x20]

        push(0x20) push(0x) return
    }
}
```

- Inside a block, any opcode can be used *except PUSH1 to PUSH32 opcodes* (PUSH0 is allowed). Raw bytecode can also be used as is. A value can be pushed using the function `push`, which can take an hexadecimal literal, a constant, a *non-abstract* block PC or size as an argument. Only values inside a `push` function will be optimized by the compiler.

```rust,ignore
contract Contract {
    const MAGIC_NUMBER = 0xff;

    #[assume(msize = 0x00)]
    block main {
        push(MAGIC_NUMBER) push(0x) mstore
        #[assume(msize = 0x20)]

        push(0x20) // can be replaced by the opcode `msize` during the compilation
        0x6020     // won't be changed at the compilation

        push(end_block.size) // will be replaced by the actual size of the block `end_block`
        push(end_block.pc)   // will be replaced by the actual pc of the beginning of the block `end_block`
        jump
    }

    block end_block {
        jumpdest // do not forget to begin with jumpdest if we can jump on this block
        push(0x) push(0x) return
    }
}
```
- A *non-abstract* block can be copied at most once inside another block using the operator `*`. An *abstract* block can be copied as many times as desired inside other blocks using the operator `&`. Therefore, we cannot refer to the `pc` or to the `size` of an *abstract* block, because it may appear multiple times in the bytecode, and not be compiled the same every time.

```rust,ignore
contract Contract {
    #[assume(msize = 0x00)]
    block main {
        callvalue &shift_right_20_bytes // will most certainly be compiled `callvalue push1 0x20 shr`

        push(0x) push(0x) mstore
        #[assume(msize = 0x20)]

        callvalue &shift_right_20_bytes // will most certainly be compiled `callvalue msize shr` because we assumed msize = 0x20.
        *end_block
    }

    abstract block shift_right_20_bytes {
        push(0x20) shr
    }

    block end_block {
        // no jumpdest here because we do not jump on this block, we copy it
        push(0x) push(0x) return
    } 
}
```
- Many **attributes** exist to guide the compiler. They are declared over a contract, a block, or a line inside a block using the syntax `#[ATTRIBUTE]`. The current list of existing attributes is:
    - `assume` to tell the compiler that *from this point*, an opcode will push on the stack a defined value. The compiler can then replace some `push` opcodes with these assumptions. 
    - `clear_assume` to clear an assumption made previously.
    - `main` the main block can be marked with this attribute if it is not named `main`.
    - `last` to tell the compiler that the block must be placed at the end of the bytecode.
    - `keep` to tell the compiler that this block must be kept somewhere in the bytecode even if it is unused.

More examples of contracts can be found in the folder [examples](examples).

## Future features

- `assert` attribute to impose conditions on a block pc or a contract size.
- Heuristics to improve compilation optimizations.
- Inheritance of contracts.
