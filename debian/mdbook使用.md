# Rust圣经生成EPUB问题解决总结

## 安装[rust-book](https://github.com/rust-lang/book?tab=readme-ov-file)

```bash
git clone https://github.com/rust-lang/book.git
cd book
```

rust版本问题

rust-toolchain

```toml
[toolchain]
channel = "stable"
```

book.toml

```toml
[output.epub]
title = "The Rust Programming Language"
authors = ["Steve Klabnik", "Carol Nichols", "Chris Krycho", "Contributions from the Rust Community"]
language = "en"
```

```bash
cargo install mdbook --locked --version "0.4.44"
cargo install --locked --path packages/mdbook-trpl
cargo install mdbook-epub --force
mdbook-epub --version
mdbook-epub -s
mdbook build
ls book/epub
```  
