pub mod applyment;
pub mod combine_trade;
pub mod fund_balance;
pub mod fund_withdraw;
pub mod profit_sharing;
pub mod refund;

use crate::{notify::WechatPayNotification, trade::JsApiTradeSignature, WechatPayClient};
use anyhow::Result;
use applyment::{
    apply_query::ApplymentQueryResponse,
    utils::{PersonalBankingResponse, UploadResponse},
    ApplymentResponse, SubMerchantApplication,
};
use async_trait::async_trait;
use combine_trade::{
    mini_program_pay::{MiniProgramPrepayRequest, MiniProgramPrepayResponse},
    notify::TradeNotifyData,
    CombineClosData, CombineOrderQueryResponse,
};
use hyper::body::Bytes;
use profit_sharing::{
    share_apply::{ShareRequestBody, ShareResponseBody},
    share_query::ShareQueryResponse,
    ProfitShareNotifyData,
};
use refund::RefundNotifyData;
use serde::{Deserialize, Serialize};

/// 解密微信支付、退款、分账通知。
/// 文档地址：https://pay.weixin.qq.com/doc/v3/partner/4012237246
fn decrypt_notification(
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
pub trait ShouFuTong: Send + Sync {
    async fn applyment_submit(&self, payload: &SubMerchantApplication)
        -> Result<ApplymentResponse>;
    async fn query_applyment_by_applyment_id(
        &self,
        applyment_id: u64,
    ) -> Result<ApplymentQueryResponse>;

    async fn applyment_upload_image(
        &self,
        image: Vec<u8>,
        filename: &str,
    ) -> Result<UploadResponse>;
    async fn get_personal_banking(&self, url: &str) -> Result<PersonalBankingResponse>;

    async fn mini_program_prepay(
        &self,
        data: &MiniProgramPrepayRequest,
    ) -> Result<MiniProgramPrepayResponse>;

    async fn query_combine_order(
        &self,
        combine_out_trade_no: &str,
    ) -> Result<CombineOrderQueryResponse>;
    async fn close_combine_order(
        &self,
        combine_out_trade_no: &str,
        data: &CombineClosData,
    ) -> Result<()>;

    async fn query_share(
        &self,
        sub_mchid: &str,
        transaction_id: &str,
        out_order_no: &str,
    ) -> Result<ShareQueryResponse>;

    async fn share_request(&self, data: &ShareRequestBody) -> Result<ShareResponseBody>;

    async fn verify_notification(&self, req: http::Request<Bytes>) -> Result<http::Request<Bytes>>;

    fn decrypt_shou_fu_tong_notification(
        &self,
        notify: &WechatPayNotification,
    ) -> Result<NotificationEvent>;

    fn encrypt(&self, data: &str) -> Result<String>;

    fn sign_jsapi_trade(&self, prepay_id: &str, app_id: &str) -> JsApiTradeSignature;

    fn get_mch_id(&self) -> &str;
}

#[async_trait]
impl ShouFuTong for WechatPayClient {
    async fn applyment_submit(
        &self,
        payload: &SubMerchantApplication,
    ) -> Result<ApplymentResponse> {
        applyment::submit(self, payload).await
    }

    async fn applyment_upload_image(
        &self,
        image: Vec<u8>,
        filename: &str,
    ) -> Result<UploadResponse> {
        applyment::utils::upload_image(self, image, filename).await
    }

    async fn get_personal_banking(&self, url: &str) -> Result<PersonalBankingResponse> {
        applyment::utils::get_personal_banking(self, url).await
    }

    async fn mini_program_prepay(
        &self,
        data: &MiniProgramPrepayRequest,
    ) -> Result<MiniProgramPrepayResponse> {
        combine_trade::mini_program_pay::mini_program_prepay(self, data).await
    }

    async fn query_combine_order(
        &self,
        combine_out_trade_no: &str,
    ) -> Result<CombineOrderQueryResponse> {
        combine_trade::query_combine_order(self, combine_out_trade_no).await
    }
    async fn close_combine_order(
        &self,
        combine_out_trade_no: &str,
        data: &CombineClosData,
    ) -> Result<()> {
        combine_trade::close_combine_order(self, combine_out_trade_no, data).await
    }

    async fn query_applyment_by_applyment_id(
        &self,
        applyment_id: u64,
    ) -> Result<ApplymentQueryResponse> {
        applyment::apply_query::query_applyment_by_applyment_id(self, applyment_id).await
    }

    async fn query_share(
        &self,
        sub_mchid: &str,
        transaction_id: &str,
        out_order_no: &str,
    ) -> Result<ShareQueryResponse> {
        profit_sharing::share_query::query_share(self, sub_mchid, transaction_id, out_order_no)
            .await
    }
    async fn share_request(&self, data: &ShareRequestBody) -> Result<ShareResponseBody> {
        profit_sharing::share_apply::share_request(self, data).await
    }

    async fn verify_notification(&self, req: http::Request<Bytes>) -> Result<http::Request<Bytes>> {
        self.verify_notification(req).await
    }

    fn decrypt_shou_fu_tong_notification(
        &self,
        notify: &WechatPayNotification,
    ) -> Result<NotificationEvent> {
        decrypt_notification(self, notify)
    }

    fn encrypt(&self, data: &str) -> Result<String> {
        self.platform_certificate.encrypt(data.as_bytes())
    }

    fn sign_jsapi_trade(&self, prepay_id: &str, app_id: &str) -> JsApiTradeSignature {
        self.sign_jsapi_trade(prepay_id, app_id)
    }

    fn get_mch_id(&self) -> &str {
        &self.mch_credential.mch_id
    }
}
