use crate::{client::BASE_URL, WechatPayClient};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// 分账查询
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012477734
pub(in crate::partner::shou_fu_tong) async fn query_share(
    wxpay: &WechatPayClient,
    sub_mchid: &str,
    transaction_id: &str,
    out_order_no: &str,
) -> Result<ShareQueryResponse> {
    let url = format!("{}/ecommerce/profitsharing/orders", BASE_URL);

    let req = wxpay
        .client
        .get(url)
        .query(&[
            ("sub_mchid", sub_mchid),
            ("transaction_id", transaction_id),
            ("out_order_no", out_order_no),
        ])
        .build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareQueryResponse {
    pub sub_mchid: String,
    pub transaction_id: String,
    pub out_order_no: String,
    pub order_id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receivers: Option<Vec<Receiver>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_amount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Receiver {
    pub receiver_mchid: String,
    pub amount: i32,
    pub description: String,
    pub result: String,
    pub finish_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_reason: Option<String>,
    #[serde(rename = "type")]
    pub receiver_type: String,
    pub receiver_account: String,
    pub detail_id: String,
}
