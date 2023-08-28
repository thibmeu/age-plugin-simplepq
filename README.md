# age-plugin-simplepq: Simple Post Quantum plugin for age

[![Documentation](https://img.shields.io/badge/docs-main-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/age-plugin-simplepq.svg)
[![crates.io](https://img.shields.io/crates/v/age-plugin-simplepq.svg)][Crates.io]

[Crates.io]: https://crates.io/crates/age-plugin-simplepq
[Documentation]: https://docs.rs/age-plugin-simplepq/

age-plugin-simplepq is a plugin for [age](https://github.com/C2SP/C2SP/blob/main/age.md). It provides an age Identity and Recipient consuming X25519Kyber768Draft00 encoded files.

age-plugin are defined by [C2SP](https://github.com/C2SP/C2SP/blob/main/age.md).

## Tables of Content

* [Features](#features)
* [Installation](#installation)
* [Usage](#usage)
* [Security Considerations](#security-considerations)
* [FAQ](#faq)
* [License](#license)

## Features

* Post Quantum recipients and identities with X25519Kyber768Draft00
* Plugin cli for age
* Cross platform (Linux, Windows, macOS)

## What's next

* Agree on age format

## Installation

| Environment        | CLI Command                         |
|:-------------------|:------------------------------------|
| Cargo (Rust 1.67+) | `cargo install age-plugin-simplepq` |

Read [age installation instructions](https://github.com/FiloSottile/age#installation) to install age.

## Usage

You can use the `--help` option to get more details about the command and its options.

```text
age-plugin-simplepq [-o OUTPUT]
age-keygen-simplepq -y [-o OUTPUT] [INPUT]
```

### Generate recipient and identity

```shell
age-plugin-simplepq -o my_id.key
```

For convenience, you can also create an associated recipient

```shell
age-plugin-simplepq -y -o my_id.key my_id.key
```

> The recipient and identity size are going to be large.

### Encrypt and decrypt

Encrypt `Hello age-plugin-simplepq!` string with your new key.

```shell
echo 'Hello age-plugin-simplepq!' | age -a -R my_id.key.pub > data.age
age --decrypt -i my_id.key data.age
Hello age-plugin-simplepq!
```

## Security Considerations

This software has not been audited. Please use at your sole discretion. With this in mind, age-plugin-simplepq security relies on the following:

* [age](https://github.com/C2SP/C2SP/blob/main/age.md) encryption protocol, and its implementation in [str4d/rage](https://github.com/str4d/rage),
* [HPKE RFC 9180](https://www.rfc-editor.org/rfc/rfc9180.html) by R. Barnes, K. Bhargavan, B. Lipp, C. Wood, its implementation in [rozbb/rust-hpke](https://github.com/rozbb/rust-hpke),and its binding for age [age-plugin-hpke](https://github.com/thibmeu/age-plugin-hpke) ,

## FAQ

Empty

## License

This project is under the MIT license.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be MIT licensed as above, without any additional terms or conditions.
