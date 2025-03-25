use crate::{client::BASE_URL, WechatPayClient};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// 申请退款
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012476892
pub async fn refund_apply(
    wxpay: &WechatPayClient,
    data: &RefundRequestBody,
) -> Result<RefundResponseBody> {
    let url = "ecommerce/refunds/apply";
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay.client.post(url).json(data).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefundRequestBody {
    pub sub_mchid: String,
    pub sp_appid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_appid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    pub out_refund_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub amount: Amount,
    pub notify_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funds_account: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Amount {
    pub refund: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<FromAccount>>, // 可选字段
    pub total: i32,
    pub currency: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FromAccount {
    pub account: String,
    pub amount: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefundResponseBody {
    pub refund_id: String,
    pub out_refund_no: String,
    pub create_time: String,
    pub amount: AmountResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_detail: Option<Vec<PromotionDetail>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountResponse {
    pub refund: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<FromAccount>>, // 可选字段
    pub payer_refund: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_refund: Option<i32>, // 可选字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>, // 可选字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advance: Option<i32>, // 可选字段
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PromotionDetail {
    pub promotion_id: String,
    pub scope: String,
    pub type_: String,
    pub amount: i32,
    pub refund_amount: i32,
}
