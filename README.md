# Castled Engine

by [Castled Org](https://github.com/CastledChess)

## Table of Contents

- [About](#about)
- [Features](#features)
- [Building](#building)
- [Usage](#usage)
- [Documentation](#documentation)

## About

Castled Engine is a UCI compatible chess engine written in Rust using the
[shakmaty chess library](https://github.com/niklasf/shakmaty). It was
designed to analyse games for the Castled Chess Project.

## Features

- UCI compatible
- Search
    - Negamax
    - Alpha-Beta Pruning
    - Iterative Deepening
    - Transposition Tables
- Evaluation
    - Material
    - Piece Square Tables
- Move Generation using [shakmaty](https://github.com/niklasf/shakmaty)

## Building

To install the engine, clone the repository and run the following command:

```bash
cargo build --release
```

## Usage

The engine uses the [Uci protocol](http://wbec-ridderkerk.nl/html/UCIProtocol.html), to communicate with the engine, you
can use
any UCI compatible GUI. Alternatively, you can run the engine in the terminal
using the following command:

```bash
cargo run --release
```

## Documentation

The documentation for the engine can be found [here](./docs/README.md).