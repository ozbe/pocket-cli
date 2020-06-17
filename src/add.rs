use crate::output::Item;
use convey::Output;
use hyper::client::IntoUrl;
use pocket::*;
use structopt::StructOpt;
use url::Url;

#[derive(Debug, StructOpt)]
pub struct AddOpts {
    url: Url,
    #[structopt(long)]
    title: Option<String>,
    #[structopt(long = "tag")]
    tags: Option<Vec<String>>,
    #[structopt(long)]
    tweet_id: Option<String>,
}

pub fn handle(pocket: &impl PocketAdd, opts: &AddOpts, out: Output) {
    let tags = opts
        .tags
        .as_ref()
        .map(|v| v.iter().map(|s| s.as_ref()).collect::<Vec<&str>>());

    let item: Item = pocket
        .add(&PocketAddRequest {
            url: &opts.url,
            title: opts.title.as_deref(),
            tags: tags.as_deref(),
            tweet_id: opts.tweet_id.as_deref(),
        })
        .unwrap()
        .into();
    out.print(item).unwrap();
}

impl From<PocketAddedItem> for Item {
    fn from(p: PocketAddedItem) -> Self {
        Item {
            item_id: p.item_id,
            given_url: p.given_url,
            given_title: None,
            word_count: p.word_count,
            excerpt: p.excerpt,
            time_added: None,
            time_read: None,
            time_updated: None,
            time_favorited: None,
            favorite: None,
            is_index: p.is_index,
            is_article: p.is_article,
            has_image: p.has_image.into(),
            has_video: p.has_video.into(),
            resolved_id: p.resolved_id,
            resolved_title: None,
            resolved_url: p.resolved_url,
            sort_id: None,
            status: None,
            tags: None,
            images: p.images.map(|v| v.into_iter().map(|i| i.into()).collect()),
            videos: p.videos.map(|v| v.into_iter().map(|i| i.into()).collect()),
            authors: p.authors.map(|v| v.into_iter().map(|i| i.into()).collect()),
            lang: p.lang,
            time_to_read: None,
            domain_metadata: None,
            listen_duration_estimate: None,
            image: None,
            amp_url: None,
            top_image_url: None,
        }
    }
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
    use convey::json;
    use hyper::Url;

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
            resolved_url: Some(url.clone()),
            domain_id: 0,
            origin_domain_id: 0,
            response_code: 0,
            mime_type: None,
            content_length: 0,
            encoding: "".to_string(),
            date_resolved: None,
            date_published: None,
            title: "".to_string(),
            excerpt: "".to_string(),
            word_count: 0,
            innerdomain_redirect: false,
            login_required: false,
            has_image: PocketItemHas::No,
            has_video: PocketItemHas::No,
            is_index: false,
            is_article: false,
            used_fallback: false,
            lang: None,
            time_first_parsed: None,
            authors: None,
            images: None,
            videos: None,
            resolved_normal_url: None,
            given_url: url.clone(),
        }
    }

    #[test]
    fn add_writes_item() {
        let raw_url = "https://example.com";
        let pocket = PocketAddMock {
            add_mock: |r| Ok(added_item(r.url)),
            push_mock: |_| Err(PocketError::Proto(0, "".to_string())),
        };
        let opts = AddOpts {
            url: raw_url.into_url().unwrap(),
            title: None,
            tags: None,
            tweet_id: None,
        };
        let url = "https://example.com".into_url().unwrap();
        let expected_result = {
            let expected_item: Item = added_item(&url).into();
            format!("{}\n", serde_json::to_string(&expected_item).unwrap())
        };
        let test_target = json::test();
        let out = convey::new().add_target(test_target.target()).unwrap();

        handle(&pocket, &opts, out);

        assert_eq!(expected_result, test_target.to_string());
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
            tweet_id: None,
        };
        let test_target = json::test();
        let out = convey::new().add_target(test_target.target()).unwrap();

        handle(&pocket, &opts, out);
    }
}
