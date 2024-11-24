# Code Documentation

## Table of Contents

- [Generating and Testing a Build](#generating-and-testing-a-build)

## Generating and Testing a Build

This is intended to be used to generate a version of the engine that can easily be tested without being overwitten by
the next build.
To generate a build, run the following command:

```bash
scripts/build.sh <build name>
```

This will:

- Run cargo release build with the `target-cpu=native` flag
- Create a new folder in the `builds` directory with the given build name
- Move the generated binary to the new folder