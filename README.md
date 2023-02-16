# path-lint

Lints your tree's filenames with minimal configuration.

## Installation

### NPM

```sh
npm install -g path-lint
```

Or install it locally to your project:

```sh
npm install --save-dev path-lint
```

and add it to your lint script in `package.json`:

```json
{
  "scripts": {
    "lint": "path-lint && <other-linters>"
  }
}
```

### Cargo

```sh
cargo install path-lint
```

## Usage

```sh
path-lint # lints the current directory for kebab-case formatting
path-lint -p src -f snake_case # lints the src directory for snake_case formatting
```

```sh
$ path-lint --help
Usage: path-lint [OPTIONS]

Options:
  -p, --path <PATH>              The root path to lint [default: .]
  -f, --formatting <FORMATTING>  Expected formatting for files [default: kebab-case] [possible values: kebab-case, lower-camel-case, pascal-case, shouty-kebab-case, shouty-snake-case, snake-case, title-case, train-case, upper-camel-case]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Configuration

One of the goals of this project is to **avoid** configuration. As of right now, the only configuration is the formatting you want to enforce. This is done via the `-f` or `--formatting` flag.

## Help! A file I cannot rename is failing the linter

Because of the goal to be as configuration-less as possible, we encourage you to open a PR to add the file to `src/ignore.rs` if the file is required to have a specific casing by some tool. For example, `package-lock.json` is already excluded as it is required to be in `kebab-case` by `npm`.

While this of course is a hassle right now, the end goal is to have this tool be as simple as possible for everyone in the future, something which would not be possible if everyone had to configure their own ignore list.

## Comparison to ls-lint

[path-lint](https://ls-lint.org) is a similar tool made to lint filenames. The approach taken by path-lint is however very different. ls-lint relies heavily on configuration, meaning that you have to specify everything from the ground up. path-lint on the other hand automatically parses any `.gitignore` files present in your tree and assumes that you want all non-configuration files to follow the same convention. This means that you can get started with path-lint in seconds, while ls-lint requires you to configure quite a bit.

On a very quick, non-scientific benchmark, path-lint and ls-lint perform roughly the same though as ls-lint cannot be configured to lint all files, the results are not directly comparable. ls-lint also required a ~40 lines long configuration file with specialized regex to lint the same files as path-lint.
