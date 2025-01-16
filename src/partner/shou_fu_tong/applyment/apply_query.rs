use crate::{client::BASE_URL, WechatPayClient};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// 通过业务申请编号查询申请状态
/// 参见 <https://pay.weixin.qq.com/doc/v3/partner/4012691376>
pub async fn query_applyment_by_out_request_no(
    wxpay: &WechatPayClient,
    out_request_no: &str,
) -> Result<ApplymentQueryResponse> {
    let url = "ecommerce/applyments/out-request-no";
    let url = format!("{}/{}/{}", BASE_URL, url, out_request_no);

    let req = wxpay.client.get(&url).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;
    Ok(res)
}

/// 通过申请单ID查询申请状态
/// 参见 <https://pay.weixin.qq.com/doc/v3/partner/4012691469>
pub(in crate::partner::shou_fu_tong) async fn query_applyment_by_applyment_id(
    wxpay: &WechatPayClient,
    applyment_id: u64,
) -> Result<ApplymentQueryResponse> {
    let url = "ecommerce/applyments";
    let url = format!("{}/{}/{}", BASE_URL, url, applyment_id);

    let req = wxpay.client.get(&url).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;
    Ok(res)
}

/// 进件查询response
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplymentQueryResponse {
    pub applyment_state: String,
    pub applyment_state_desc: String,
    pub sign_url: Option<String>,
    pub sub_mchid: Option<String>,
    pub account_validation: Option<AccountValidation>,
    pub audit_detail: Option<Vec<AuditDetail>>,
    pub legal_validation_url: Option<String>,
    pub out_request_no: String,
    pub applyment_id: u64,
    pub sign_state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountValidation {
    pub account_name: String,
    pub account_no: Option<String>,
    pub pay_amount: u32,
    pub destination_account_number: String,
    pub destination_account_name: String,
    pub destination_account_bank: String,
    pub city: String,
    pub remark: String,
    pub deadline: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditDetail {
    pub param_name: String,
    pub reject_reason: String,
}
