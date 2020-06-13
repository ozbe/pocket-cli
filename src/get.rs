use chrono::{DateTime, Utc};
use pocket::*;
use std::io;
use std::io::ErrorKind;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct GetOpts {
    #[structopt(long)]
    search: Option<String>,
    #[structopt(long)]
    domain: Option<String>,
    #[structopt(long)]
    tag: Option<String>,
    #[structopt(long)]
    untagged: bool,
    #[structopt(long, parse(try_from_str = parse_get_state))]
    state: Option<PocketGetState>,
    #[structopt(long, parse(try_from_str = parse_get_content_type))]
    content_type: Option<PocketGetType>,
    #[structopt(long, parse(try_from_str = parse_get_detail_type))]
    detail_type: Option<PocketGetDetail>,
    #[structopt(long)]
    favorite: Option<bool>,
    #[structopt(long)]
    since: Option<DateTime<Utc>>,
    #[structopt(long, parse(try_from_str = parse_get_sort))]
    sort: Option<PocketGetSort>,
    #[structopt(long)]
    count: Option<usize>,
    #[structopt(long)]
    offset: Option<usize>,
}

pub fn handle(pocket: &impl PocketGet, opts: &GetOpts, mut writer: impl std::io::Write) {
    let items = {
        let mut f = pocket.filter();

        if let Some(search) = &opts.search {
            f.search(search);
        }

        // domain
        if let Some(domain) = &opts.domain {
            f.domain(domain);
        }

        // tag match
        match (&opts.tag, opts.untagged) {
            (Some(_), true) => panic!("Cannot set tag and untagged"),
            (Some(tag), false) => {
                f.tag(PocketGetTag::Tagged(tag));
            }
            (None, true) => {
                f.tag(PocketGetTag::Untagged);
            }
            (None, false) => {}
        }

        // state
        if let Some(state) = opts.state {
            f.state(state);
        }

        // content_type
        if let Some(content_type) = opts.content_type {
            f.content_type(content_type);
        }

        // detail_type
        if let Some(detail_type) = opts.detail_type {
            f.detail_type(detail_type);
        }

        // favorite
        if let Some(favorite) = opts.favorite {
            f.favorite(favorite);
        }

        // sort
        if let Some(sort) = opts.sort {
            f.sort(sort);
        }

        // offset
        if let Some(offset) = opts.offset {
            f.offset(offset);
        }

        // count
        if let Some(count) = opts.count {
            f.count(count);
        }

        pocket.get(&f)
    }
    .unwrap();
    writeln!(writer, "items: {:?}", items).unwrap();
}

pub trait PocketGet {
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
    use super::*;
    use std::io;

    struct PocketGetMock<'a, F, G>
    where
        F: Fn() -> PocketGetRequest<'a>,
        G: Fn(&PocketGetRequest) -> PocketResult<Vec<PocketItem>>,
    {
        filter_mock: F,
        get_mock: G,
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
        let opts = GetOpts {
            search: None,
            domain: None,
            tag: None,
            untagged: false,
            state: None,
            content_type: None,
            detail_type: None,
            favorite: None,
            since: None,
            sort: None,
            count: None,
            offset: None,
        };
        let mut result = Vec::new();

        handle(&pocket, &opts, &mut result);

        assert_eq!(format!("items: {:?}\n", &items).into_bytes(), result);
    }

    #[test]
    #[should_panic]
    fn get_panics_when_pocket_error() {
        let pocket = PocketGetMock {
            filter_mock: || PocketGetRequest::new(),
            get_mock: |_| Err(PocketError::Proto(1, "".to_string())),
        };
        let opts = GetOpts {
            search: None,
            domain: None,
            tag: None,
            untagged: false,
            state: None,
            content_type: None,
            detail_type: None,
            favorite: None,
            since: None,
            sort: None,
            count: None,
            offset: None,
        };
        let mut writer = Vec::new();

        handle(&pocket, &opts, &mut writer);
    }

    #[test]
    #[should_panic]
    fn get_panics_when_write_error() {
        let pocket = PocketGetMock {
            filter_mock: || PocketGetRequest::new(),
            get_mock: |_| Ok(vec![]),
        };
        let opts = GetOpts {
            search: None,
            domain: None,
            tag: None,
            untagged: false,
            state: None,
            content_type: None,
            detail_type: None,
            favorite: None,
            since: None,
            sort: None,
            count: None,
            offset: None,
        };
        let mut writer = WriteMock {
            flush_mock: || Ok(()),
            write_mock: |_| Err(io::Error::new(io::ErrorKind::Other, "oh no")),
        };

        handle(&pocket, &opts, &mut writer);
    }
}

fn parse_get_state(s: &str) -> Result<PocketGetState, io::Error> {
    match s {
        "unread" => Ok(PocketGetState::Unread),
        "archive" => Ok(PocketGetState::Archive),
        "all" => Ok(PocketGetState::All),
        _ => Err(io::Error::new(
            ErrorKind::Other,
            format!("Invalid state: {}", s),
        )),
    }
}

fn parse_get_content_type(s: &str) -> Result<PocketGetType, io::Error> {
    match s {
        "article" => Ok(PocketGetType::Article),
        "video" => Ok(PocketGetType::Video),
        "image" => Ok(PocketGetType::Image),
        _ => Err(io::Error::new(
            ErrorKind::Other,
            format!("Invalid content type: {}", s),
        )),
    }
}

fn parse_get_detail_type(s: &str) -> Result<PocketGetDetail, io::Error> {
    match s {
        "simple" => Ok(PocketGetDetail::Simple),
        "complete" => Ok(PocketGetDetail::Complete),
        _ => Err(io::Error::new(
            ErrorKind::Other,
            format!("Invalid detail type: {}", s),
        )),
    }
}

fn parse_get_sort(s: &str) -> Result<PocketGetSort, io::Error> {
    match s {
        "newest" => Ok(PocketGetSort::Newest),
        "oldest" => Ok(PocketGetSort::Oldest),
        "title" => Ok(PocketGetSort::Title),
        "site" => Ok(PocketGetSort::Site),
        _ => Err(io::Error::new(
            ErrorKind::Other,
            format!("Invalid sort: {}", s),
        )),
    }
}
