use chrono::{DateTime, Utc};
use pocket::{PocketSendRequest, PocketResult, PocketSendResponse, PocketSendAction, Pocket};
use structopt::StructOpt;
use std::io::Write;

#[derive(Debug, StructOpt)]
pub struct ArchiveOpts {
    item_id: u64,
    #[structopt(long)]
    time: Option<DateTime<Utc>>,
}

pub fn handle(pocket: &impl PocketSend, opts: &ArchiveOpts, mut writer: impl Write) {
    let response = pocket.send(&PocketSendRequest {
        actions: &[
            &PocketSendAction::Archive {
                item_id: opts.item_id,
                time: opts.time.map(|t| t.timestamp() as u64)
            }
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