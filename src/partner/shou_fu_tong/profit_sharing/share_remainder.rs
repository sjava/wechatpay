use crate::{client::BASE_URL, WechatPayClient};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// 分账剩余未分金额查询
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012477751
pub async fn share_remainder_query(
    wxpay: &WechatPayClient,
    transaction_id: &str,
) -> Result<ShareRemainderQueryResponseBody> {
    let url = format!("ecommerce/profitsharing/orders/{}/amounts", transaction_id);
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay.client.get(url).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareRemainderQueryResponseBody {
    pub transaction_id: String,
    pub unsplit_amount: i32,
}
