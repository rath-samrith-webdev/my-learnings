
Install Rust

However, if you want to download and install rust, you can go to [rust-lang.org](https://www.rust-lang.org/) and follow the instructions there.
or you can use [rustup](https://rust.rust-lang.org/), a tool to install rust.

```bash
curl -fsSL https://sh.rustup.rs -o sh | sh
```

```bash
rustup default stable
```

Check Installation
After installing, check if Rust is installed correctly by running:

```bash
rustc --version
```

output:

```text
rustc 1.86.0 (05f9846f8 2025-03-31)
```

create a new project
```bash
cargo new my_project
```

```bash
cd my_project
```

This creates a folder called my_project with the following files:

Cargo.toml: Project settings
src/main.rs: Main Rust file

The main.rs file contains this default code:

```rust
fn main() {
    println!("Hello, world!");
}
```

Build and Run the Project
Next, write the following code to go into the project folder:

```bash
cd my_project
```

```bash
cargo build
```

```bash
cargo run
```

The output should be:

```text
Hello, world!
```
