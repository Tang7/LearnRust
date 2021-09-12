## HTTPie

HTTPie in Rust.

### Crate Management
1. Install _cargo-edit_: `cargo install cargo-edit`
2. Add needed package, i.e, `cargo add clap --allow-prerelease`


### Command Line Process

**clap**

Key Studies:
* Using trait `#[]`
    > To be exactly, #[derive] attribute, capable of using basic implementation from some traits.
* Why needs to derive trait in the remaining types?
    > explicitly declare using implementation from derived trait.
* What does `parse()` do?
    > parse string slice into any types implements the FromStr trait.

    > string slice is the argument from command line.

    > subcommand defined two support command: get and post. which are support different argument list.
* What is `{:?}`?
    > a formatter from trait Debug which formats value.

### HTTP Process

### Style Output
