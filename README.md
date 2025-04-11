
# Pluto Chess Engine

by [Castled Org](https://github.com/CastledChess)

---

## Table of Contents

- [About](#about)
- [Features](#features)
- [Building](#building)
- [Usage](#usage)
- [Documentation](#documentation)

---

## About

**Pluto** is a UCI-compatible chess engine written in **Rust**, leveraging the [Shakmaty Chess Library](https://github.com/niklasf/shakmaty). It was primarily designed for analyzing chess games as part of the **Castled Chess Project**.

---

## Features

### Core Features

- UCI Compatibility
- Move Generation powered by [Shakmaty](https://github.com/niklasf/shakmaty)

### Search Techniques

- **Negamax**
- **Alpha-Beta Pruning**
- **Iterative Deepening**
- **Transposition Tables**
- **Move Ordering**
- **Principal Variation Search**
- **Reverse Futility Pruning**
- **Quiescence Search**
- **Draw & Checkmate Detection**

### Evaluation

- **Pesto Evaluation**

---

## Building

To build the engine, clone the repository and run the following command in your terminal:

```bash
cargo build --release
```

This will compile the engine in release mode for optimal performance.

---

## Usage

Castled Engine communicates using the [UCI protocol](http://wbec-ridderkerk.nl/html/UCIProtocol.html), so it can be used with any UCI-compatible GUI. Alternatively, you can run the engine directly in the terminal by executing:

```bash
cargo run --release
```

This will start the engine and you can interact with it through the terminal.

---

## Documentation

Comprehensive documentation for Castled Engine can be found [here](./docs/README.md).
