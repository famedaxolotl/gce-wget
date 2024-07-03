# gce-wget

[![GitHub](https://img.shields.io/badge/GitHub-Profile-blue?logo=github)](https://github.com/famedaxolotl) [![crates.io](https://img.shields.io/crates/v/gce-wget.svg)](https://crates.io/crates/gce-wget) [![Total Downloads](https://img.shields.io/crates/d/gce-wget.svg)](https://crates.io/crates/gce-wget) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## v1.0.2 What's new

### Force qualification flags: O Levels support added

- ***You can now optionally force the program to search a particular qualification using the `-i`, `-a`, `-o` options, for IGCSE, A Levels, and O Levels respectively. Automatic search still remains if these options are not given, but only for IGCSE and A Levels. See details in Usage below.***

gce-wget is a specialized tool to bulk download IGCSE and AS/A Levels (O Levels not yet supported) papers from papers.gceguide.net, skipping the need to write regexes and make link files. This tool requires wget to be pre-installed on the machine.

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

The basic command  structure (no options) is:

```bash
gce-wget [SUBJECT_CODE] [YEARS]
```

For example, `gce-wget 0620 2021,2022` will download all papers of IGCSE Chemistry from 2021 and IGCSE Chemistry 2022

To selectively download papers, use the `-t` (`--types`) and `-c` (`--codes`) options, which allow you to enter comma-seperated lists of paper types and codes.

For example:

```bash
gce-wget 0620 -t ms,qp -c 21,41 2022,2023
```

This downloads all Variant 1 extended MCQ and Theory papers of IGCSE Chemistry years 2022 and 2023, and their marking schemes.

To download O Levels papers, use the `-o` (`--o-level`) option like so:

```bash
gce-wget 5070 -o -t ms,qp -c 11,12 2021
```

This downloads all Paper 11 and Paper 12 question question paper and marking schemes from O Levels Chemistry 2021.

Use the `-i` (`--igcse`) and `-a`(`-a-level`) options if, for example, the programs seems to searching the wrong qualification.

To understand the paper naming system, [see this.](https://papers.gceguide.net/assets/images/res_guide.svg).
