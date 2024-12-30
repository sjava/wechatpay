use serde::{Deserialize, Serialize};

/// 合单支付-小程序下单
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012760633
#[derive(Serialize, Deserialize)]
pub struct MiniProgramPrepayRequest {
    combine_appid: String,
    combine_mchid: String,
    combine_out_trade_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    scene_info: Option<SceneInfo>,
    sub_orders: Vec<SubOrder>,
    combine_payer_info: CombinePayerInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_expire: Option<String>,
    notify_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct SceneInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_id: Option<String>,
    payer_client_ip: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubOrder {
    mchid: String,
    attach: String,
    amount: Amount,
    out_trade_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_mchid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    settle_info: Option<SettleInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_appid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    goods_tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Amount {
    total_amount: i32,
    currency: String,
}

#[derive(Serialize, Deserialize)]
pub struct SettleInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    profit_sharing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subsidy_amount: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct CombinePayerInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    openid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_openid: Option<String>,
}

/// 合单支付-小程序下单Response
#[derive(Serialize, Deserialize)]
pub struct MiniProgramPrepayResponse {
    prepay_id: String,
}
