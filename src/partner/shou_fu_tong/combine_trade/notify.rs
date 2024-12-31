use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct TradeNotifyData {
    pub combine_appid: String,
    pub combine_mchid: String,
    pub combine_out_trade_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<SceneInfo>,
    pub sub_orders: Vec<SubOrder>,
    pub combine_payer_info: CombinePayerInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SceneInfo {
    pub device_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubOrder {
    pub mchid: String,
    pub trade_type: String,
    pub trade_state: String,
    pub bank_type: String,
    pub attach: String,
    pub success_time: String,
    pub transaction_id: String,
    pub out_trade_no: String,
    pub sub_mchid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_appid: Option<String>,
    pub amount: Amount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_detail: Option<Vec<PromotionDetail>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Amount {
    pub total_amount: i32,
    pub currency: String,
    pub payer_amount: i64,
    pub payer_currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_rate: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PromotionDetail {
    pub coupon_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>, // `type` is a reserved keyword in Rust, so prefix it with `r#`
    pub amount: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechatpay_contribute: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_contribute: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_contribute: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<Vec<GoodsDetail>>, // goods_detail moved here
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GoodsDetail {
    pub goods_id: String,
    pub quantity: i32,
    pub unit_price: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_remark: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CombinePayerInfo {
    pub openid: String,
}
