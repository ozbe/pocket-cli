use pocket::*;
use structopt::StructOpt;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;


#[derive(Debug, StructOpt)]
pub enum Auth {
    Login,
}

pub fn handle(cmd: &Auth, consumer_key: &str, writer: impl std::io::Write) {
    match cmd {
        Auth::Login => login(consumer_key, writer),
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

fn login(consumer_key: &str, mut writer: impl std::io::Write) {
    let auth = PocketAuthentication::new(&consumer_key, "http://127.0.0.1:7878");
    let code = auth.request(None).unwrap();
    writeln!(writer, "Follow auth URL to provide access: {}", auth.authorize_url(&code)).unwrap();

    auth_server();

    let user = auth.authorize(&code, None).unwrap();
    writeln!(writer, "username: {}", user.username).unwrap();
    writeln!(writer, "access token: {:?}", user.access_token).unwrap();
}