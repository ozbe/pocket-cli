# WIP: Pocket CLI

## --help

Possible help output with inline notes
```
USAGE:  
    pocket [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints this message
    -V, --version    Prints version information
    -o, --output     Output format

OPTIONS:
    -c, --config <FILE>    Sets a custom config file

SUBCOMMANDS:  
    help            Prints this message  
    config set      key value - auth/consumer_key, auth/access_token, auth/username, get/count, get/offset
    config get      key
    config view
    auth            describe
    auth            login
    get             get - all optional
    add             send - url, others optional
    archive         send - item_id
    readd           send - item_id
    favorite        send - item_id
    unfavorite      send - item_id
    delete          send - item_id
    tags-add        send - item_id, tags
    tags-remove     send - item_id, tags
    tags-replace    send - item_id, tags
    tags-clear      send - item_id
    tag rename      send - old_tag new_tag
    tag delete      send - tag
```

## Possible examples

###  Add URL
`$ pocket add https://example.com`

###  Remove tag
`$ pocket tags remove tag1`