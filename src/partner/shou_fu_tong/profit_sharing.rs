pub mod share_apply;
pub mod share_query;
pub mod share_finish;
pub mod share_remainder;

pub use share_query::share_query;
pub use share_apply::share_request;
pub use share_finish::share_finish;

use crate::util::datetime_fmt;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Receiver {
    #[serde(rename = "type")]
    pub receiver_type: String,
    pub account: String,
    pub amount: u32,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfitShareNotifyData {
    pub sp_mchid: String,
    pub sub_mchid: String,
    pub transaction_id: String,
    pub order_id: String,
    pub out_order_no: String,
    pub receiver: Receiver,
    #[serde(with = "datetime_fmt")]
    pub success_time: DateTime<Local>,
}
