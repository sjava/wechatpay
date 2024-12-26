//! 微信支付平台证书。

use anyhow::Result;
use base64::prelude::*;
use bytes::{BufMut, BytesMut};
use reqwest::Response;
use rsa::pkcs1v15::{Signature, VerifyingKey};
use rsa::sha2::Sha256;
use rsa::signature::Verifier;
use rsa::{Oaep, RsaPublicKey};
use sha1::Sha1;

/// 微信支付平台证书。
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PlatformCertificate {
    pub public_id: String,
    pub public_key: RsaPublicKey,
}

impl PlatformCertificate {
    /// 对响应进行数字签名验证。
    pub(crate) async fn verify_response(&self, res: Response) -> Result<Response> {
        let res = verify_response(&self.public_key, res).await?;
        Ok(res)
    }

    // TODO: 定义一个 RSA 加密方法，用于对敏感信息进行加密
    pub fn encrypt(&self, data: &[u8]) -> Result<String> {
        let mut rng = rand::thread_rng();

        let enc_data = self
            .public_key
            .encrypt(&mut rng, Oaep::new::<Sha1>(), data)?;

        // Convert to base64
        Ok(BASE64_STANDARD.encode(enc_data))
    }
}

/// 响应签名验证器: 对响应进行数字签名验证。
/// 验证响应的签名。
/// <https://pay.weixin.qq.com/doc/v3/merchant/4013053249>
pub async fn verify_response(public_key: &RsaPublicKey, res: Response) -> Result<Response> {
    // 需要这个 builder 重新构建一个 Response 并返回。
    let mut builder = http::Response::builder()
        .status(res.status())
        .version(res.version());
    for (key, value) in res.headers() {
        builder = builder.header(key, value);
    }

    let signature = res
        .headers()
        .get("Wechatpay-Signature")
        .ok_or_else(|| anyhow::format_err!("missing `Wechatpay-Signature` header"))?
        .to_str()?;
    let signature = BASE64_STANDARD.decode(signature.as_bytes())?;

    let timestamp = res
        .headers()
        .get("Wechatpay-Timestamp")
        .ok_or_else(|| anyhow::format_err!("missing `Wechatpay-Timestamp` header"))?
        .to_str()?;
    let nonce_str = res
        .headers()
        .get("Wechatpay-Nonce")
        .ok_or_else(|| anyhow::format_err!("missing `Wechatpay-Nonce` header"))?
        .to_str()?;

    let mut msg = BytesMut::new();
    msg.put_slice(timestamp.as_bytes());
    msg.put_u8(b'\n');
    msg.put_slice(nonce_str.as_bytes());
    msg.put_u8(b'\n');
    let body = res.text().await?;
    msg.put_slice(body.as_bytes());
    msg.put_u8(b'\n');

    let verifying_key = VerifyingKey::<Sha256>::new(public_key.clone());
    let signature = Signature::try_from(signature.as_slice())?;
    verifying_key.verify(&msg, &signature)?;

    let new_res = builder.body(body)?;
    Ok(new_res.into())
}
