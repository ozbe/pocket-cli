# Pocket

[Pocket API](https://getpocket.com/developer/apps/) command-line client.

A utility for interacting with the Pocket API.

## Compile from Source

```bash
$ git clone git@github.com:ozbe/pocket-cli.git
$ cd pocket-cli
$ cargo build --release
```
The build output is `./target/release/pocket-cli`. Copy and execute that as you
desire.

## Usage

Run pocket with `pocket-cli -h` or `pocket-cli --help` to view the latest available flags, arguments, and
commands.

```text
pocket-cli 0.1.0
Interact with the Pocket API

USAGE:
    pocket-cli [OPTIONS] --consumer-key <consumer-key> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --access-token <access-token>    Pocket access token [env: POCKET_ACCESS_TOKEN=]
        --consumer-key <consumer-key>    Pocket consumer key [env: POCKET_CONSUMER_KEY=]

SUBCOMMANDS:
    add     Add
    auth    Authenticate
    get     Get
    help    Prints this message or the help of the given subcommand(s)
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.