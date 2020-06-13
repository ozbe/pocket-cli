extern crate pocket;
extern crate structopt;

use pocket::*;
use structopt::StructOpt;
use serde::{Deserialize, Serialize};

mod add;
mod auth;
mod get;
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
}

#[derive(Serialize, Deserialize)]
struct Config {
    consumer_key: Option<String>,
    access_token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            consumer_key: None,
            access_token: None,
        }
    }
}

fn main() {
    let Opts { consumer_key: opt_consumer_key, access_token: opt_access_token, command } = Opts::from_args();
    let cfg: Config = confy::load(env!("CARGO_PKG_NAME")).unwrap();
    let consumer_key= opt_consumer_key.or(cfg.consumer_key).expect("Consumer key missing.");
    let access_token = opt_access_token.or(cfg.access_token);
    let pocket = || Pocket::new(
        &consumer_key,
        &access_token.expect("Access token missing."),
    );
    let mut writer = std::io::stdout();

    match command {
        Commands::Add { opts: ref add_opts } => add::handle(&pocket(), add_opts, &mut writer),
        Commands::Archive { ref opts } => send::archive::handle(&pocket(), opts, &mut writer),
        Commands::Auth(ref sc) => auth::handle(sc, &consumer_key, &mut writer),
        Commands::Delete { ref opts } => send::delete::handle(&pocket(), opts, &mut writer),
        Commands::Favorite { ref opts } => send::favorite::handle(&pocket(), opts, &mut writer),
        Commands::Get { opts: ref get_opts } => get::handle(&pocket(), get_opts, &mut writer),
        Commands::Readd { ref opts } => send::readd::handle(&pocket(), opts, &mut writer),
        Commands::Tag(ref tag) => tag::handle(&pocket(), tag, &mut writer),
        Commands::TagsAdd { ref opts } => tags::tags_add::handle(&pocket(), opts, &mut writer),
        Commands::TagsClear { ref opts } => send::tags_clear::handle(&pocket(), opts, &mut writer),
        Commands::TagsRemove { ref opts } => {
            tags::tags_remove::handle(&pocket(), opts, &mut writer)
        }
        Commands::TagsReplace { ref opts } => {
            tags::tags_replace::handle(&pocket(), opts, &mut writer)
        }
        Commands::Unfavorite { ref opts } => send::unfavorite::handle(&pocket(), opts, &mut writer),
    }
}
