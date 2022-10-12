# Compiler for EVM OpCodes

## Motivation

Solidity compiler isn't very efficient. Often for elementary operations the Solidity compiler will add a ton of unnecessary opcodes. The compiler does this because it adds a lot of checks if the developer didn't screw up somehow, and if they did, the compiler would revert before doing any damage.

This is great when you start with solidity and just want to write something that works. But since gas on Ethereum can get very expensive, sometimes we need to optimize the contract as much as possible.

You can use inline assembly, but the solidity compiler still adds many checks and unnecessary opcodes that just cost more gas.

You can also use language like Huff, which is excellent. It gives you access to the stack, and you can do pretty much any optimization you want. But to write a whole complex contract in Huff is torture since you have to go opcode by opcode.

The best option, in my opinion, is to write the contract in solidity, leave out the functions that need more optimization, and just add the bytecode with the [verbatim](https://docs.soliditylang.org/en/v0.8.17/yul.html#verbatim) later.

This is great, but we need a way to generate the bytecode since writing pure bytes is also torture.

That's why I wrote this helpful tool.

## How to use

1. Create a file with the extension _.mn_
2. Write your desired function with pure opcodes.

```rust
push1 0x04
calldataload
push1 0x00
mstore
```

3. Run `mnemonic` and input path to the file as an argument like this:

```bash
cargo run -- box.mn
```

This will print the desired output to the terminal.

4. Write your solidity contract and compile it down to yul like this:

```bash
solc nameOfTheFile.sol --ir > nameOfTheDestinationFile.yul
```

5. Use the [verbatim](https://docs.soliditylang.org/en/v0.8.17/yul.html#verbatim) to input the bytecode into your desired function.

Here is an example: https://github.com/Kuly14/mnemonic/blob/main/box.yul#L311

6. Compile the .yul file to bytecode. You can do so like this:

```bash
solc --strict-assembly nameofTheFile.yul --bin
```

7. Deploy the bytecode

I created an example repo that shows how to do this: https://github.com/Kuly14/deployBytecode 
8. Write a test for every little thing.

Since we are messing with the stack, it is essential to test everything.

Have Fun!!!

## Disclaimer

Don't use in production. Unsafe software.
