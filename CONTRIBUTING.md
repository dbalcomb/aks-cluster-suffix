# Contributing

Contribution to this project is welcomed but we ask that you adhere to the
following guidelines where applicable.

## Conduct

This project adopts the [Rust Code of Conduct][conduct] to provide a welcoming
environment for all contributors. Any queries or concerns relating to conduct
should be directed to the maintainers of this project and not the moderation
team as they are not affiliated.

## Developing

### Install rust

See the rust [installation documentation][install-rust] or use the following
command to install the rust toolchain installer _rustup_.

```sh
curl https://sh.rustup.rs -sSf | sh
```

### Install development tools (optional)

This project makes use of continuous integration to perform automated linting
and testing. Pull requests cannot be merged until all checks have passed so it
is advisable to run the checks locally before pushing to the remote repository.

The following command installs _cargo-make_, a task runner which is used to
simplify the execution of tasks that can be performed.

```sh
cargo install cargo-make
```

With this tool you can run various commands such as `cargo make lint` and
`cargo make test` to manually invoke linting and testing tasks respectively.

The following commands can be used to setup your local development environment
to use git hooks that lint on commit and test on push. Be aware that the hooks
are dependent on the _cargo-make_ tool as described above.

```sh
cargo install rusty-hook
rusty-hook init
```

[conduct]: https://github.com/rust-lang/rust/blob/master/CODE_OF_CONDUCT.md
[install-rust]: https://www.rust-lang.org/tools/install
