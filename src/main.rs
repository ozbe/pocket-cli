extern crate pocket;
extern crate structopt;

use pocket::*;
use std::{io, fs};
use std::io::prelude::*;
use structopt::StructOpt;
use hyper::client::IntoUrl;
use chrono::{DateTime, Utc};
use std::io::ErrorKind;
use hyper::Url;
use std::net::{TcpListener, TcpStream};

mod get;
mod add;

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
#[structopt(rename_all = "kebab-case")]
enum Commands {
    Auth(Auth),
    Add {
        #[structopt(flatten)]
        opts: add::AddOpts
    },
    Get {
        #[structopt(flatten)]
        opts: get::GetOpts
    },
}

#[derive(Debug, StructOpt)]
enum Auth {
    Login,
}

fn auth(cmd: &Auth, opts: &Opts, reader: impl std::io::BufRead, writer: impl std::io::Write) {
    match cmd {
        Auth::Login => login(opts, reader, writer),
    }
}

fn auth_server() {
    // TODO - rand port and duplicate address
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming().take(1) {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
}

const AUTH_SUCCESS_RESPONSE_BODY: &'static str = r#"
    <!DOCTYPE html>
    <html lang="en">
        <head>
            <meta charset="utf-8">
            <title>Pocket CLI</title>
        </head>
        <body>
            <h1>Success!</h1>
            <p>You have successfully authorized Pocket CLI.</p>
            <p>Close this window and return to Pocket CLI in your terminal.</p>
        </body>
    </html>
"#;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", AUTH_SUCCESS_RESPONSE_BODY);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn login(opts: &Opts, mut reader: impl std::io::BufRead, mut writer: impl std::io::Write) {
    let auth = PocketAuthentication::new(&opts.consumer_key, "http://127.0.0.1:7878");
    let code = auth.request(None).unwrap();
    writeln!(writer, "Follow auth URL to provide access: {}", auth.authorize_url(&code)).unwrap();

    auth_server();

    let user = auth.authorize(&code, None).unwrap();
    writeln!(writer, "username: {}", user.username).unwrap();
    writeln!(writer, "access token: {:?}", user.access_token).unwrap();
}

fn main() {
    let opts = Opts::from_args();
    let pocket = || {
        Pocket::new(
            &opts.consumer_key,
            &opts.access_token.as_deref().unwrap(),
        )
    };
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut writer = std::io::stdout();

    match opts.command {
        Commands::Auth(ref sc) => auth(sc, &opts, &mut reader, &mut writer),
        Commands::Add { opts: ref add_opts } => {
            add::handle(&pocket(), add_opts, &mut writer)
        },
        Commands::Get { opts: ref get_opts } => {
            get::handle(&pocket(), get_opts, &mut writer)
        }
    }
}