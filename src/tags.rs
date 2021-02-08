use chrono::{DateTime, Utc};
use pocket::{Pocket, PocketResult, PocketSendRequest, PocketSendResponse};
use structopt::StructOpt;

macro_rules! tags {
    ($command:ident, $action:ident) => {
        pub mod $command {
            use super::{PocketSend, TagsOpts};
            use pocket::{PocketSendAction, PocketSendRequest};
            use std::io::Write;
            use crate::output::Output;
            use crate::models::IndividualSendResponse;

            pub fn handle<W: Write>(pocket: &impl PocketSend, opts: &TagsOpts, output: &mut Output<W>) {
                let response: IndividualSendResponse = pocket
                    .send(&PocketSendRequest {
                        actions: &[&PocketSendAction::$action {
                            item_id: opts.item_id,
                            tags: opts
                                .tags
                                .as_ref()
                                .map(|tags| tags.join(","))
                                .unwrap_or("".to_string()),
                            time: opts.time.map(|t| t.timestamp() as u64),
                        }],
                    })
                    .unwrap()
                    .into();
                    output.write(response).unwrap();
            }
        }
    };
}

tags!(tags_add, TagsAdd);
tags!(tags_remove, TagsRemove);
tags!(tags_replace, TagsReplace);

#[derive(Debug, StructOpt)]
pub struct TagsOpts {
    item_id: u64,
    #[structopt(long = "tag")]
    tags: Option<Vec<String>>,
    #[structopt(long)]
    time: Option<DateTime<Utc>>,
}
pub trait PocketSend {
    fn send(&self, request: &PocketSendRequest) -> PocketResult<PocketSendResponse>;
}

impl PocketSend for Pocket {
    fn send<'a>(&self, request: &PocketSendRequest<'a>) -> PocketResult<PocketSendResponse> {
        self.send(request)
    }
}
