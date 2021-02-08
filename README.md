# Pocket

[Pocket API](https://getpocket.com/developer/apps/) command-line client.

A utility for interacting with the Pocket API.

## Installation

### macOS (OSX)

```bash
$ curl -L https://github.com/ozbe/pocket-cli/releases/download/v1.0.0/pocket-macos -o pocket
$ chmod +x pocket
$ mv pocket /usr/local/bin
```

### Ubuntu (Linux)

```bash
$ wget https://github.com/ozbe/pocket-cli/releases/download/v1.0.0/pocket-linux -o pocket
$ chmod +x pocket
$ mv pocket /usr/local/bin
```

### Windows

Download the latest Windows
[release](https://github.com/ozbe/pocket-cli/releases) and place the executable
in a folder that is in your $PATH.

## Compile from Source

```bash
$ git clone git@github.com:ozbe/pocket-cli.git
$ cd pocket-cli
$ cargo build --release
```
The build output is `./target/release/pocket`. Copy and execute that as you
desire.

## Usage

Run pocket with `pocket -h` or `pocket --help` to view the latest available flags, arguments, and
commands.

```text
pocket 1.0.0
Interact with the Pocket API

USAGE:
    pocket [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --access-token <access-token>    Pocket access token [env: POCKET_ACCESS_TOKEN=]
        --consumer-key <consumer-key>    Pocket consumer key [env: POCKET_CONSUMER_KEY=]

SUBCOMMANDS:
    add             Add
    archive         Archive
    auth            Authenticate
    config          Config
    delete          Delete
    favorite        Favorite
    get             Get
    help            Prints this message or the help of the given subcommand(s)
    readd           Readd
    tag             Tag
    tags-add        Add tags
    tags-clear      Clear tags
    tags-remove     Remove tags
    tags-replace    Replace tags
    unfavorite      Unfavorite
```

## Releases

### Create Release

To create a new release, the only manual part of the process is creating
and pushing a tag to GitHub. The following commands will create and push
a tag. Before running the commands, be sure to update `MAJOR`, `MINOR`,
and `PATCH` based on the current version and
[Semantic Version](https://semver.org/) guidelines.

**Create and Push Tag**
```bash
$ git checkout master
$ git pull
$ export TAG=v$(awk -F'"' '/version/ {print $2}' Cargo.toml)
$ git tag -a $TAG
# enter tag message
$ git push origin $TAG
```

After pushing a new tag, the
[Publish Workflow](.github/workflows/publish.yml) will create a
corresponding GitHub Release and attach artifacts for each supported
platform.

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