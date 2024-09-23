# Today's Work
- Install VSCode Rust analyzer
- Try Hello World
- Get done with rust book chapter through 1~2

rustc 1.76.0 (07dca489a 2024-02-04)

## Compiling with rust

when Rust is finished with compile on linux and macos you'll see two files (the main.rs and main). main is the binary executable.
On mac os you can run it via
```
$ ./main
```
Rust is <em>ahead-of-time compiled</em> which means the person who runs this file doen't have to run it with `rust`

## WTF Cargo

Cargo == Package Manechan

using `cargo build` will create the new cargo binary file and using `./target/debug/hello_cargo` will run the executeable

using `cargo run` will detect whether the source file have been changed and if not it will just run the binary and if yes it will compile and run the binary.

using `cargo check` will see if the code could be compiled or not. this will not create the binary file.

using `cargo build --release` will create he release file and will going to optimize the rust code. this takes longer than just compiling but makes your bin executable faster.

### Guess the number

String == UTF-8
does rust support base64? base64 vs UTF-8

stdin -> IO

when `use std::io` is not on top you should use `std::io::stdin`

```
let mut guess = String::new() // ownership f@ck
```
`.expect()` is method and it is better to put a newline between methods.

`read_line` not only saves the string user input, it will return on value.
`io::Result` Result is enumeration in short enums.
enums got `variants`.

```
let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```
rust supports shadow.(which means just re-justifying already used variable without making guess_str or guess)

you can use loop to go over and over again ez

```

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

```
if string have been successfully turned into num it will return the number, if not goes to Err and will skip the rest of the loop and get to the start of the loop.

### rust-analyzer error

```
ERROR FetchWorkspaceError: rust-analyzer failed to discover workspace
```
This error happens because you have to open the rust file with cargo.toml as a root directory in vscode.