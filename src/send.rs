use chrono::{DateTime, Utc};
use pocket::{PocketSendRequest, PocketResult, PocketSendResponse, Pocket};
use structopt::StructOpt;

macro_rules! send_item {
    ($command:ident, $action:ident) => (
        pub mod $command {
            use super::{PocketSend, SendItemOpts};
            use pocket::{PocketSendRequest, PocketSendAction};
            use std::io::Write;

            pub fn handle(pocket: &impl PocketSend, opts: &SendItemOpts, mut writer: impl Write) {
                let response = pocket.send(&PocketSendRequest {
                    actions: &[
                        &PocketSendAction::$action {
                            item_id: opts.item_id,
                            time: opts.time.map(|t| t.timestamp() as u64)
                        }
                    ],
                }).unwrap();
                writeln!(writer, "response: {:?}", response).unwrap();
            }
        }
    )
}

send_item!(archive, Archive);
send_item!(delete, Delete);
send_item!(favorite, Favorite);
send_item!(readd, Readd);
send_item!(unfavorite, Unfavorite);

#[derive(Debug, StructOpt)]
pub struct SendItemOpts {
    item_id: u64,
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