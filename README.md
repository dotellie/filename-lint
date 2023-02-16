# filename-lint

Lints your tree's filenames with minimal configuration.

## Usage

```sh
filename-lint # lints the current directory for kebab-case formatting
filename-lint -p src -f snake_case # lints the src directory for snake_case formatting
```

```sh
$ filename-lint --help
Usage: filename-lint [OPTIONS]

Options:
  -p, --path <PATH>              The root path to lint [default: .]
  -f, --formatting <FORMATTING>  Expected formatting for files [default: kebab-case] [possible values: kebab-case, lower-camel-case, pascal-case, shouty-kebab-case, shouty-snake-case, snake-case, title-case, train-case, upper-camel-case]
  -h, --help                     Print help
  -V, --version                  Print version
```
