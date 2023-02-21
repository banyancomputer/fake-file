<div align="center">
  <a href="https://github.com/banyancomputer/fake-file" target="_blank">
    <img src="https://raw.githubusercontent.com/banyancomputer/fake-file/main/assets/a_logo.png" alt="fake-file Logo" width="100"></img>
  </a>

  <h1 align="center">fake-file</h1>

  <p>
    <a href="https://crates.io/crates/fake-file">
      <img src="https://img.shields.io/crates/v/fake-file?label=crates" alt="Crate">
    </a>

[//]: # (    <a href="https://codecov.io/gh/banyancomputer/fake-file">)

[//]: # (      <img src="https://codecov.io/gh/banyancomputer/fake-file/branch/main/graph/badge.svg?token=SOMETOKEN" alt="Code Coverage"/>)

[//]: # (    </a>)

[//]: # (    <a href="https://github.com/banyancomputer/fake-file/actions?query=">)

[//]: # (      <img src="https://github.com/banyancomputer/fake-file/actions/workflows/tests_and_checks.yml/badge.svg" alt="Build Status">)

[//]: # (    </a>)

[//]: # (    <a href="https://github.com/banyancomputer/fake-file/blob/main/LICENSE">)

[//]: # (      <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License">)

[//]: # (    </a>)

[//]: # (    <a href="https://docs.rs/fake-file">)

[//]: # (      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue" alt="Docs">)

[//]: # (    </a>)
  </p>
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

## Testing the Project

- Run tests

  ```console
  cargo test
  ```

## Benchmarking the Project

- Run benchmarks

  ```console
  cargo bench --features test_utils
  ```

### Formatting

For formatting Rust in particular, please use `cargo +nightly fmt` as it uses
specific nightly features we recommend. **Make sure you have nightly
installed**.

### Pre-commit Hook

This project recommends using [pre-commit][pre-commit] for running pre-commit
hooks. Please run this before every commit and/or push.

- Once installed, Run `pre-commit install` and `pre-commit install --hook-type commit-msg`
  to setup the pre-commit hooks locally. This will reduce failed CI builds.

- If you are doing interim commits locally, and for some reason if you _don't_
  want pre-commit hooks to fire, you can run
  `git commit -a -m "Your message here" --no-verify`.

### Recommended Development Flow

- We recommend installing and leveraging [cargo-watch][cargo-watch],
  [cargo-expand][cargo-expand] and [irust][irust] for Rust development.

### Conventional Commits

This project *lightly* follows the [Conventional Commits
convention][commit-spec-site] to help explain
commit history and tie in with our release process. The full specification
can be found [here][commit-spec]. We recommend prefixing your commits with
a type of `fix`, `feat`, `docs`, `ci`, `refactor`, etc..., structured like so:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

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
