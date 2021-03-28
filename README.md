# manrfc

[![Version](https://img.shields.io/badge/rustc-1.50+-ab6000.svg)](https://blog.rust-lang.org/2021/02/11/Rust-1.50.0.html)
![MIT or Apache 2.0 licensed](https://img.shields.io/badge/license-MIT-blue)

`manrfc` is a simple CLI interface to the [RFC Editor](https://www.rfc-editor.org/) website and allows to easily
search and view inside the terminal all the RFCs using [minus](https://crates.io/crates/minus) as internal pager.

## Features

- Search an RFC by its title or the abstract
- Limit the number of results returned (default is 10)
- View the RFC inside the terminal using the [minus](https://crates.io/crates/minus) pager

## Requirements
`manrfc` internally uses [reqwest](https://github.com/seanmonstar/reqwest) to makes HTTP requests, for this reason it is
necessary to install the following dependencies based on the OS used.

On Linux:

- OpenSSL 1.0.1, 1.0.2, 1.1.0, or 1.1.1 with headers (see https://github.com/sfackler/rust-openssl)

On Windows and macOS:

- Nothing.

On Debian based distros, OpenSSL could be installed with the following command:
```sh
sudo apt install libssl-dev
```

#### Compile for source
In order to compile `manrfc` from scratch, type the following commands:

```sh
git clone https://github.com/cdzeno/manrfc.git
cd manrfc
cargo build --release
```
and install it with:
```sh
cargo install --path .
```

## License

MIT
