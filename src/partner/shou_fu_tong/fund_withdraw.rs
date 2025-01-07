use crate::{client::BASE_URL, WechatPayClient};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// 二级商户预约提现
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012476652
pub async fn sub_mch_withdraw(
    wxpay: &WechatPayClient,
    request: &SubMchWithdrawalRequest,
) -> Result<SubMchWithdrawResponse> {
    let url = "ecommerce/fund/withdraw";
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay.client.post(url).json(request).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

/// 二级商户查询预约提现状态（根据商户预约提现单号查询）
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012476656
pub async fn query_sub_mch_withdraw_by_out_req_no(
    wxpay: &WechatPayClient,
    sub_mchid: &str,
    out_request_no: &str,
) -> Result<QuerySubMchWithdrawResponse> {
    let url = format!("ecommerce/fund/withdraw/out-request-no/{}", out_request_no);
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

/// 二级商户查询预约提现状态（根据微信支付预约提现单号查询）
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012476665
pub async fn query_sub_mch_withdraw_by_withdraw_id(
    wxpay: &WechatPayClient,
    sub_mchid: &str,
    withdraw_id: &str,
) -> Result<QuerySubMchWithdrawResponse> {
    let url = format!("ecommerce/fund/withdraw/{}", withdraw_id);
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

/// 平台预约提现
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012476670
pub async fn platform_withdraw(
    wxpay: &WechatPayClient,
    request: &PlatformWithdrawRequest,
) -> Result<PlatformWithdrawResponse> {
    let url = "merchant/fund/withdraw";
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay.client.post(url).json(request).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

/// 平台查询预约提现状态（根据商户预约提现单号查询）
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012476672
pub async fn query_platform_withdraw_by_out_req_no(
    wxpay: &WechatPayClient,
    out_request_no: &str,
) -> Result<QueryPlatformWithdrawResponse> {
    let url = format!("merchant/fund/withdraw/out-request-no/{}", out_request_no);
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay.client.get(url).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

/// 平台查询预约提现状态（根据微信支付预约提现单号查询）
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012476674
pub async fn query_platform_withdraw_by_withdraw_id(
    wxpay: &WechatPayClient,
    withdraw_id: &str,
) -> Result<QueryPlatformWithdrawResponse> {
    let url = format!("merchant/fund/withdraw/withdraw-id/{}", withdraw_id);
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay.client.get(url).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

/// 按日下载提现异常文件
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012476678
pub async fn download_withdraw_fail_file(
    wxpay: &WechatPayClient,
    bill_type: &str,
    bill_date: &str,
) -> Result<WithdrawFailFileInfo> {
    let url = format!("merchant/fund/withdraw/bill-type/{}", bill_type);
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay
        .client
        .get(url)
        .query(&[("bill_date", bill_date)])
        .build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPlatformWithdrawResponse {
    status: String,
    withdraw_id: String,
    out_request_no: String,
    amount: u64,
    create_time: String,
    update_time: String,
    reason: String,
    remark: String,
    bank_memo: String,
    account_type: String,
    solution: String,
    account_number: String,
    account_bank: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    bank_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformWithdrawRequest {
    out_request_no: String,
    amount: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    remark: Option<String>, // 可选字段
    #[serde(skip_serializing_if = "Option::is_none")]
    bank_memo: Option<String>, // 可选字段
    account_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify_url: Option<String>, // 可选字段
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformWithdrawResponse {
    withdraw_id: String,
    out_request_no: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubMchWithdrawalRequest {
    pub sub_mchid: String,
    pub out_request_no: String,
    pub amount: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubMchWithdrawResponse {
    pub sub_mchid: String,
    pub withdraw_id: String,
    pub out_request_no: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuerySubMchWithdrawResponse {
    sp_mchid: String,
    sub_mchid: String,
    status: String,
    withdraw_id: String,
    out_request_no: String,
    amount: u64,
    create_time: String,
    update_time: String,
    reason: String,
    remark: String,
    bank_memo: String,
    account_type: String,
    account_number: String,
    account_bank: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    bank_name: Option<String>, // 可选字段
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawFailFileInfo {
    hash_type: String,
    hash_value: String,
    download_url: String,
}
