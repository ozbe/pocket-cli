use chrono::{DateTime, Utc};
use pocket::{Pocket, PocketResult, PocketSendRequest, PocketSendResponse};
use structopt::StructOpt;

macro_rules! send_item {
    ($command:ident, $action:ident) => {
        pub mod $command {
            use super::{PocketSend, SendItemOpts};
            use crate::models::IndividualSendResponse;
            use crate::output::Output;
            use pocket::{PocketSendAction, PocketSendRequest};
            use std::io::Write;

            pub fn handle<W: Write>(
                pocket: &impl PocketSend,
                opts: &SendItemOpts,
                output: &mut Output<W>,
            ) {
                let response: IndividualSendResponse = pocket
                    .send(&PocketSendRequest {
                        actions: &[&PocketSendAction::$action {
                            item_id: opts.item_id,
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

send_item!(archive, Archive);
send_item!(delete, Delete);
send_item!(favorite, Favorite);
send_item!(readd, Readd);
send_item!(unfavorite, Unfavorite);
send_item!(tags_clear, TagsClear);

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
