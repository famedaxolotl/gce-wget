# gce-wget

![crates.io](https://img.shields.io/crates/v/gce-wget.svg) ![docs.rs](https://docs.rs/gce-wget/badge.svg)![Total Downloads](https://img.shields.io/crates/d/gce-wget.svg)

gce-wget is a specialized tool to bulk download IGCSE and AS/A Levels (O Levels not yet supported) papers from papers.gceguide.net, skipping the need to write regexes and make link files. This tool requires wget to be pre-installed on the machine (alternatives such as cURL are being considered).

## Installation

Use any of the following methods.

### 1. Downloading binary

- You can download the binary from the [releases page.](https://github.com/famedaxolotl/gce-wget/releases)

- Move the binary `gce-wget` to `/usr/bin` with

```bash
sudo mv /Downloads/gce-wget /usr/bin
```

### 2. Using Cargo

If you are a Rust programmer and have Cargo installed, simply run the following:

```bash
cargo install gce-wget
```

This will install the binary from crates.io

### 3. Building from GitHub

If you have Cargo installed, run the following commands

```bash
git clone https://github.com/famedaxolotl/gce-wget

cd gce-wget

cargo build --release
```

Now, run with `cargo run` or move the binary to `/usr/bin` with:

`sudo mv target/release/gce-wget /usr/bin`

## Usage

Use `gce-wget --help` to see detailed help page.

***NOTE: Only 1 year at a time for now***

The basic command  structure (no options) is:

```bash
gce-wget [SUBJECT_CODE] [YEAR]
```

For example, `gce-wget 0620 2022` will download all papers of IGCSE Chemistry from 2022

To selectively download papers, use the `-t` (`--types`) and `-c` (`--codes`) options, which allow you to enter comma-seperated lists of paper types and codes.

For example:

```bash
gce-wget 0620 -t ms,qp -c 21,41 2022
```

This downloads all Variant 1 extended MCQ and Theory papers of IGCSE Chemistry 2022.

To understand the paper naming system, [see this.](https://papers.gceguide.net/assets/images/res_guide.svg).

### Problems remaining

- The error handling is absolutely horrendous right now
- only one year is accepted
- performance improvements in the url creator
- maybe adopt a better module system
- O Levels not accepted
