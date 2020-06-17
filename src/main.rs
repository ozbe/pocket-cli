#[macro_use]
extern crate convey;

use convey::{human, json};
use pocket::*;
use std::fmt;
use structopt::StructOpt;

mod add;
mod auth;
mod config;
mod get;
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
    #[structopt(default_value, long)]
    output: Output,
    /// Subcommand
    #[structopt(subcommand)]
    command: Commands,
}

#[derive(Debug)]
enum Output {
    Text,
    Json,
}

impl std::str::FromStr for Output {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(Output::Text),
            "json" => Ok(Output::Json),
            _ => Err(format!("Unexpected output: {}", s)),
        }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = match self {
            Output::Text => "text",
            Output::Json => "json",
        }
        .to_string();

        write!(f, "{}", display)
    }
}

impl Default for Output {
    fn default() -> Self {
        Output::Text
    }
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
        output,
        command,
    } = Opts::from_args();
    let cfg = config::load();
    let consumer_key = opt_consumer_key
        .or(cfg.consumer_key)
        .expect("Consumer key missing.");
    let access_token = opt_access_token.or(cfg.access_token);
    let pocket = || Pocket::new(&consumer_key, &access_token.expect("Access token missing."));
    let mut writer = std::io::stdout();
    let out = match output {
        Output::Text => convey::new().add_target(human::stdout().unwrap()).unwrap(),
        Output::Json => convey::new().add_target(json::stdout().unwrap()).unwrap(),
    };

    match command {
        Commands::Add { opts: ref add_opts } => add::handle(&pocket(), add_opts, out),
        Commands::Archive { ref opts } => send::archive::handle(&pocket(), opts, &mut writer),
        Commands::Auth(ref sc) => auth::handle(sc, &consumer_key, &mut writer),
        Commands::Config(ref opts) => config::handle(opts, &mut writer),
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
