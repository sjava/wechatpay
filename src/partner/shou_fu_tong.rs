pub mod applyment;
pub mod combine_trade;
pub mod fund_balance;
pub mod fund_withdraw;
pub mod profit_sharing;
pub mod refund;

use crate::{notify::WechatPayNotification, WechatPayClient};
use anyhow::Result;
use applyment::{ApplymentResponse, SubMerchantApplication};
use async_trait::async_trait;
use combine_trade::notify::TradeNotifyData;
use profit_sharing::ProfitShareNotifyData;
use refund::RefundNotifyData;
use serde::{Deserialize, Serialize};

/// 解密微信支付、退款、分账通知。
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012237246
pub fn decrypt_notification(
    wxpay: &WechatPayClient,
    notify: &WechatPayNotification,
) -> Result<NotificationEvent> {
    let plain = wxpay.mch_credential.aes_decrypt(
        &notify.resource.ciphertext,
        &notify.resource.associated_data,
        &notify.resource.nonce,
    )?;

    let event = match notify.resource.original_type.as_str() {
        "transaction" => NotificationEvent::Trade(serde_json::from_slice(&plain)?),
        "refund" => NotificationEvent::Refund(serde_json::from_slice(&plain)?),
        "profitsharing" => NotificationEvent::ProfitShare(serde_json::from_slice(&plain)?),
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
    ProfitShare(ProfitShareNotifyData),
}

#[async_trait]
pub trait ShouFuTong {
    async fn applyment_submit(&self, payload: u32) -> Result<ApplymentResponse>;
}
