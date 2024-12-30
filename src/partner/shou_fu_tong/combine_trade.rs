use crate::{client::BASE_URL, WechatPayClient};
use anyhow::Result;
use serde::{Deserialize, Serialize};

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
    data: &CombineOrderData,
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
pub struct CombineOrderData {
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
    /// 合单商户Appid，最大长度32
    pub combine_appid: String,

    /// 合单商户号，最大长度32
    pub combine_mchid: String,

    /// 合单支付者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combine_payer_info: Option<CombinePayerInfo>,

    /// 子单信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_orders: Option<Vec<SubOrder>>,

    /// 场景信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<SceneInfo>,

    /// 合单商户订单号
    pub combine_out_trade_no: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CombinePayerInfo {
    /// 用户标识，最大长度128
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubOrder {
    /// 子单商户号，最大长度32
    pub mchid: String,

    /// 交易类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_type: Option<TradeType>,

    /// 交易状态
    pub trade_state: TradeState,

    /// 付款银行，最大长度32
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_type: Option<String>,

    /// 附加数据，最大长度128
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,

    /// 支付完成时间，最大长度32
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_time: Option<String>,

    /// 订单金额信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,

    /// 微信支付订单号，最大长度32
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,

    /// 商品单订单号，最大长度32
    pub out_trade_no: String,

    /// 特约商户商户号，最大长度32
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,

    /// 子商户绑定的Appid，最大长度32
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_appid: Option<String>,

    /// sub_appid对应的openid，最大长度128
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_openid: Option<String>,

    /// 优惠功能
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_detail: Option<Vec<PromotionDetail>>,
}

/// 交易类型枚举
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TradeType {
    /// Native支付
    Native,
    /// 公众号支付
    Jsapi,
    /// APP支付
    App,
    /// H5支付
    Mweb,
}

/// 交易状态枚举
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TradeState {
    /// 支付成功
    Success,
    /// 转入退款
    Refund,
    /// 未支付
    Notpay,
    /// 已关闭
    Closed,
    /// 支付失败
    Payerror,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    /// 标价金额，单位为分
    pub total_amount: u32,

    /// 现金支付金额
    pub payer_amount: u32,

    /// 标价币种，最大长度16
    pub currency: String,

    /// 现金支付币种，最大长度16
    pub payer_currency: String,

    /// 结算汇率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_rate: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromotionDetail {
    /// 券ID，最大长度32
    pub coupon_id: String,

    /// 优惠名称，最大长度64
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// 优惠范围，最大长度32
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// 优惠类型，最大长度32
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub promotion_type: Option<String>,

    /// 优惠券面额
    pub amount: u32,

    /// 活动ID，最大长度32
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_id: Option<String>,

    /// 微信出资，单位为分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechatpay_contribute: Option<u32>,

    /// 商户出资，单位为分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_contribute: Option<u32>,

    /// 其他出资，单位为分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_contribute: Option<u32>,

    /// 优惠币种，最大长度16
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// 单品列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<Vec<GoodsDetail>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoodsDetail {
    /// 商品编码，最大长度32
    pub goods_id: String,

    /// 商品数量
    pub quantity: u32,

    /// 商品单价，单位为分
    pub unit_price: u32,

    /// 商品优惠金额
    pub discount_amount: u32,

    /// 商品备注，最大长度128
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneInfo {
    /// 商户端设备号，最大长度32
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}
