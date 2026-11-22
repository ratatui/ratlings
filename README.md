# Ratlings 🐀

Small, interactive exercises for learning [Ratatui](https://ratatui.rs),
powered by [Rustlings](https://rustlings.rust-lang.org) 🦀

## Getting started

Install Rustlings, clone this repository, and start the exercise runner:

```sh
cargo install rustlings
git clone https://github.com/ratatui/ratlings

cd ratlings
rustlings
```

Rustlings watches your files and advances when an exercise passes. Common
commands inside the runner are `h` for a hint, `l` to list exercises, and `q`
to quit. To work on one exercise directly, run:

```sh
rustlings run <exercise_name>
```

## Developing exercises

See [CONTRIBUTING.md](CONTRIBUTING.md). The short version is:

```sh
rustlings dev update
rustlings dev check
```

## License

Licensed under the [MIT License](LICENSE).
