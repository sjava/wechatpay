pub mod mini_program_pay;
pub mod notify;

use crate::util::option_datetime_fmt;
use crate::{client::BASE_URL, WechatPayClient};
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError};

pub use mini_program_pay::mini_program_prepay;

/// 合单查询订单
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012761049
pub async fn query_combine_order(
    wxpay: &WechatPayClient,
    combine_out_trade_no: &str,
) -> Result<CombineOrderQueryResponse> {
    let url = "combine-transactions/out-trade-no";
    let url = format!("{}/{}/{}", BASE_URL, url, combine_out_trade_no);

    let req = wxpay.client.get(url).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;

    Ok(res)
}

/// 合单关闭订单
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012761093
pub async fn close_combine_order(
    wxpay: &WechatPayClient,
    combine_out_trade_no: &str,
    data: &CombineClosData,
) -> Result<()> {
    let url = format!(
        "combine-transactions/out-trade-no/{}/close",
        combine_out_trade_no
    );
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay.client.post(url).json(data).build()?;
    let _res = wxpay.execute(req, None).await?;

    Ok(())
}

/// 合单关闭订单
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012761093
#[derive(Debug, Serialize, Deserialize)]
pub struct CombineClosData {
    /// 合单发起方的 Appid
    pub combine_appid: String,

    /// 子单列表
    pub sub_orders: Vec<ReqCloseSubOrder>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReqCloseSubOrder {
    /// 子单商户号，与合单发起方的 Appid 有绑定关系
    pub mchid: String,

    /// 商品单订单号，商户系统内部对商品单定义的订单号
    pub out_trade_no: String,

    /// 特约商户商户号（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,

    /// 服务商模式下，`sub_mchid` 对应的 `sub_appid`（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_appid: Option<String>,
}

/// 合单支付查询Response
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012761049
#[derive(Debug, Serialize, Deserialize)]
pub struct CombineOrderQueryResponse {
    pub combine_appid: String, // 必填，组合支付的应用ID
    pub combine_mchid: String, // 必填，组合支付的商户ID

    #[serde(skip_serializing_if = "Option::is_none")]
    pub combine_payer_info: Option<CombinePayerInfo>, // 选填，组合支付者信息

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_orders: Option<Vec<SubOrder>>, // 子订单列表

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<SceneInfo>, // 选填，场景信息

    pub combine_out_trade_no: String, // 必填，组合支付的商户订单号
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CombinePayerInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>, // 选填，用户在商户appid下的唯一标识
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct SubOrder {
    pub mchid: String, // 必填，子单商户号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_type: Option<String>, // 选填，交易类型
    pub trade_state: String, // 必填，交易状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_type: Option<String>, // 选填，付款银行
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>, // 选填，附加数据

    #[serde(default, with = "option_datetime_fmt")]
    pub success_time: Option<DateTime<Local>>, // 选填，支付完成时间

    #[serde_as(as = "DefaultOnError<_>")]
    pub amount: Option<Amount>, // 选填，订单金额

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>, // 选填，微信支付订单号
    pub out_trade_no: String, // 必填，商品单订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>, // 选填，特约商户商户号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_appid: Option<String>, // 选填，子商户绑定的Appid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_openid: Option<String>, // 选填，sub_appid 对应的 openid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_detail: Option<Vec<PromotionDetail>>, // 选填，优惠功能
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    pub total_amount: i32,      // 必填，标价金额
    pub payer_amount: i32,      // 必填，现金支付金额
    pub currency: String,       // 必填，标价币种
    pub payer_currency: String, // 必填，现金支付币种
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_rate: Option<u64>, // 选填，结算汇率
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromotionDetail {
    pub coupon_id: String, // 必填，券ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, // 选填，优惠名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>, // 选填，优惠范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>, // 选填，优惠类型
    pub amount: i32,       // 必填，优惠券面额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_id: Option<String>, // 选填，活动ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechatpay_contribute: Option<i32>, // 选填，微信出资
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_contribute: Option<i32>, // 选填，商户出资
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_contribute: Option<i32>, // 选填，其他出资
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>, // 选填，优惠币种
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<Vec<GoodsDetail>>, // 选填，单品列表
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoodsDetail {
    pub goods_id: String,     // 必填，商品编码
    pub quantity: i32,        // 必填，商品数量
    pub unit_price: i32,      // 必填，商品单价
    pub discount_amount: i32, // 必填，商品优惠金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_remark: Option<String>, // 选填，商品备注
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>, // 选填，商户端设备号
}
