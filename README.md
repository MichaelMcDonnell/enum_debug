# Rust Debug Enum Example

The CodeLLDB extension currently doesn't show Rust enumerations correctly on
Windows when using Rust 1.65 or newer. This is an example to demonstrate that.

Steps to reproduce:

1. Open the folder in VS Code with the CodeLLDB and rust-analyzer extensions installed.
2. Set a break point at the last line in main.
3. Click the Debug link above the main declaration to start debugging.
4. Notice that the `advanced_a` and `advanced_b` variables don't say which values
   they are.
5. Change the Rust version in rust-toolchain from 1.64.0 to 1.65.0
6. Start debugging again and notice that enumeration values are no longer shown.

A screenshot of the example using Rust 1.64:

![Rust 1.64.0](assets/Rust%201.64.0%20%20looks%20correct.png)

A screenshot of the example using Rust 1.65:

![Rust 1.65.0](assets/Rust%201.65.0%20%20looks%20wrong.png)
