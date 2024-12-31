use crate::{client::BASE_URL, WechatPayClient};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// 分账完结
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012477745
pub async fn share_finish(
    wxpay: &WechatPayClient,
    data: &ShareFinishRequestBody,
) -> Result<ShareFinishResponseBody> {
    let url = "ecommerce/profitsharing/finish-order";
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay.client.post(url).json(data).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareFinishRequestBody {
    pub sub_mchid: String,
    pub transaction_id: String,
    pub out_order_no: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareFinishResponseBody {
    pub sub_mchid: String,
    pub transaction_id: String,
    pub out_order_no: String,
    pub order_id: String,
}
