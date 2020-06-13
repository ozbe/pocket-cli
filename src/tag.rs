use pocket::*;
use structopt::StructOpt;
use chrono::{Utc, DateTime};

#[derive(Debug, StructOpt)]
pub enum Tag {
    Rename {
        old_tag: String,
        new_tag: String,
        #[structopt(long)]
        time: Option<DateTime<Utc>>,
    },
    Delete {
        tag: String,
        #[structopt(long)]
        time: Option<DateTime<Utc>>,
    },
}

pub fn handle(pocket: &impl PocketSend, opts: &Tag, mut writer: impl std::io::Write) {
    let action = match opts {
        Tag::Rename { old_tag, new_tag, time } => PocketSendAction::TagRename {
            old_tag: old_tag.clone(),
            new_tag: new_tag.clone(),
            time: time.map(|t| t.timestamp() as u64)
        },
        Tag::Delete { tag, time } => PocketSendAction::TagDelete {
            tag: tag.clone(),
            time: time.map(|t| t.timestamp() as u64)
        },
    };

    let response = pocket.send(&PocketSendRequest {
        actions: &[
            &action
        ],
    }).unwrap();
    writeln!(writer, "response: {:?}", response).unwrap();
}

pub trait PocketSend {
    fn send(&self, request: &PocketSendRequest) -> PocketResult<PocketSendResponse>;
}

impl PocketSend for Pocket {
    fn send<'a>(&self, request: &PocketSendRequest<'a>) -> PocketResult<PocketSendResponse> {
        self.send(request)
    }
}