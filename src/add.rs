use pocket::*;
use structopt::StructOpt;
use url::Url;
use hyper::client::IntoUrl;

#[derive(Debug, StructOpt)]
pub struct AddOpts {
    url: Url,
    #[structopt(long)]
    title: Option<String>,
    #[structopt(long = "tag")]
    tags: Option<Vec<String>>,
    #[structopt(long)]
    tweet_id: Option<String>
}

pub fn handle(pocket: &impl PocketAdd, opts: &AddOpts, mut writer: impl std::io::Write) {
    let tags = opts.tags.as_ref()
        .map(|v| v.iter().map(|s| s.as_ref()).collect::<Vec<&str>>());

    let item = pocket.add(&PocketAddRequest {
        url: &opts.url,
        title: opts.title.as_deref(),
        tags: tags.as_ref().map(|v| v.as_slice()),
        tweet_id: opts.tweet_id.as_deref(),
    }).unwrap();
    writeln!(writer, "item: {:?}", item).unwrap();
}

pub trait PocketAdd {
    fn add(&self, request: &PocketAddRequest) -> PocketResult<PocketAddedItem>;
    fn push<T: IntoUrl>(&self, url: T) -> PocketResult<PocketAddedItem>;
}

impl PocketAdd for Pocket {
    fn add(&self, request: &PocketAddRequest) -> PocketResult<PocketAddedItem> {
        self.add(request)
    }

    fn push<T: IntoUrl>(&self, url: T) -> PocketResult<PocketAddedItem> {
        self.push(url)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use hyper::Url;

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

    struct PocketAddMock<A, P>
        where
            A: Fn(&PocketAddRequest) -> PocketResult<PocketAddedItem>,
            P: Fn(Url) -> PocketResult<PocketAddedItem>,
    {
        add_mock: A,
        push_mock: P,
    }

    impl<A, P> PocketAdd for PocketAddMock<A, P>
        where
            A: Fn(&PocketAddRequest) -> PocketResult<PocketAddedItem>,
            P: Fn(Url) -> PocketResult<PocketAddedItem>,
    {
        fn add(&self, request: &PocketAddRequest) -> PocketResult<PocketAddedItem> {
            (self.add_mock)(request)
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
            add_mock: |r|  Ok(added_item(r.url)),
            push_mock: |_| Err(PocketError::Proto(0, "".to_string())),
        };
        let opts = AddOpts {
            url: raw_url.into_url().unwrap(),
            title: None,
            tags: None,
            tweet_id: None
        };
        let mut result = Vec::new();
        let url = "https://example.com".into_url().unwrap();
        let expected_item = added_item(&url);

        handle(&pocket, &opts, &mut result);

        assert_eq!(format!("item: {:?}\n", expected_item).into_bytes(), result);
    }

    #[test]
    #[should_panic]
    fn add_panics_when_pocket_error() {
        let raw_url = "https://example.com";
        let pocket = PocketAddMock {
            add_mock: |_| Err(PocketError::Proto(0, "".to_string())),
            push_mock: |_| Ok(added_item(&raw_url.into_url().unwrap())),
        };
        let opts = AddOpts {
            url: raw_url.into_url().unwrap(),
            title: None,
            tags: None,
            tweet_id: None
        };
        let mut writer = Vec::new();

        handle(&pocket, &opts, &mut writer);
    }

    #[test]
    #[should_panic]
    fn add_panics_when_write_error() {
        let raw_url = "https://example.com";
        let pocket = PocketAddMock {
            add_mock: |r| Ok(added_item(r.url)),
            push_mock: |_| Ok(added_item(&raw_url.into_url().unwrap())),
        };
        let opts = AddOpts {
            url: raw_url.into_url().unwrap(),
            title: None,
            tags: None,
            tweet_id: None
        };
        let mut writer = WriteMock {
            flush_mock: || Ok(()),
            write_mock: |_| Err(io::Error::new(io::ErrorKind::Other, "oh no")),
        };

        handle(&pocket, &opts, &mut writer);
    }
}