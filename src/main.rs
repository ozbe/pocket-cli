extern crate pocket;
extern crate structopt;

use pocket::*;
use structopt::StructOpt;

mod get;
mod add;
mod auth;

#[derive(Debug, StructOpt)]
/// Interact with the Pocket API.
struct Opts {
    /// Pocket consumer key
    #[structopt(long, env = "POCKET_CONSUMER_KEY")]
    consumer_key: String,
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
        opts: add::AddOpts
    },
    /// Get
    Get {
        #[structopt(flatten)]
        opts: get::GetOpts
    },
}

fn main() {
    let opts = Opts::from_args();
    let pocket = || {
        Pocket::new(
            &opts.consumer_key,
            &opts.access_token.as_deref().unwrap(),
        )
    };
    let mut writer = std::io::stdout();

    match opts.command {
        Commands::Auth(ref sc) => auth::handle(sc, &opts.consumer_key, &mut writer),
        Commands::Add { opts: ref add_opts } => {
            add::handle(&pocket(), add_opts, &mut writer)
        },
        Commands::Get { opts: ref get_opts } => {
            get::handle(&pocket(), get_opts, &mut writer)
        }
    }
}