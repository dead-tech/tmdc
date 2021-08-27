![example workflow](https://github.com/Dead-tech/tmdc/actions/workflows/build.yml/badge.svg)
![example workflow](https://github.com/Dead-tech/tmdc/actions/workflows/tests.yml/badge.svg)

# tmdc - Tiny Markdown Compiler
Simple Markdown to HTML compiler written in Rust.

**_DISCLAIMER: The language itself hasn't been developed yet!_**

## Features
  - [x] Headings
  - [x] Paragraphs
  - [x] Italic
  - [x] Bold
  - [x] Multi-line code blocks
  - [x] Unordered lists 
  - [x] Line Breaks

## Dependecies
  * [rust](https://www.rust-lang.org/it)

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

  Test files can be found in the examples dir.
