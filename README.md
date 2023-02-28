<div align="center">
  <a href="https://github.com/banyancomputer/fake-file" target="_blank">
    <img src="https://raw.githubusercontent.com/banyancomputer/fake-file/main/assets/a_logo.png" alt="fake-file Logo" width="100"></img>
  </a>

  <h1 align="center">fake-file</h1>
</div>

<div align="center"><sub>:warning: Work in progress :warning:</sub></div>

##

## Outline

- [Installation](#installation)
- [Testing the Project](#testing-the-project)
- [Benchmarking the Project](#benchmarking-the-project)
- [Getting Help](#getting-help)
- [External Resources](#external-resources)
- [License](#license)

## Installation

### Using `cargo`

```console
cargo install fake-file
```

## Usage
```console
# Get the help menu
fake-file --help
# Generate a 4 level deep directory structre. Each directory should have 4 files.
# Each end file should add up to 1024 bytes (1 KB) in size.
fake-file -d 4 -w 4 -s 1024 -o .
```

## Testing the Project

- Run tests

  ```console
  cargo test
  ```

## Benchmarking the Project

- Run benchmarks - this sees how long it takes to generate a 1GB file structure

  ```console
  cargo bench 
  ```

### Formatting

For formatting Rust in particular, please use `cargo fmt` as it uses
specific nightly features we recommend. **Make sure you have nightly
installed**.

## Getting Help

For usage questions, usecases, or issues please open an issue in our repository.

We would be happy to try to answer your question or try opening a new issue on Github.

## External Resources

These are references to specifications, talks and presentations, etc.

## License

This project is licensed under the [MIT License](./LICENSE),
or [http://opensource.org/licenses/MIT][mit].


[cargo-expand]: https://github.com/dtolnay/cargo-expand
[cargo-udeps]: https://github.com/est31/cargo-udeps
[cargo-watch]: https://github.com/watchexec/cargo-watch
[commit-spec]: https://www.conventionalcommits.org/en/v1.0.0/#specification
[commit-spec-site]: https://www.conventionalcommits.org/
[criterion]: https://github.com/bheisler/criterion.rs
[irust]: https://github.com/sigmaSd/IRust
[mit]: http://opensource.org/licenses/MIT
[pre-commit]: https://pre-commit.com/
[proptest]: https://github.com/proptest-rs/proptest
[strategies]: https://docs.rs/proptest/latest/proptest/strategy/trait.Strategy.html
