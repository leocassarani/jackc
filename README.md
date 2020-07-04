# Jackc

Jackc is a simple, end-to-end compiler for the Jack programming language, which is the high-level, object-oriented programming language implemented as part of the [Nand to Tetris](https://www.nand2tetris.org) course, as specified in the book [_The Elements of Computing Systems_](https://mitpress.mit.edu/books/elements-computing-systems) by Nisan and Shocken.

Unlike the approach taken in the book, which separates each stage of the compiler into different programs, Jackc is designed to transform `.jack` files all the way down to the Hack machine language, which can be eecuted by the Hack CPU specified in the book, or a suitable emulator.

## Usage

To compile a file containing Jack source code, simply run:

```
$ jackc Main.jack
```

This will produce a `Main.hack` file in the current directory. To write to a different filename, use the `-o` flag. To output to stdout, use the `--stdout` flag instead.

Jackc can compile any number of files or directories into a single Hack program. The Nand to Tetris course ships with eight `.vm` files that implement basic "OS" functionality such as drawing to the screen, reading from the keyboard, and a dynamic memory allocator. For any moderately complex Jack program, it's expected that some or all of the OS `.vm` files will be available to the compiler alongside the Jack source code. Jackc understands both `.jack` and `.vm` input files and will treat them equally.

Jackc can output one of three file formats:

* a `.hack` file containing the binary representation of the compiled CPU instructions as 16-bit ASCII ones and zeroes, one per line. Following the book's example, this is the default file format, and can also be specified using the `--hack` flag.
* a `.bin` file containing the binary, big-endian representation of the CPU instructions. This can be selected using the `--bin` flag.
* an `.asm` file containing the human-readable, assembly language translation of the CPU instructions. Use the `--asm` flag to specify this.

According to the specification in the book, execution of a Hack program is supposed to start at the `Sys.init` function, which forms part of the provided OS `.vm` files. However, the name of the program's start point can be overwritten using the `--init` flag, which can be useful if a non-standard OS is used. On the other hand, when compiling very simple `.vm` files that are not divided into separate functions, a `--no-init` flag can be given to start execution from the first line in the input file instead.

## License

Jackc is licensed under the terms of the Apache 2.0 license. See [LICENSE](LICENSE) for details.
