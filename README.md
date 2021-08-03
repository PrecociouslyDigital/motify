# Motify
Motify is a quick little portable utility for Windows, Macos and Linux to declaratively manage your symlinks.
It's designed to help you keep all your configuration files for your various applications to a central place,
for ease of version control and backup.

[![Build and Release](https://github.com/PrecociouslyDigital/motify/actions/workflows/release.yml/badge.svg)](https://github.com/PrecociouslyDigital/motify/actions/workflows/release.yml)

## Usage
```
    motify.exe [FLAGS] [OPTIONS] <SUBCOMMAND>
    
    FLAGS:
        -h, --help       Prints help information
        -v, --verbose    Output verbose output
        -V, --version    Prints version information
    
    OPTIONS:
        -c, --config <config>    Sets a custom config file. [default: motify.yaml]
    
    SUBCOMMANDS:
        deploy      Deploy symlinks according to a motive.yaml file
        help        Prints this message or the help of the given subcommand(s)
        undeploy    remove symlinks according to a motive.yaml file
```