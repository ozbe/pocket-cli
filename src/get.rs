use chrono::{DateTime, Utc};
use pocket::*;
use std::io;
use std::io::{ErrorKind, Write};
use structopt::StructOpt;

use crate::models::{Image, Item};
use crate::output::Output;

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

pub fn handle<W: Write>(pocket: &impl PocketGet, opts: &GetOpts, output: &mut Output<W>) {
    let items: Vec<Item> = {
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

        pocket
            .get(&f)
            .map(|v| v.into_iter().map(|i| i.into()).collect())
    }
    .unwrap();
    output.write(&items).unwrap();
}

impl From<PocketItem> for Item {
    fn from(p: PocketItem) -> Self {
        Item {
            item_id: p.item_id,
            given_url: p.given_url,
            given_title: Some(p.given_title),
            word_count: p.word_count,
            excerpt: p.excerpt,
            time_added: Some(p.time_added),
            time_read: p.time_read,
            time_updated: Some(p.time_updated),
            time_favorited: p.time_favorited,
            favorite: Some(p.favorite),
            is_index: p.is_index,
            is_article: p.is_article,
            has_image: p.has_image.into(),
            has_video: p.has_video.into(),
            resolved_id: p.resolved_id,
            resolved_title: Some(p.resolved_title),
            resolved_url: p.resolved_url,
            sort_id: Some(p.sort_id),
            status: Some(p.status.into()),
            tags: p.tags.map(|v| v.into_iter().map(|t| t.into()).collect()),
            images: p.images.map(|v| v.into_iter().map(|i| i.into()).collect()),
            videos: p.videos.map(|v| v.into_iter().map(|v| v.into()).collect()),
            authors: p.authors.map(|v| v.into_iter().map(|a| a.into()).collect()),
            lang: Some(p.lang),
            time_to_read: p.time_to_read,
            domain_metadata: p.domain_metadata.map(|d| d.into()),
            listen_duration_estimate: p.listen_duration_estimate,
            image: p.image.map(|i| i.into()),
            amp_url: p.amp_url,
            top_image_url: p.top_image_url,
        }
    }
}

impl From<PocketImage> for Image {
    fn from(i: PocketImage) -> Self {
        Image {
            item_id: i.item_id,
            image_id: Some(i.image_id),
            src: i.src,
            width: i.width,
            height: i.height,
            credit: Some(i.credit),
            caption: Some(i.caption),
        }
    }
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
    use crate::output::OutputFormat;
    use std::io;
    use std::io::stdout;

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
        let writer = Vec::new();
        let mut output = Output::new(OutputFormat::Json, writer);

        handle(&pocket, &opts, &mut output);

        assert_eq!("[]", String::from_utf8_lossy(&output.into_vec()));
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
        let mut output = Output::new(OutputFormat::Json, stdout());

        handle(&pocket, &opts, &mut output);
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
        let writer = WriteMock {
            flush_mock: || Ok(()),
            write_mock: |_| Err(io::Error::new(io::ErrorKind::Other, "oh no")),
        };
        let mut output = Output::new(OutputFormat::Json, writer);

        handle(&pocket, &opts, &mut output);
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
