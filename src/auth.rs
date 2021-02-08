use crate::output::Output;
use pocket::*;
use serde::Serialize;
use std::error::Error;
use std::io::prelude::*;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use structopt::StructOpt;
use url::Url;

#[derive(Debug, StructOpt)]
pub enum Auth {
    // Login
    Login {
        #[structopt(long)]
        /// Save access token to config
        save: bool,
    },
}

pub fn handle<W: Write>(cmd: &Auth, consumer_key: &str, output: &mut Output<W>) {
    match cmd {
        Auth::Login { save } => {
            let server = TcpAuthServer::new();
            let pocket = PocketAuthentication::new(&consumer_key, server.addr());
            login(pocket, *save, server, &open_browser, output)
        }
    }
}

fn open_browser(url: &Url) -> Result<(), Box<dyn Error>> {
    webbrowser::open(url.as_str())
        .map(|_| ())
        .map_err(|e| e.into())
}

fn login<W: Write>(
    pocket: impl PocketAuth,
    save: bool,
    server: impl AuthServer,
    open_browser: &dyn Fn(&Url) -> Result<(), Box<dyn Error>>,
    output: &mut Output<W>,
) {
    let code = pocket.request(None).unwrap();
    let authorize_url = pocket.authorize_url(&code);
    open_browser(&authorize_url).unwrap();
    server.wait_for_response();

    let user: User = pocket.authorize(&code, None).unwrap().into();

    if save {
        let mut cfg = crate::config::load();
        cfg.access_token = Some(user.access_token.clone());
        crate::config::store(cfg);
    }

    output.write(user).unwrap();
}

#[derive(Serialize)]
struct User {
    access_token: String,
    username: String,
}

impl From<PocketUser> for User {
    fn from(u: PocketUser) -> Self {
        User {
            access_token: u.access_token,
            username: u.username,
        }
    }
}

trait PocketAuth {
    fn request(&self, state: Option<&str>) -> PocketResult<String>;
    fn authorize_url(&self, code: &str) -> Url;
    fn authorize(&self, code: &str, state: Option<&str>) -> PocketResult<PocketUser>;
}

impl PocketAuth for PocketAuthentication {
    fn request(&self, state: Option<&str>) -> PocketResult<String> {
        self.request(state)
    }

    fn authorize_url(&self, code: &str) -> Url {
        self.authorize_url(code)
    }

    fn authorize(&self, code: &str, state: Option<&str>) -> PocketResult<PocketUser> {
        self.authorize(code, state)
    }
}

trait AuthServer {
    fn wait_for_response(&self);
}

impl AuthServer for TcpAuthServer {
    fn wait_for_response(&self) {
        self.wait_for_response();
    }
}

struct TcpAuthServer {
    listener: TcpListener,
    addr: String,
}

impl TcpAuthServer {
    fn new() -> TcpAuthServer {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = format!("http://{}", listener.local_addr().unwrap());

        TcpAuthServer { listener, addr }
    }

    fn wait_for_response(&self) {
        for stream in self.listener.incoming().take(1) {
            let stream = stream.unwrap();
            TcpAuthServer::handle_connection(stream)
        }
    }
    #[allow(clippy::unused_io_amount)]
    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", AUTH_SUCCESS_RESPONSE_BODY);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn addr(&self) -> &str {
        &self.addr
    }
}

const AUTH_SUCCESS_RESPONSE_BODY: &str = r#"
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::OutputFormat;
    use pocket::{PocketResult, PocketUser};
    use std::io;

    #[test]
    fn login_writes_user() {
        let username = "username";
        let access_token = "access_token";
        let url = "http://example.com/-";
        let code = "code";
        let pocket = PocketAuthMock {
            request_mock: |_s| Ok(code.to_string()),
            authorize_url: |_c| Url::parse(url).unwrap(),
            authorize_mock: |_c, _s| {
                Ok(PocketUser {
                    consumer_key: "".to_string(),
                    access_token: access_token.to_string(),
                    username: username.to_string(),
                })
            },
        };
        let server = AuthServerMock {
            wait_for_response_mock: || {},
        };
        let writer = Vec::new();
        let mut output = Output::new(OutputFormat::Json, writer);
        let expected_user = User {
            username: username.to_string(),
            access_token: access_token.to_string(),
        };

        login(pocket, false, server, &noop_browser, &mut output);

        assert_eq!(
            serde_json::to_string(&expected_user).unwrap(),
            String::from_utf8_lossy(&output.into_vec())
        )
    }

    fn noop_browser(_url: &Url) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    // fn to_string(bytes: &[u8]) -> String {
    //     std::str::from_utf8(&bytes).unwrap().to_string()
    // }

    struct PocketAuthMock<R, U, A>
    where
        R: Fn(Option<&str>) -> PocketResult<String>,
        U: Fn(&str) -> Url,
        A: Fn(&str, Option<&str>) -> PocketResult<PocketUser>,
    {
        request_mock: R,
        authorize_url: U,
        authorize_mock: A,
    }

    impl<R, U, A> PocketAuth for PocketAuthMock<R, U, A>
    where
        R: Fn(Option<&str>) -> PocketResult<String>,
        U: Fn(&str) -> Url,
        A: Fn(&str, Option<&str>) -> PocketResult<PocketUser>,
    {
        fn request(&self, state: Option<&str>) -> PocketResult<String> {
            (self.request_mock)(state)
        }

        fn authorize_url(&self, code: &str) -> Url {
            (self.authorize_url)(code)
        }

        fn authorize(&self, code: &str, state: Option<&str>) -> PocketResult<PocketUser> {
            (self.authorize_mock)(code, state)
        }
    }

    struct AuthServerMock<W>
    where
        W: Fn(),
    {
        wait_for_response_mock: W,
    }

    impl<W> AuthServer for AuthServerMock<W>
    where
        W: Fn(),
    {
        fn wait_for_response(&self) {
            (self.wait_for_response_mock)()
        }
    }

    struct WriteMock<W, F>
    where
        W: Fn(&[u8]) -> io::Result<usize>,
        F: Fn() -> io::Result<()>,
    {
        write_mock: W,
        flush_mock: F,
    }

    impl<W, F> io::Write for WriteMock<W, F>
    where
        W: Fn(&[u8]) -> io::Result<usize>,
        F: Fn() -> io::Result<()>,
    {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            (self.write_mock)(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            (self.flush_mock)()
        }
    }
}
