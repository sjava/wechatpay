use crate::{client::BASE_URL, WechatPayClient};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// 查询结算账户
/// 参见 <https://pay.weixin.qq.com/doc/v3/partner/4012761142>
pub async fn query_settlement(
    wxpay: &WechatPayClient,
    sub_mchid: &str,
) -> Result<SettlementQueryResponse> {
    let url = format!("apply4sub/sub_merchants/{}/settlement", sub_mchid);
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay.client.get(&url).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;
    Ok(res)
}

/// 修改结算账号
/// 参见 <https://pay.weixin.qq.com/doc/v3/partner/4012761138>
pub async fn modify_settlement(
    wxpay: &WechatPayClient,
    sub_mchid: &str,
    data: &SettlementModifyData,
) -> Result<SettlementModifyResponse> {
    let url = format!("apply4sub/sub_merchants/{}/modify-settlement", sub_mchid);
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay.client.post(&url).json(data).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;
    Ok(res)
}

/// 查询结算账户修改申请状态
/// 参见 <https://pay.weixin.qq.com/doc/v3/partner/4012761169>
pub async fn query_settlement_modify(
    wxpay: &WechatPayClient,
    sub_mchid: &str,
    application_no: &str,
) -> Result<QuerySettlementModifyResponse> {
    let url = format!(
        "apply4sub/sub_merchants/{}/application/{}",
        sub_mchid, application_no
    );
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay.client.get(&url).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;
    Ok(res)
}

/// 修改结算账号Data
#[derive(Debug, Serialize, Deserialize)]
pub struct SettlementModifyData {
    pub modify_mode: String,
    pub account_type: String,
    pub account_bank: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_branch_id: Option<String>,
    pub account_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}
/// 修改结算账号Response
#[derive(Debug, Serialize, Deserialize)]
pub struct SettlementModifyResponse {
    pub application_no: String,
}

/// 查询结算账户修改申请状态response
#[derive(Debug, Serialize, Deserialize)]
pub struct QuerySettlementModifyResponse {
    pub account_name: String,
    pub account_type: String,
    pub account_bank: String,
    pub bank_name: Option<String>,
    pub bank_branch_id: Option<String>,
    pub account_number: String,
    pub verify_result: String,
    pub verify_fail_reason: Option<String>,
    pub verify_finish_time: Option<String>,
}

/// 结算账号查询Response
#[derive(Serialize, Deserialize, Debug)]
pub struct SettlementQueryResponse {
    pub account_type: String,
    pub account_bank: String,
    pub bank_name: Option<String>,
    pub bank_branch_id: Option<String>,
    pub account_number: String,
    pub verify_result: String,
    pub verify_fail_reason: Option<String>,
}
/// 提交进件Response
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplymentResponse {
    pub applyment_id: u64,
    pub out_request_no: String,
}
