# Sherpa

[![Build Status](https://travis-ci.org/mikeastock/sherpa-cli.svg?branch=master)](https://travis-ci.org/mikeastock/sherpa-cli)

A CLI for using Sherpa from the command line.  This tool allows you to deploy
from the comfort of your console.

## Installation

Homebrew:

```bash
brew install sherpa
```

If you are a Rust developer you can simply do:

```bash
cargo install sherpa
```

## Usage

To get started run:

```bash
sherpa authenticate <github-handle> <github-token>
```

### Deploying

Sherpa will infer what Trekker and branch you want to deploy to based on what
directory you run the command from.

To trigger a deploy run:
```bash
sherpa deploy <stage>
```

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/mikeastock/sherpa-cli. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [Contributor Covenant](contributor-covenant.org) code of conduct.

