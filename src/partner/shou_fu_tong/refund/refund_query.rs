use crate::{client::BASE_URL, WechatPayClient};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// 退款查询
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012476908
pub async fn refund_query_by_transaction_id(
    wxpay: &WechatPayClient,
    refund_id: &str,
    sub_mchid: &str,
) -> Result<RefundQueryResponseBody> {
    let url = format!("ecommerce/refunds/id/{}", refund_id);
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay
        .client
        .get(url)
        .query(&[("sub_mchid", sub_mchid)])
        .build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

/// 退款查询
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012476911
pub async fn refund_query_by_out_refund_no(
    wxpay: &WechatPayClient,
    out_refund_no: &str,
    sub_mchid: &str,
) -> Result<RefundQueryResponseBody> {
    let url = format!("ecommerce/refunds/out-refund-no/{}", out_refund_no);
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay
        .client
        .get(url)
        .query(&[("sub_mchid", sub_mchid)])
        .build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundQueryResponseBody {
    pub refund_id: String,
    pub out_refund_no: String,
    pub transaction_id: String,
    pub out_trade_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_received_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_time: Option<String>,
    pub create_time: String,
    pub status: String,
    pub amount: RefundAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_detail: Option<Vec<PromotionDetail>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funds_account: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundAmount {
    pub refund: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<RefundFrom>>,
    pub payer_refund: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_refund: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advance: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundFrom {
    pub account: String,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromotionDetail {
    pub promotion_id: String,
    pub scope: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub amount: i32,
    pub refund_amount: i32,
}
