pub mod refund_apply;
pub mod refund_query;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundNotifyDetail {
    pub sp_mchid: String,
    pub sub_mchid: String,
    pub transaction_id: String,
    pub out_trade_no: String,
    pub refund_id: String,
    pub out_refund_no: String,
    pub refund_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_time: Option<String>,
    pub user_received_account: String,
    pub amount: RefundAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundAmount {
    pub total: i32,
    pub refund: i32,
    pub payer_total: i32,
    pub payer_refund: i32,
}
