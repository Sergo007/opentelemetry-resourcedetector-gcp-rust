# Contributing

## Pull Requests

### How to Send Pull Requests

Everyone is welcome to contribute code to `opentelemetry-resourcedetector-gcp-rust` via
GitHub pull requests (PRs).

```sh
git clone https://github.com/Sergo007/opentelemetry-resourcedetector-gcp-rust
```

Enter the newly created directory and add your fork as a new remote:

```sh
git remote add <YOUR_FORK> git@github.com:<YOUR_GITHUB_USERNAME>/opentelemetry-resourcedetector-gcp-rust
```

Check out a new branch, make modifications, run linters and tests, and
push the branch to your fork:

```sh
$ git checkout -b <YOUR_BRANCH_NAME>
# edit files
$ git add -p
$ git commit
$ git push <YOUR_FORK> <YOUR_BRANCH_NAME>
```

Open a pull request against the main
[opentelemetry-resourcedetector-gcp-rust](https://github.com/Sergo007/opentelemetry-resourcedetector-gcp-rust)
repo.

> **Note**
> It is recommended to run [pre-commit script](precommit.sh) from the root of
the repo to catch any issues locally.


## Style Guide

- Run `cargo clippy --all` - this will catch common mistakes and improve your Rust code
- Run `cargo clippy --fix`
- Run `cargo fmt` - this will find and fix code formatting issues.

## Testing and Benchmarking

- Run `cargo test --all` - this will execute code and doc tests.

