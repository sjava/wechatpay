use crate::{client::BASE_URL, WechatPayClient};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// 查询二级商户账户实时余额
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012476690
pub async fn query_sub_mch_balance(
    wxpay: &WechatPayClient,
    sub_mchid: &str,
    account_type: &str,
) -> Result<SubMchBalanceResponse> {
    let url = format!("ecommerce/fund/balance/{}", sub_mchid);
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay
        .client
        .get(url)
        .query(&[("account_type", account_type)])
        .build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

/// 查询二级商户账户日终余额
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012476693
pub async fn query_sub_mch_end_day_balance(
    wxpay: &WechatPayClient,
    sub_mchid: &str,
    account_type: &str,
    date: &str,
) -> Result<SubMchBalanceResponse> {
    let url = format!("ecommerce/fund/enddaybalance/{}", sub_mchid);
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay
        .client
        .get(url)
        .query(&[("account_type", account_type), ("date", date)])
        .build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

/// 查询平台账户实时余额
/// 文档地址：<https://pay.weixin.qq.com/doc/v3/partner/4012476700>
pub async fn query_platform_balance(
    wxpay: &WechatPayClient,
    account_type: &str,
) -> Result<PlatformBalanceResponse> {
    let url = "merchant/fund/balance";
    let url = format!("{}/{}/{}", BASE_URL, url, account_type);

    let req = wxpay.client.get(url).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

/// 查询平台账户日终余额
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012476702
pub async fn query_platform_end_day_balance(
    wxpay: &WechatPayClient,
    account_type: &str,
    date: &str,
) -> Result<PlatformBalanceResponse> {
    let url = "merchant/fund/dayendbalance";
    let url = format!("{}/{}/{}", BASE_URL, url, account_type);

    let req = wxpay.client.get(url).query(&[("date", date)]).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubMchBalanceResponse {
    pub sub_mchid: String,
    pub available_amount: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformBalanceResponse {
    pub available_amount: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_amount: Option<u32>,
}
