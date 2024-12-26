//! 二级商户进件相关接口。
use crate::client::{WechatPayClient, BASE_URL};

use anyhow::{bail, Context, Result};
use http::{header::CONTENT_TYPE, HeaderMap};
use reqwest::multipart::{Form, Part};
use rsa::sha2::{Digest, Sha256};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    pub media_id: String,
}

#[derive(Debug, Deserialize)]
pub struct PersonalBankingResponse {
    pub total_count: u32,
    pub count: u32,
    pub data: Option<Vec<BankData>>,
    pub offset: u32,
    pub links: Links,
}

#[derive(Debug, Deserialize)]
pub struct BankData {
    pub bank_alias: String,
    pub bank_alias_code: String,
    pub account_bank: String,
    pub account_bank_code: u32,
    pub need_bank_branch: bool,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    pub next: String,
    pub prev: String,
    #[serde(rename = "self")]
    pub self_link: String,
}

impl WechatPayClient {
    /// 二级商户进件-图片上传。
    /// 通过该接口上传二级商户相关图片，获取media_id。
    /// 参见 <https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter2_1_1.shtml>
    pub async fn upload_image(&self, image: Vec<u8>, filename: &str) -> Result<UploadResponse> {
        const MAX_SIZE: usize = 2 * 1024 * 1024;
        if image.len() > MAX_SIZE {
            bail!("image size too large");
        }

        // check image format is supported
        let ext = filename
            .split('.')
            .last()
            .context("Invalid filename, no extension found")?;
        if !is_supported_image(ext) {
            bail!("Unsupported image format: {}", ext);
        }

        // calculate sha256
        let mut hasher = Sha256::new();
        hasher.update(&image);
        let hash = hasher.finalize();
        let hash = format!("{:x}", hash);
        println!("hash: {}", hash);

        let meta = json!( {
            "filename": filename,
            "sha256": hash
        })
        .to_string();

        let mut json_part_headers = HeaderMap::new();
        json_part_headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        let json_part = Part::text(meta.clone()).headers(json_part_headers);

        let mime = match ext {
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "bmp" => "image/bmp",
            _ => "image/jpeg",
        };

        let form_part = Part::bytes(image.to_vec())
            .file_name(filename.to_string())
            .mime_str(mime)?;

        let form = Form::new().part("meta", json_part).part("file", form_part);

        let url = format!("{}/merchant/media/upload", BASE_URL);
        let req = self.client.post(&url).multipart(form).build()?;
        let res = self.execute(req, Some(meta)).await?;
        let res: UploadResponse = res.json().await?;
        Ok(res)
    }
    pub async fn get_personal_banking(
        &self,
        offset: u32,
        limit: u32,
    ) -> Result<PersonalBankingResponse> {
        let url = format!("{}/capital/capitallhh/banks/personal-banking", BASE_URL);
        let req = self
            .client
            .get(&url)
            .query(&[("offset", offset), ("limit", limit)])
            .build()?;
        let res = self.execute(req, None).await?;
        let res = res.json().await?;
        Ok(res)
    }
}
fn is_supported_image(extension: &str) -> bool {
    let extensions: [&str; 4] = ["jpg", "jpeg", "png", "bmp"];
    extensions.contains(&extension.to_lowercase().as_str())
}
