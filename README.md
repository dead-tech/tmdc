![example workflow](https://github.com/Dead-tech/tmdc/actions/workflows/cargo.yml/badge.svg)

# tmdc - Tiny Markdown Compiler
Simple Markdown to HTML compiler written in Rust.

## Features
  - [x] Headings
  - [x] Paragraphs
  - [x] Multi-line code blocks
  - [x] Unordered lists 

## Dependecies
  * [rust](https://www.rust-lang.org/it)
  * [regex](https://docs.rs/regex/1.5.4/regex/) (may be removed)

## Usage

  ### Running with cargo
  ```sh
  $ cargo run -- <filepath>.md
  
  ```
  ### Building with cargo then running
  ```
  $ cargo build
  $ ./target/debug/tmdc <filepath>.md
  ```
