use pocket::*;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use structopt::StructOpt;
use url::Url;

#[derive(Debug, StructOpt)]
pub enum Auth {
    Login,
}

pub fn handle(cmd: &Auth, consumer_key: &str, writer: impl std::io::Write) {
    match cmd {
        Auth::Login => {
            let server = TcpAuthServer::new();
            let pocket = PocketAuthentication::new(&consumer_key, server.addr());
            login(pocket, server, writer)
        }
    }
}

fn login(pocket: impl PocketAuth, server: impl AuthServer, mut writer: impl std::io::Write) {
    let code = pocket.request(None).unwrap();
    writeln!(
        writer,
        "Follow auth URL to provide access: {}",
        pocket.authorize_url(&code)
    )
    .unwrap();

    server.wait_for_response();

    let user = pocket.authorize(&code, None).unwrap();
    writeln!(writer, "username: {}", user.username).unwrap();
    writeln!(writer, "access token: {:?}", user.access_token).unwrap();
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
        let mut result = Vec::new();

        login(pocket, server, &mut result);

        assert_eq!(
            format!(
                "Follow auth URL to provide access: {}\nusername: {}\naccess token: {:?}\n",
                url, username, access_token
            ),
            to_string(&result)
        )
    }

    fn to_string(bytes: &[u8]) -> String {
        std::str::from_utf8(&bytes).unwrap().to_string()
    }

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
