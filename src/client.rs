use crate::credential::MchCredential;
use crate::error::WechatPayApiError;
use crate::platform_certificate::PlatformCertificate;
use anyhow::Result;
use reqwest::{Client, Request, Response};

#[derive(Debug, Clone)]
pub struct WechatPayClient {
    pub(crate) client: Client,
    pub(crate) mch_credential: MchCredential,
    pub(crate) platform_certificate: PlatformCertificate,
}

pub(crate) const BASE_URL: &str = "https://api.mch.weixin.qq.com/v3";

impl WechatPayClient {
    pub fn new(
        ua: String,
        mch_credential: MchCredential,
        platform_certificate: PlatformCertificate,
    ) -> Result<WechatPayClient> {
        let client = Client::builder().user_agent(ua).build()?;
        Ok(WechatPayClient {
            client,
            mch_credential,
            platform_certificate,
        })
    }

    /// 执行 HTTP 请求
    /// 请求发送时，先进行签名；收到响应时，先进行验签，通过后再返回。
    /// (本 crate 未实现的接口，可以通过此方法访问)
    pub async fn execute(&self, req: Request) -> Result<Response> {
        let mut req = req;
        // 根据 https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay2_0.shtml#part-1
        // 给所有请求都加上 accept header。
        req.headers_mut()
            .append("Accept", "application/json".parse().unwrap());

        let req = self.mch_credential.sign_request(req)?;
        let res = self.client.execute(req).await?;

        // 请求出错时，响应中可能不存在验签相关的字段。因此直接返回 error。
        if !res.status().is_success() {
            let e: WechatPayApiError = res.json().await?;
            Err(e.into())
        } else {
            self.verify_response(res).await
        }
    }

    /// 对响应进行数字签名验证。
    pub(crate) async fn verify_response(&self, res: Response) -> Result<Response> {
        self.platform_certificate.verify_response(res).await
    }
}
