extern crate pocket;
extern crate structopt;

use pocket::Pocket;
use std::io;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(long, env = "POCKET_CONSUMER_KEY")]
    consumer_key: String,
    #[structopt(long, env = "POCKET_ACCESS_TOKEN")]
    access_token: Option<String>,
    #[structopt(subcommand)]
    command: Commands,
}

#[derive(Debug, StructOpt)]
enum Commands {
    Auth(Auth),
    Add { url: String },
    Get,
}

#[derive(Debug, StructOpt)]
enum Auth {
    Login,
}

fn auth(cmd: &Auth, opts: &Opts) {
    match cmd {
        Auth::Login => login(opts),
    }
}

fn login(opts: &Opts) {
    let mut pocket = Pocket::new(&opts.consumer_key, None);
    let url = pocket.get_auth_url().unwrap();
    println!("Follow auth URL to provide access: {}", url);
    let _ = io::stdin().read_line(&mut String::new());
    let username = pocket.authorize().unwrap();
    println!("username: {}", username);
    println!("access token: {:?}", pocket.access_token());
}

fn add(url: &String, opts: &Opts) {
    let mut pocket = Pocket::new(
        &opts.consumer_key,
        opts.access_token.as_deref(),
    );
    let item = pocket.push(url).unwrap();
    println!("item: {:?}", item);
}

fn get(opts: &Opts) {
    let pocket = Pocket::new(
        &opts.consumer_key,
        opts.access_token.as_deref(),
    );
    let items = {
        let f = pocket.filter();
        pocket.get(&f)
    };
    println!("items: {:?}", items);
}

fn main() {
    let opts = Opts::from_args();

    match opts.command {
        Commands::Auth(ref sc) => auth(sc,&opts),
        Commands::Add { ref url } => add(&url, &opts),
        Commands::Get => get(&opts)
    }
}