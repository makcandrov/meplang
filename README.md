# Meplang - An EVM low-level language

Meplang is a low-level programming language that produces EVM bytecode. It is designed for developers who need full control over the control flow in their smart contracts. It is strongly inspired by the Rust syntax.

Meplang is a low-level language and is not meant for complex smart contract development. It is recommended to use more high-level languages like Solidity or Vyper for that.

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

```rust
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