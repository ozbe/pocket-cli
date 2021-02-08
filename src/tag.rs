use crate::models::IndividualSendResponse;
use crate::output::Output;
use chrono::{DateTime, Utc};
use pocket::*;
use std::io::Write;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Tag {
    /// Rename
    Rename {
        old_tag: String,
        new_tag: String,
        #[structopt(long)]
        time: Option<DateTime<Utc>>,
    },
    /// Delete
    Delete {
        tag: String,
        #[structopt(long)]
        time: Option<DateTime<Utc>>,
    },
}

pub fn handle<W: Write>(pocket: &impl PocketSend, opts: &Tag, output: &mut Output<W>) {
    let action = match opts {
        Tag::Rename {
            old_tag,
            new_tag,
            time,
        } => PocketSendAction::TagRename {
            old_tag: old_tag.clone(),
            new_tag: new_tag.clone(),
            time: time.map(|t| t.timestamp() as u64),
        },
        Tag::Delete { tag, time } => PocketSendAction::TagDelete {
            tag: tag.clone(),
            time: time.map(|t| t.timestamp() as u64),
        },
    };

    let response: IndividualSendResponse = pocket
        .send(&PocketSendRequest {
            actions: &[&action],
        })
        .unwrap()
        .into();
    output.write(response).unwrap();
}

pub trait PocketSend {
    fn send(&self, request: &PocketSendRequest) -> PocketResult<PocketSendResponse>;
}

impl PocketSend for Pocket {
    fn send<'a>(&self, request: &PocketSendRequest<'a>) -> PocketResult<PocketSendResponse> {
        self.send(request)
    }
}
