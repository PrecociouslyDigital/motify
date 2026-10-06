# Motify
Motify is a quick little portable utility for Windows, Macos and Linux to declaratively manage your symlinks.
It's designed to help you keep all your configuration files for your various applications to a central place,
for ease of version control and backup.

[![Build and Release](https://github.com/PrecociouslyDigital/motify/actions/workflows/release.yml/badge.svg)](https://github.com/PrecociouslyDigital/motify/actions/workflows/release.yml)

## Usage
```
Usage: motify [OPTIONS] <COMMAND>

Commands:
  deploy    Deploy symlinks according to a motify.yaml file
  undeploy  Remove symlinks according to a motify.yaml file
  help      Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  Sets a custom config file [default: motify.yaml]
  -v, --verbose...       Output verbose output
  -h, --help             Print help
  -V, --version          Print version
```

`deploy -o` replaces a target that is already a symlink; it never replaces a real file or directory.
`undeploy` only removes a target that is a symlink pointing at its source.
motify exits with status 1 if the config can't be read or any entry fails.

## Config
```yaml
deploy:
  vim:                      # a name used in progress messages
    source: ./vimrc         # relative to the directory motify is run from
    target: ~/.vimrc
  terminal:
    source: ./$terminal
    target:                 # any value can be chosen per OS (linux, macos, windows)
      linux: ~/.config/$terminal
      macos: ~/Library/Application Support/$terminal
env:                        # variables for $name / ${name}; the process environment is used as a fallback
  terminal: alacritty
```
Using a variable that is defined in neither `env:` nor the environment is an error.
