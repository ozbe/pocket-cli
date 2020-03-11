extern crate pocket;
extern crate structopt;

use pocket::Pocket;
use std::io;
use structopt::StructOpt;

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

fn auth(cmd: Auth) {
    match cmd {
        Auth::Login => login(),
    }
}

fn login() {
    let mut pocket = Pocket::new(&std::env::var("POCKET_CONSUMER_KEY").unwrap(), None);
    let url = pocket.get_auth_url().unwrap();
    println!("Follow auth URL to provide access: {}", url);
    let _ = io::stdin().read_line(&mut String::new());
    let username = pocket.authorize().unwrap();
    println!("username: {}", username);
    println!("access token: {:?}", pocket.access_token());
}

fn add(url: &String) {
    let mut pocket = Pocket::new(
        &std::env::var("POCKET_CONSUMER_KEY").unwrap(),
        Some(&std::env::var("POCKET_ACCESS_TOKEN").unwrap()),
    );
    let item = pocket.push(url).unwrap();
    println!("item: {:?}", item);
}

fn get() {
    let pocket = Pocket::new(
        &std::env::var("POCKET_CONSUMER_KEY").unwrap(),
        Some(&std::env::var("POCKET_ACCESS_TOKEN").unwrap()),
    );
    let items = {
        let f = pocket.filter();
        pocket.get(&f)
    };
    println!("items: {:?}", items);
}

fn main() {
    match Commands::from_args() {
        Commands::Auth(sc) => auth(sc),
        Commands::Add { url } => add(&url),
        Commands::Get => get()
    }
}