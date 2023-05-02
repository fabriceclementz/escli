# escli

> **A CLI for interacting with Elasticsearch (You Know, for Search)**

![Downloads](https://img.shields.io/github/downloads/fabriceclementz/escli/total) ![Contributors](https://img.shields.io/github/contributors/fabriceclementz/escli?color=dark-green) ![Forks](https://img.shields.io/github/forks/fabriceclementz/escli?style=social) ![Stargazers](https://img.shields.io/github/stars/fabriceclementz/escli?style=social) ![Issues](https://img.shields.io/github/issues/fabriceclementz/escli) ![License](https://img.shields.io/github/license/fabriceclementz/escli)

## Status

> Not production ready (working progress)

This project is still in its early days.

## About

**escli** is a fast and efficient command-line interface (CLI) tool for managing Elasticsearch indexes, executing searches, and retrieving insights from Elasticsearch data. With its intuitive and user-friendly interface, escli makes it easy to interact with Elasticsearch, regardless of whether you're managing a single instance or a large cluster.

Built using Rustlang, escli provides a reliable and scalable solution for developers and DevOps engineers who need to manage Elasticsearch data on a daily basis. Whether you're a seasoned Elasticsearch user or just getting started, escli offers a powerful and flexible toolset to help you optimize your workflow.

## Installation

TODO

## Usage

### Configuration

escli uses a configuration file that contains the connection parameters for each of the clusters you want to connect on. The format is as follows:

```yaml
clusters:
  local:
    host: 127.0.0.1
    port: 9200
    protocol: http
  staging:
    host: staging.es
    port: 9200
    protocol: https
  production:
    host: production.es
    port: 9200
    protocol: https
```

By default, the CLI will try to load a config file at `~/.escli/config.yaml` but you can provide another path via the `--config` flag as follows:

```sh
escli -c ./my-config.yaml
```

### Aliases

- [x] List
- [ ] Add
- [ ] Remove
- [ ] Update

### Mappings

- [x] Get

### Indices

- [x] List
- [x] Create
- [x] Open
- [x] Close
- [x] Delete
- [ ] Refresh
- [ ] Force merge
- [ ] Flush
- [ ] Reindex

**List indices**

```sh
$ escli --cluster local indices list -h
List all indices

Usage: escli indices list [OPTIONS]

Options:
  -o, --output <OUTPUT>  Output format [default: default] [possible values: default, json]
  -p, --pretty           Pretty print JSON output
  -c, --config <CONFIG>  Config file (default is $HOME/.escli.yaml)
  -v, --verbose          Make the operation more talkative
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

**Create index**

```sh
Create an index

Usage: escli indices create [OPTIONS] <NAME>

Arguments:
  <NAME>  Name of the index to create

Options:
  -o, --output <OUTPUT>  Output format [default: default] [possible values: default, json]
  -p, --pretty           Pretty print JSON output
  -c, --config <CONFIG>  Config file (default is $HOME/.escli.yaml)
  -v, --verbose          Make the operation more talkative
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

**Open index**

```sh
Opens a closed index

Usage: escli indices open [OPTIONS] <NAME>

Arguments:
  <NAME>  Name of the index to open

Options:
  -o, --output <OUTPUT>  Output format [default: default] [possible values: default, json]
  -p, --pretty           Pretty print JSON output
  -c, --config <CONFIG>  Config file (default is $HOME/.escli.yaml)
  -v, --verbose          Make the operation more talkative
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

**Close index**

```sh
Closes an index

Usage: escli indices close [OPTIONS] <NAME>

Arguments:
  <NAME>  Name of the index to close

Options:
  -o, --output <OUTPUT>  Output format [default: default] [possible values: default, json]
  -p, --pretty           Pretty print JSON output
  -c, --config <CONFIG>  Config file (default is $HOME/.escli.yaml)
  -v, --verbose          Make the operation more talkative
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

## Built With

- [Rust](https://www.rust-lang.org)
- [Clap](https://github.com/clap-rs/clap)

## License

Distributed under the MIT License. See [LICENSE](https://github.com/fabriceclementz/escli/blob/main/LICENSE.md) for more information.
