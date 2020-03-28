extern crate pocket;
extern crate structopt;

use pocket::*;
use structopt::StructOpt;
use crate::output::Output;

mod get;
mod add;
mod auth;
mod output;
mod models;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(long, env = "POCKET_CONSUMER_KEY")]
    consumer_key: String,
    #[structopt(long, env = "POCKET_ACCESS_TOKEN")]
    access_token: Option<String>,
    #[structopt(subcommand)]
    command: Commands,
    #[structopt(default_value, long, short)]
    output: output::OutputFormat,
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Commands {
    Auth(auth::Auth),
    Add {
        #[structopt(flatten)]
        opts: add::AddOpts
    },
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
            let mut output = Output::new(opts.output, writer);
            get::handle(&pocket(), get_opts, &mut output)
        }
    }
}