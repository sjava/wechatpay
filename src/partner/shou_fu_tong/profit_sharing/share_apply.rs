use crate::{client::BASE_URL, WechatPayClient};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// 分账请求
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012691594
pub(in crate::partner::shou_fu_tong) async fn share_request(
    wxpay: &WechatPayClient,
    data: &ShareRequestBody,
) -> Result<ShareResponseBody> {
    let url = "ecommerce/profitsharing/orders";
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay.client.post(url).json(data).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Receiver {
    #[serde(rename = "type")]
    pub receiver_type: String,
    pub receiver_account: String,
    pub amount: u32,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareRequestBody {
    pub appid: String,
    pub sub_mchid: String,
    pub transaction_id: String,
    pub out_order_no: String,
    pub receivers: Vec<Receiver>,
    pub finish: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseReceiver {
    pub receiver_mchid: String,
    pub amount: u32,
    pub description: String,
    pub result: String,
    pub finish_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_reason: Option<String>, // 可选字段，序列化时跳过 None
    #[serde(rename = "type")]
    pub receiver_type: String,
    pub receiver_account: String,
    pub detail_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareResponseBody {
    pub sub_mchid: String,
    pub transaction_id: String,
    pub out_order_no: String,
    pub order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receivers: Option<Vec<ResponseReceiver>>, // 可选字段，序列化时跳过 None
    pub status: String,
}
