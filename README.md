
# Pluto Chess Engine

by [Lxdovic](https://github.com/Lxdovic)

## Table of Contents

- [About](#about)
- [Features](#features)
- [Building](#building)
- [Usage](#usage)
- [Documentation](#documentation)

## About

**Pluto** is a UCI-compatible chess engine written in **Rust**, the current version uses the [Shakmaty Chess Library](https://github.com/niklasf/shakmaty) for move generation. It was initially designed as an analysis engine for the the **Castled Chess Project**.

## Strength

| Version        | Date       | Estimated ELO |
| ----------     | ---------- | ------------- |
| Before Release | 8 May 2025 | 2870          |

## Testing

Pluto is tested with OpenBench, a distributed testing framework for UCI chess engines. Hosted at: https://openbench.castled.app

## Features

- UCI Compatible
- Move Generation powered by [Shakmaty](https://github.com/niklasf/shakmaty)
- Search:
    Pluto uses a negamax search with alpha-beta pruning and is reinforced by many other techniques and heuristics
    - **Negamax**
    - **Alpha-Beta Pruning**
    - **Iterative Deepening**
    - **Transposition Tables**
    - **Principal Variation Search**
    - **Reverse Futility Pruning**
    - **Extended Futility Pruning**
    - **Late Move Reductions**
    - **Late Move Pruning**
    - **Null Move Pruning**
    - **Improving Heuristic**
    - **Internal Iterative Reductions**
    - **Quiescence Search**
    - **Draw & Checkmate Detection**
- Move Ordering:
    In order to improve the efficiency of the alpha-beta framework, pluto uses a few move ordering tehchniques and heuristics
    - **Most Valuable Victim - Less Valuable Attacker (MVV-LVA)**
    - **History heuristics**
    - **Killer Moves**
    - **Transposition Tables**
- Evaluation:
    Pluto adopted Efficiently Updatable Neural Networks for its evaluation function quite early in development. Earlier versions were using Simple Eval/Pesto Eval
    - **NNUE (768->512)x2->1** Trained using [Bullet](https://github.com/jw1912/bullet) and Stockfish data.

## Building

To build the engine, clone the repository and run the following command in your terminal:

```bash
make
```

## Contributors

- [PaulJhonson26](https://github.com/PaulJhonson26) (Eliott Reigner) for the initial implementations of pv collection and killer moves
- [MehdiAribi23](https://github.com/Mehdiaribi23) (Mehdi Aribi) for some documentation improvements

## Credits

Pluto was built using these resources and tools

- [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page)
- [Carp](https://github.com/dede1751/carp) -> some ideas and the first NNUE inference implementation of pluto came from Carp, the code has been mostly rewritten by now.
- [Shakmaty](https://github.com/niklasf/shakmaty) -> great move generator. sped up the development process by alot
- [OpenBench](https://github.com/AndyGrant/OpenBench) -> great tool for distributed SPRT testing and SPSA tuning
- [Bullet](https://github.com/jw1912/bullet) -> great tool for building NNUEs
- [Stockfish](https://stockfishchess.org/) -> some implementation examples and ideas

