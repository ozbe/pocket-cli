extern crate pocket;
extern crate structopt;

use pocket::{Pocket, PocketGetRequest, PocketResult, PocketItem, PocketAddedItem};
use std::io;
use structopt::StructOpt;
use hyper::client::IntoUrl;

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
    let pocket = Pocket::auth(&opts.consumer_key);
    let pocket = pocket.request("rustapi:finishauth").unwrap();
    println!("Follow auth URL to provide access: {}", pocket.url());
    let _ = io::stdin().read_line(&mut String::new());
    let user = pocket.authorize().unwrap();
    println!("username: {}", user.username);
    println!("access token: {:?}", user.access_token);
}

fn add<T: IntoUrl>(pocket: &impl PocketAdd, url: T, _opts: &Opts, mut writer: impl std::io::Write) {
    let item = pocket.push(url).unwrap();
    writeln!(writer, "item: {:?}", item).unwrap();
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
        Commands::Add { ref url } => {
            let pocket = Pocket::new(
                &opts.consumer_key,
                opts.access_token.as_deref().unwrap(),
            );
            add(&pocket, url, &opts, &mut std::io::stdout())
        },
        Commands::Get => {
            let pocket = Pocket::new(
                &opts.consumer_key,
                opts.access_token.as_deref().unwrap(),
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

trait PocketAdd {
    fn add<T: IntoUrl>(&self, url: T, title: Option<&str>, tags: Option<&str>, tweet_id: Option<&str>) -> PocketResult<PocketAddedItem>;
    fn push<T: IntoUrl>(&self, url: T) -> PocketResult<PocketAddedItem>;
}

impl PocketAdd for Pocket {
    fn add<T: IntoUrl>(&self, url: T, title: Option<&str>, tags: Option<&str>, tweet_id: Option<&str>) -> PocketResult<PocketAddedItem> {
        self.add(url, title, tags, tweet_id)
    }

    fn push<T: IntoUrl>(&self, url: T) -> PocketResult<PocketAddedItem> {
        self.push(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pocket::{PocketItem, PocketResult, PocketGetRequest, PocketError, PocketAddedItem, PocketItemHas};
    use std::io;
    use hyper::Url;

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

    struct PocketAddMock<A, P>
        where
            A: Fn(Url, Option<&str>, Option<&str>, Option<&str>) -> PocketResult<PocketAddedItem>,
            P: Fn(Url) -> PocketResult<PocketAddedItem>,
    {
        add_mock: A,
        push_mock: P,
    }

    impl<A, P> PocketAdd for PocketAddMock<A, P>
        where
            A: Fn(Url, Option<&str>, Option<&str>, Option<&str>) -> PocketResult<PocketAddedItem>,
            P: Fn(Url) -> PocketResult<PocketAddedItem>,
    {
        fn add<T: IntoUrl>(&self, url: T, title: Option<&str>, tags: Option<&str>, tweet_id: Option<&str>) -> PocketResult<PocketAddedItem> {
            (self.add_mock)(url.into_url().unwrap(), title, tags, tweet_id)
        }

        fn push<T: IntoUrl>(&self, url: T) -> PocketResult<PocketAddedItem> {
            (self.push_mock)(url.into_url().unwrap())
        }
    }

    fn added_item(url: &Url) -> PocketAddedItem {
        PocketAddedItem {
            item_id: 0,
            normal_url: url.clone(),
            resolved_id: 0,
            extended_item_id: 0,
            resolved_url: url.clone(),
            domain_id: 0,
            origin_domain_id: 0,
            response_code: 0,
            mime_type: "".to_string(),
            content_length: 0,
            encoding: "".to_string(),
            date_resolved: "".to_string(),
            date_published: "".to_string(),
            title: "".to_string(),
            excerpt: "".to_string(),
            word_count: 0,
            login_required: false,
            has_image: PocketItemHas::No,
            has_video: PocketItemHas::No,
            is_index: false,
            is_article: false,
            used_fallback: false,
            lang: "".to_string(),
            authors: vec![],
            images: vec![],
            videos: vec![],
            given_url: url.clone(),
        }
    }

    #[test]
    fn add_writes_item() {
        let raw_url = "https://example.com";
        let pocket = PocketAddMock {
            add_mock: |_, _, _, _| Err(PocketError::Proto(0, "".to_string())),
            push_mock: |_| Ok(added_item(&raw_url.into_url().unwrap())),
        };
        let opts = Opts {
            consumer_key: "".to_string(),
            access_token: None,
            command: Commands::Get
        };
        let mut result = Vec::new();
        let url = "https://example.com".into_url().unwrap();
        let expected_item = added_item(&url);

        add(&pocket, url,  &opts, &mut result);

        assert_eq!(format!("item: {:?}\n", expected_item).into_bytes(), result);
    }

    #[test]
    #[should_panic]
    fn add_panics_when_pocket_error() {
        let raw_url = "https://example.com";
        let pocket = PocketAddMock {
            add_mock: |_, _, _, _| Ok(added_item(&raw_url.into_url().unwrap())),
            push_mock: |_| Err(PocketError::Proto(0, "".to_string())),
        };
        let opts = Opts {
            consumer_key: "".to_string(),
            access_token: None,
            command: Commands::Get
        };
        let mut writer = Vec::new();
        let url = "https://example.com".into_url().unwrap();

        add(&pocket, url,  &opts, &mut writer);
    }

    #[test]
    #[should_panic]
    fn add_panics_when_write_error() {
        let raw_url = "https://example.com";
        let pocket = PocketAddMock {
            add_mock: |_, _, _, _| Ok(added_item(&raw_url.into_url().unwrap())),
            push_mock: |_| Ok(added_item(&raw_url.into_url().unwrap())),
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
        let url = "https://example.com".into_url().unwrap();

        add(&pocket, url,  &opts, &mut writer);
    }
}