pub mod applyment;
pub mod combine_trade;
pub mod profit_sharing;
pub mod refund;
pub mod fund;

use crate::{notify::WechatPayNotification, WechatPayClient};
use anyhow::Result;
use combine_trade::notify::TradeNotifyData;
use hyper::body::Bytes;
use refund::RefundNotifyData;
use serde::{Deserialize, Serialize};

/// 微信支付、退款通知。
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012237246
/// 验证微信支付结果通知签名
pub async fn verify_notification(
    wxpay: &WechatPayClient,
    request: http::Request<Bytes>,
) -> Result<http::Request<Bytes>> {
    wxpay.verify_notification(request).await
}

/// 解密微信支付、退款通知。
pub fn decrypt_notification(
    wxpay: &WechatPayClient,
    notify: &WechatPayNotification,
) -> Result<NotificationEvent> {
    let plain = wxpay.mch_credential.aes_decrypt(
        notify.resource.ciphertext.as_bytes(),
        notify.resource.associated_data.as_bytes(),
        notify.resource.nonce.as_bytes(),
    )?;

    let event = match notify.resource.original_type.as_str() {
        "transaction" => NotificationEvent::Trade(serde_json::from_slice(&plain)?),
        "refund" => NotificationEvent::Refund(serde_json::from_slice(&plain)?),
        _ => {
            return Err(anyhow::anyhow!(
                "unknown notification type: {}",
                notify.resource.original_type
            ));
        }
    };
    Ok(event)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationEvent {
    Trade(TradeNotifyData),
    Refund(RefundNotifyData),
}
