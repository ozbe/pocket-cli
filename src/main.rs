extern crate pocket;
extern crate structopt;

use crate::output::Output;
use pocket::*;
use structopt::StructOpt;

mod add;
mod auth;
mod config;
mod get;
mod models;
mod output;
mod send;
mod tag;
mod tags;

#[derive(Debug, StructOpt)]
/// Interact with the Pocket API.
struct Opts {
    /// Pocket consumer key
    #[structopt(long, env = "POCKET_CONSUMER_KEY")]
    consumer_key: Option<String>,
    /// Pocket access token
    #[structopt(long, env = "POCKET_ACCESS_TOKEN")]
    access_token: Option<String>,
    /// Subcommand
    #[structopt(subcommand)]
    command: Commands,
    #[structopt(default_value, long, short)]
    output: output::OutputFormat,
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Commands {
    /// Authenticate
    Auth(auth::Auth),
    /// Add
    Add {
        #[structopt(flatten)]
        opts: add::AddOpts,
    },
    /// Get
    Get {
        #[structopt(flatten)]
        opts: get::GetOpts,
    },
    /// Archive
    Archive {
        #[structopt(flatten)]
        opts: send::SendItemOpts,
    },
    /// Readd
    Readd {
        #[structopt(flatten)]
        opts: send::SendItemOpts,
    },
    /// Favorite
    Favorite {
        #[structopt(flatten)]
        opts: send::SendItemOpts,
    },
    /// Unfavorite
    Unfavorite {
        #[structopt(flatten)]
        opts: send::SendItemOpts,
    },
    /// Delete
    Delete {
        #[structopt(flatten)]
        opts: send::SendItemOpts,
    },
    /// Clear tags
    TagsClear {
        #[structopt(flatten)]
        opts: send::SendItemOpts,
    },
    /// Add tags
    TagsAdd {
        #[structopt(flatten)]
        opts: tags::TagsOpts,
    },
    /// Remove tags
    TagsRemove {
        #[structopt(flatten)]
        opts: tags::TagsOpts,
    },
    /// Replace tags
    TagsReplace {
        #[structopt(flatten)]
        opts: tags::TagsOpts,
    },
    /// Tag
    Tag(tag::Tag),
    /// Config
    Config(config::ConfigOpts),
}

fn main() {
    let Opts {
        consumer_key: opt_consumer_key,
        access_token: opt_access_token,
        command,
        output,
    } = Opts::from_args();
    let config::Config {
        consumer_key: cfg_consumer_key,
        access_token: cfg_access_token,
    } = config::load();
    let consumer_key = || {
        opt_consumer_key
            .or(cfg_consumer_key)
            .expect("Consumer key missing.")
    };
    let access_token = opt_access_token.or(cfg_access_token);
    let pocket =
        |consumer_key| Pocket::new(consumer_key, &access_token.expect("Access token missing."));
    let writer = std::io::stdout();
    let mut output = Output::new(output, writer);

    match command {
        Commands::Add { opts: ref add_opts } => {
            add::handle(&pocket(&consumer_key()), add_opts, &mut output)
        }
        Commands::Archive { ref opts } => {
            send::archive::handle(&pocket(&consumer_key()), opts, &mut output)
        }
        Commands::Auth(ref sc) => auth::handle(sc, &consumer_key(), &mut output),
        Commands::Config(ref opts) => config::handle(opts, &mut output),
        Commands::Delete { ref opts } => {
            send::delete::handle(&pocket(&consumer_key()), opts, &mut output)
        }
        Commands::Favorite { ref opts } => {
            send::favorite::handle(&pocket(&consumer_key()), opts, &mut output)
        }
        Commands::Get { opts: ref get_opts } => {
            get::handle(&pocket(&consumer_key()), get_opts, &mut output)
        }
        Commands::Readd { ref opts } => {
            send::readd::handle(&pocket(&consumer_key()), opts, &mut output)
        }
        Commands::Tag(ref tag) => tag::handle(&pocket(&consumer_key()), tag, &mut output),
        Commands::TagsAdd { ref opts } => {
            tags::tags_add::handle(&pocket(&consumer_key()), opts, &mut output)
        }
        Commands::TagsClear { ref opts } => {
            send::tags_clear::handle(&pocket(&consumer_key()), opts, &mut output)
        }
        Commands::TagsRemove { ref opts } => {
            tags::tags_remove::handle(&pocket(&consumer_key()), opts, &mut output)
        }
        Commands::TagsReplace { ref opts } => {
            tags::tags_replace::handle(&pocket(&consumer_key()), opts, &mut output)
        }
        Commands::Unfavorite { ref opts } => {
            send::unfavorite::handle(&pocket(&consumer_key()), opts, &mut output)
        }
    }
}
