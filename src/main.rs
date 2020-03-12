extern crate pocket;
extern crate structopt;

use pocket::{Pocket, PocketGetRequest, PocketResult, PocketItem};
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

fn get(pocket: &impl PocketGet, _opts: &Opts, mut writer: impl std::io::Write) {
    let items = {
        let f = pocket.filter();
        pocket.get(&f)
    }.unwrap();
    writeln!(writer, "items: {:?}", items).unwrap();
}

fn main() {
    let opts = Opts::from_args();

    match opts.command {
        Commands::Auth(ref sc) => auth(sc, &opts),
        Commands::Add { ref url } => add(&url, &opts),
        Commands::Get => {
            let pocket = Pocket::new(
                &opts.consumer_key,
                opts.access_token.as_deref(),
            );
            get(&pocket, &opts, &mut std::io::stdout())
        }
    }
}

trait PocketGet {
    fn filter(&self) -> PocketGetRequest;
    fn get(&self, request: &PocketGetRequest) -> PocketResult<Vec<PocketItem>>;
}

impl PocketGet for Pocket {
    fn filter(&self) -> PocketGetRequest {
        self.filter()
    }

    fn get(&self, request: &PocketGetRequest) -> PocketResult<Vec<PocketItem>> {
        self.get(request)
    }
}

#[cfg(test)]
mod tests {
    use super::{Commands, PocketGet, get, Opts};
    use pocket::{PocketItem, PocketResult, PocketGetRequest, PocketError};
    use std::io;

    struct PocketGetMock<'a, F, G>
        where
            F: Fn() -> PocketGetRequest<'a>,
            G: Fn(&PocketGetRequest) -> PocketResult<Vec<PocketItem>>,
    {
        filter_mock: F,
        get_mock: G
    }

    impl<'a, F, G> PocketGet for PocketGetMock<'a, F, G>
        where
            F: Fn() -> PocketGetRequest<'a>,
            G: Fn(&PocketGetRequest) -> PocketResult<Vec<PocketItem>>,
    {
        fn filter(&self) -> PocketGetRequest {
            (self.filter_mock)()
        }

        fn get(&self, request: &PocketGetRequest) -> PocketResult<Vec<PocketItem>> {
            (self.get_mock)(request)
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

    #[test]
    fn get_writes_items() {
        let items: Vec<PocketItem> = vec![];
        let pocket = PocketGetMock {
            filter_mock: || PocketGetRequest::new(),
            get_mock: |_| Ok(vec![]),
        };
        let opts = Opts {
            consumer_key: "".to_string(),
            access_token: None,
            command: Commands::Get
        };
        let mut result = Vec::new();

        get(&pocket, &opts, &mut result);

        assert_eq!(format!("items: {:?}\n", &items).into_bytes(), result);
    }

    #[test]
    #[should_panic]
    fn get_panics_when_pocket_error() {
        let pocket = PocketGetMock {
            filter_mock: || PocketGetRequest::new(),
            get_mock: |_| Err(PocketError::Proto(1, "".to_string())),
        };
        let opts = Opts {
            consumer_key: "".to_string(),
            access_token: None,
            command: Commands::Get
        };
        let mut writer = Vec::new();

        get(&pocket, &opts, &mut writer);
    }

    #[test]
    #[should_panic]
    fn get_panics_when_write_error() {
        let pocket = PocketGetMock {
            filter_mock: || PocketGetRequest::new(),
            get_mock: |_| Ok(vec![]),
        };
        let opts = Opts {
            consumer_key: "".to_string(),
            access_token: None,
            command: Commands::Get
        };
        let mut writer = WriteMock {
            flush_mock: || Ok(()),
            write_mock: |_| Err(io::Error::new(io::ErrorKind::Other, "oh no")),
        };

        get(&pocket, &opts, &mut writer);
    }
}