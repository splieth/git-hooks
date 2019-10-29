# Git-Hooks

> A collection of git-hooks (written in rust).
> At the moment only contains a commit-msg hook that automatically adds co-authored-by-lines.

## Table of Contents

  * [Table of Contents](#table-of-contents)
  * [Prerequisites](#prerequisites)
  * [Installing](#installing)
  * [commit-msg](#commit-msg)
     * [Usage example](#usage-example)
     * [Configuration](#configuration)
  * [Running the tests](#running-the-tests)
  * [Built With](#built-with)
  * [Release History](#release-history)
  * [Meta](#meta)
  * [Contributing](#contributing)

## Prerequisites

You should have `cargo` and `rust` installed to compile yourself.

## Installing

Compile it with `make release`.
Put the binary in your git-hooks directory (e.g. ~/git-hooks).
In your .gitconfig:
```bash
[core]
	hooksPath = ~>/git-hooks
```

Set the ENV-Variable to the path to your config.
(e.g. `COMMIT_TEAM_CONFIG=~/dotfiles/commit-msg-conf.yaml`)

## commit-msg

### Usage example

When compiled and linked successfully, a configuration like: 
```yaml
regex: "\\[.+?\\]\\s(.*?)\\s.*"
separator: "|"
me: fli
team:
  - short: hug
    name: Hugo Heimlich
    email: hugo.heimlich@domain.com
  - short: fli
    name: Fliedbelt Igel
    email: fliedbelt.igel@domain.com
```

changes this commit message
```text
[STORY-123] hug|fli some commit message
```

to
```text
[STORY-123] hug|fli some commit message


Co-authored-by: Hugo Heimlich <hugo.heimlich@domain.com>
```

### Configuration

The Configuration is a simple yaml file.
```yaml
regex: "<Regex to get team-members from the commit-msg>"
separator: "<separator between team-members>"
me: <your short name>
team:
  - short: <short name in commit-msg>
    name: <full name>
    email: <git mail adress>
  [...]
```

## Running the tests

Run `make test` to execute all tests via _cargo_.

## Built With

* [regex](https://github.com/rust-lang/regex) - library for regular expressions
* [exitcode](https://github.com/benwilber/exitcode) - system exit codes
* [yaml-rust](https://github.com/chyh1990/yaml-rust) - yaml-parser

## Release History

* v1.0.0
    * add `me` to then ignore your own short-name when building the co-authored-by-lines (commit-msg)
* v0.1.0
    * initial release

## Meta

Twitter: [@fr3dch3n](https://twitter.com/fr3dch3n)

Distributed under the Apache 2.0 license. See ``LICENSE`` for more information.

## Contributing

1. Fork it (<https://github.com/fr3dch3n/git-hooks/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request
