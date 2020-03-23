use pocket::*;
use chrono::{DateTime, Utc};
use structopt::StructOpt;
use std::io;
use std::io::ErrorKind;
use serde::Serialize;
use url::Url;
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
    offset: Option<usize>
}

pub fn handle(pocket: &impl PocketGet, opts: &GetOpts, output: &mut impl Output) {
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
            (Some(tag), false) => { f.tag(PocketGetTag::Tagged(tag)); },
            (None, true) => { f.tag(PocketGetTag::Untagged); },
            (None, false) => {},
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
            .map(|v| v.into_iter()
                .map(|i| i.into()).collect()
            )
    }.unwrap();
    output.write(items).unwrap();
}

#[derive(Serialize, Debug)]
pub struct Item {
    pub item_id: u64,
    #[serde(with = "url_serde")]
    pub given_url: Url,
    pub given_title: String,
    pub word_count: usize,
    pub excerpt: String,
    pub time_added: DateTime<Utc>,
    pub time_read: Option<DateTime<Utc>>,
    pub time_updated: DateTime<Utc>,
    pub time_favorited: Option<DateTime<Utc>>,
    pub favorite: bool,
    pub is_index: bool,
    pub is_article: bool,
    pub has_image: ItemHas,
    pub has_video: ItemHas,
    pub resolved_id: u64,
    pub resolved_title: String,
    #[serde(with = "url_serde")]
    pub resolved_url: Option<Url>,
    pub sort_id: u64,
    pub status: ItemStatus,
    pub tags: Option<Vec<Tag>>,
    pub images: Option<Vec<Image>>,
    pub videos: Option<Vec<Video>>,
    pub authors: Option<Vec<Author>>,
    pub lang: String,
    pub time_to_read: Option<u64>,
    pub domain_metadata: Option<DomainMetadata>,
    pub listen_duration_estimate: Option<u64>,
    pub image: Option<Image>,
    #[serde(with = "url_serde")]
    pub amp_url: Option<Url>,
    #[serde(with = "url_serde")]
    pub top_image_url: Option<Url>,
}

#[derive(Debug, Serialize)]
pub struct Image {
    pub item_id: u64,
    pub image_id: Option<u64>,
    #[serde(with = "url_serde")]
    pub src: Url,
    pub width: u16,
    pub height: u16,
    pub credit: Option<String>,
    pub caption: Option<String>,
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

impl From<ItemImage> for Image {
    fn from(i: ItemImage) -> Self {
        Image {
            item_id: i.item_id,
            image_id: None,
            src: i.src,
            width: i.width,
            height: i.height,
            credit: None,
            caption: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DomainMetadata {
    pub name: Option<String>,
    pub logo: String,
    pub greyscale_logo: String,
}

impl From<DomainMetaData> for DomainMetadata {
    fn from(d: DomainMetaData) -> Self {
        DomainMetadata {
            name: d.name,
            logo: d.logo,
            greyscale_logo: d.greyscale_logo,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Tag(String);

impl From<ItemTag> for Tag {
    fn from(t: ItemTag) -> Self {
        Tag(t.tag)
    }
}

#[derive(Debug, Serialize)]
pub struct Video {
    pub item_id: u64,
    pub video_id: u64,
    #[serde(with = "url_serde")]
    pub src: Url,
    pub width: u16,
    pub height: u16,
    pub length: Option<usize>,
    pub vid: String,
    #[serde(rename = "type")]
    pub vtype: u16,
}

impl From<ItemVideo> for Video {
    fn from(v: ItemVideo) -> Self {
        Video {
            item_id: v.item_id,
            video_id: v.video_id,
            src: v.src,
            width: v.width,
            height: v.height,
            length: v.length,
            vid: v.vid,
            vtype: v.vtype,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Author {
    pub item_id: u64,
    pub author_id: u64,
    pub name: String,
    pub url: String,
}

impl From<ItemAuthor> for Author {
    fn from(a: ItemAuthor) -> Self {
        Author {
            item_id: a.item_id,
            author_id: a.author_id,
            name: a.name,
            url: a.url,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ItemHas {
    No,
    Yes,
    Is,
}

impl From<PocketItemHas> for ItemHas {
    fn from(h: PocketItemHas) -> Self {
        match h {
            PocketItemHas::No => ItemHas::No,
            PocketItemHas::Yes => ItemHas::Yes,
            PocketItemHas::Is => ItemHas::Is,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ItemStatus {
    Normal,
    Archived,
    Deleted,
}

impl From<PocketItemStatus> for ItemStatus {
    fn from(s: PocketItemStatus) -> Self {
        match s {
            PocketItemStatus::Normal => ItemStatus::Normal,
            PocketItemStatus::Archived => ItemStatus::Archived,
            PocketItemStatus::Deleted => ItemStatus::Deleted,
        }
    }
}

impl From<PocketItem> for Item {
    fn from(p: PocketItem) -> Self {
        Item {
            item_id: p.item_id,
            given_url: p.given_url,
            given_title: p.given_title,
            word_count: p.word_count,
            excerpt: p.excerpt,
            time_added: p.time_added,
            time_read: p.time_read,
            time_updated: p.time_updated,
            time_favorited: p.time_favorited,
            favorite: p.favorite,
            is_index: p.is_index,
            is_article: p.is_article,
            has_image: p.has_image.into(),
            has_video: p.has_video.into(),
            resolved_id: p.resolved_id,
            resolved_title: p.resolved_title,
            resolved_url: p.resolved_url,
            sort_id: p.sort_id,
            status: p.status.into(),
            tags: p.tags.map(|v| v.into_iter().map(|t| t.into()).collect()),
            images: p.images.map(|v| v.into_iter().map(|i| i.into()).collect()),
            videos: p.videos.map(|v| v.into_iter().map(|v| v.into()).collect()),
            authors: p.authors.map(|v| v.into_iter().map(|a| a.into()).collect()),
            lang: p.lang,
            time_to_read: p.time_to_read,
            domain_metadata: p.domain_metadata.map(|d| d.into()),
            listen_duration_estimate: p.listen_duration_estimate,
            image: p.image.map(|i| i.into()),
            amp_url: p.amp_url,
            top_image_url: p.top_image_url
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
    use std::io;
    use crate::output::OutputError;
    use std::io::Write;
    use std::fmt::Debug;

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

    struct WriteLnOutput {
        output: Vec<u8>,
    }

    impl Output for WriteLnOutput {
        fn write(&mut self, value: impl Serialize + Debug) -> Result<(), OutputError> {
            writeln!(&mut self.output, "{:?}", value)
                .map_err(|_| OutputError {})
        }
    }

    struct OutputMock<W>
        where
            W: Fn() -> Result<(), OutputError>,
    {
        write_mock: W,
    }

    impl<W> Output for OutputMock<W>
        where
            W: Fn() -> Result<(), OutputError>,
    {
        fn write(&mut self, _value: impl Serialize + Debug) -> Result<(), OutputError> {
            (self.write_mock)()
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
            offset: None
        };
        let mut output = WriteLnOutput {
            output: Vec::new(),
        };

        handle(&pocket, &opts, &mut output);

        assert_eq!(format!("{:?}\n", &items).into_bytes(), output.output);
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
            offset: None
        };
        let mut output = WriteLnOutput {
            output: Vec::new(),
        };

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
            offset: None
        };
        let mut output = OutputMock {
            write_mock: || Err(OutputError {}),
        };

        handle(&pocket, &opts, &mut output);
    }
}

fn parse_get_state(s: &str) -> Result<PocketGetState, io::Error> {
    match s {
        "unread" => Ok(PocketGetState::Unread),
        "archive" => Ok(PocketGetState::Archive),
        "all" => Ok(PocketGetState::All),
        _ => Err(io::Error::new(ErrorKind::Other, format!("Invalid state: {}", s))),
    }
}

fn parse_get_content_type(s: &str) -> Result<PocketGetType, io::Error> {
    match s {
        "article" => Ok(PocketGetType::Article),
        "video" => Ok(PocketGetType::Video),
        "image" => Ok(PocketGetType::Image),
        _ => Err(io::Error::new(ErrorKind::Other, format!("Invalid content type: {}", s))),
    }
}

fn parse_get_detail_type(s: &str) -> Result<PocketGetDetail, io::Error> {
    match s {
        "simple" => Ok(PocketGetDetail::Simple),
        "complete" => Ok(PocketGetDetail::Complete),
        _ => Err(io::Error::new(ErrorKind::Other, format!("Invalid detail type: {}", s))),
    }
}

fn parse_get_sort(s: &str) -> Result<PocketGetSort, io::Error> {
    match s {
        "newest" => Ok(PocketGetSort::Newest),
        "oldest" => Ok(PocketGetSort::Oldest),
        "title" => Ok(PocketGetSort::Title),
        "site" => Ok(PocketGetSort::Site),
        _ => Err(io::Error::new(ErrorKind::Other, format!("Invalid sort: {}", s))),
    }
}