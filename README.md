## Installing rust

- Assuming you're on Unix system, i.e Linux or MacOS and don't have Rust installed, install Rust with the following: 

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- If not on a Unix system, i.e Windows(which I don't know why you're using Windows in the first place but no judgement here), follow the instructions [here](https://doc.rust-lang.org/book/ch01-01-installation.html) to install rust.

## How to run the challenge

1. Create a new rust project with `cargo new <projectName>` e.g

```bash
cargo new zk_password_verifier
```

2. Replace the default `main.rs` with our zk challenge.
3. Your `cargo.toml` file should look something like the below:

```rust
[package]
name = "zk_password_verifier"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
sha2 = "0.10.6"
hex = "0.4.3"
```
I recommend having a good Rust linter to make your life easier with this challenge. If on VS Code or VS Codium, I recommend [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) which looks like the below:

![rust analyzer](./rust-analyzer.png)


And if you're a chad on neovim, vanilla vim or God forbid emacs, check documentation [here](https://rust-analyzer.github.io/) to get rust analyzer on your development environment.


4. Run the binary as 

```bash
cargo run --release
```
5. Interact with the challenge using netcat or any other tool you see fit.

---

# May the cryptography gods be with you!
