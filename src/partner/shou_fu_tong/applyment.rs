//! 二级商户进件相关接口。
pub mod apply_query;
pub mod settlement;
pub mod utils;

use crate::client::{WechatPayClient, BASE_URL};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub use apply_query::query_applyment_by_out_request_no;
pub use settlement::{modify_settlement, query_settlement, query_settlement_modify};

/// 二级商户进件-申请。
/// 通过该接口提交二级商户进件申请。
/// 参见 <https://pay.weixin.qq.com/doc/v3/partner/4012713017>
pub(super) async fn submit(
    wxpay: &WechatPayClient,
    sub_merchant: &SubMerchantApplication,
) -> Result<ApplymentResponse> {
    let url = "ecommerce/applyments/";
    let url = format!("{}/{}", BASE_URL, url);

    let req = wxpay.client.post(&url).json(sub_merchant).build()?;
    let res = wxpay.execute(req, None).await?;
    let res = res.json().await?;
    Ok(res)
}

/// 提交进件Response
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplymentResponse {
    pub applyment_id: u64,
    pub out_request_no: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SubMerchantApplication {
    pub out_request_no: String,              // 业务申请编号
    pub organization_type: OrganizationType, // 主体类型

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finance_institution: Option<bool>, // 是否金融机构

    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_license_info: Option<BusinessLicenseInfo>, // 营业执照/登记证书信息

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finance_institution_info: Option<FinanceInstitutionInfo>, // 金融机构许可证信息

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_holder_type: Option<IdHolderType>, // 证件持有人类型

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_doc_type: Option<IdDocType>, // 经营者/法人证件类型

    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorize_letter_copy: Option<String>, // 法定代表人说明函

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_card_info: Option<IdCardInfo>, // 经营者/法人身份证信息

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_doc_info: Option<IdDocInfo>, // 经营者/法人其他类型证件信息

    ///【经营者/法人是否为受益人】 主体类型为企业时，需要填写：
    /// 1、 若经营者/法人是最终受益人，则填写：true。
    /// 2、若经营者/法人不是最终受益人，则填写：false。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>, // 经营者、法人是否为受益人

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubo_info_list: Option<Vec<UboInfo>>, // 最终受益人信息列表
    pub account_info: AccountInfo,        // 结算账户信息
    pub contact_info: ContactInfo,        // 超级管理员信息
    pub sales_scene_info: SalesSceneInfo, // 经营场景信息

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_info: Option<SettlementInfo>, // 结算规则

    /// UTF-8格式，中文占3个字节，即最多21个汉字长度。将在支付完成页向买家展示，需与商家的实际售卖商品相符 。
    pub merchant_shortname: String, // 商户简称

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifications: Option<Vec<String>>, // 特殊资质图片media_id

    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_addition_pics: Option<Vec<String>>, // 补充材料图片media_id

    /// 若主体为“个人卖家”，该字段必传，则需填写描述“ 该商户已持续从事电子商务经营活动满6个月，且期间经营收入累计超过20万元。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_addition_desc: Option<String>, // 补充材料说明
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SettlementInfo {
    /// 结算规则ID
    /// 1、选填，请选择二级商户的结算规则ID，需匹配电商平台开通工具箱选择的费率档位，详细参见电商二级商户结算规则对照表；
    /// 2、若电商平台未传入，将默认选择0.6%费率对应的结算规则id；
    pub settlement_id: Option<i32>,

    /// 所属行业
    /// 1、选填，请填写二级商户所属的行业名称，映射特殊资质要求，详细参见电商二级商户结算规则对照表；
    /// 2、若电商平台未传入，将默认填写无需特殊资质的行业名称；
    pub qualification_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SalesSceneInfo {
    /// 店铺名称
    /// 请填写店铺全称。
    pub store_name: String,

    /// 店铺链接
    /// 1、店铺二维码or店铺链接二选一必填。
    /// 2、请填写店铺主页链接，需符合网站规范。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_url: Option<String>,

    /// 店铺二维码
    /// 1、店铺二维码 or 店铺链接二选一必填。
    /// 2、若为电商小程序，可上传店铺页面的小程序二维码。
    /// 3、请填写通过图片上传API预先上传图片生成好的MediaID，仅能上传1张图片。
    pub store_qr_code: Option<String>,

    /// 商家小程序APPID
    /// 1、商户自定义字段，可填写已认证的小程序AppID，认证主体需与二级商户主体一致；
    /// 2、完成入驻后， 系统发起二级商户号与该AppID的绑定（即配置为sub_appid，可在发起支付时传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mini_program_sub_appid: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactInfo {
    /// 超级管理员类型
    /// 1、主体为“小微/个人卖家 ”，可选择：65-经营者/法人。
    /// 2、主体为“个体工商户/企业/政府机关/事业单位/社会组织”，可选择：65-经营者/法人、66- 经办人。 （经办人：经商户授权办理微信支付业务的人员）。
    pub contact_type: String,

    /// 超级管理员姓名
    /// 1、若管理员类型为“法人”，则该姓名需与法人身份证姓名一致。
    /// 2、若管理员类型为“经办人”，则可填写实际负责人的姓名。
    /// ... (其他约束)
    pub contact_name: String,

    /// 超级管理员证件类型
    /// 当超级管理员类型是经办人时，请上传超级管理员证件类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id_doc_type: Option<IdDocType>,

    /// 超级管理员证件号码
    /// 1、若超级管理员类型为法人，则该身份证号码需与法人身份证号码一致。若超级管理员类型为经办人，则可填写实际经办人的身份证号码。
    /// ... (其他约束)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id_card_number: Option<String>,

    /// 超级管理员证件正面照片
    /// 1、当超级管理员类型是经办人时，请上传超级管理员证件的正面照片。
    /// ... (其他约束)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id_doc_copy: Option<String>,

    /// 超级管理员证件反面照片
    /// 1、当超级管理员类型是经办人时，请上传超级管理员证件的反面照片。
    /// ... (其他约束)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id_doc_copy_back: Option<String>,

    /// 超级管理员证件有效期开始时间
    /// 1、当超级管理员类型是经办人时，请上传证件有效期开始时间。
    /// ... (其他约束)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id_doc_period_begin: Option<String>,

    /// 超级管理员证件有效期结束时间
    /// 1、当超级管理员类型是经办人时，请上传证件有效期结束时间。
    /// ... (其他约束)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id_doc_period_end: Option<String>,

    /// 业务办理授权函
    /// 1、当超级管理员类型是经办人时，请上传业务办理授权函。
    /// ... (其他约束)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_authorization_letter: Option<String>,

    /// 超级管理员手机
    /// 1、前后不能有空格、制表符、换行符
    /// ... (其他约束)
    pub mobile_phone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountInfo {
    /// 账户类型
    /// 1、若主体为企业/政府机关/事业单位/社会组织，可填写：74-对公账户。
    /// 2、主体为小微/个人卖家，可选择：75-对私账户。
    /// 3、若主体为个体工商户，可填写：74-对公账户、75-对私账户。
    pub bank_account_type: String,

    /// 开户银行
    /// 对私银行调用：查询支持个人业务的银行列表API
    /// 对公银行调用：查询支持对公业务的银行列表API。
    pub account_bank: String,

    /// 开户名称
    /// 1、选择经营者个人银行卡时，开户名称必须与身份证姓名一致。
    /// 2、选择对公账户时，开户名称必须与营业执照上的“商户名称”一致。
    /// 3、该字段需要使用微信支付公钥加密（推荐），请参考获取微信支付公钥ID说明以及微信支付公钥加密敏感信息指引，也可以使用微信支付平台证书公钥加密，参考获取平台证书序列号、平台证书加密敏感信息指引
    pub account_name: String,

    /// 开户银行省市编码
    /// 至少精确到市，详细参见省市区编号对照表。
    /// 注：仅当省市区编号对照表中无对应的省市区编号时，可向上取该银行对应市级编号或省级编号。
    pub bank_address_code: String,

    /// 开户银行联行号
    /// 1、根据开户银行查询接口中的“是否需要填写支行”判断是否需要填写。如为其他银行，开户银行全称（含支行）和开户银行联行号二选一。
    /// 2、详细需调用查询支行列表API查看查询结果。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_branch_id: Option<String>,

    /// 开户银行全称 （含支行）
    /// 1、根据开户银行查询接口中的“是否需要填写支行”判断是否需要填写。如为其他银行，开户银行全称（含支行）和开户银行联行号二选一。
    /// 2、详细需调用查询支行列表API查看查询结果。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,

    /// 银行账号
    /// 1、数字，长度遵循系统支持的对公/对私卡号长度要求表。
    /// 2、该字段需要使用微信支付公钥加密（推荐），请参考获取微信支付公钥ID说明以及微信支付公钥加密敏感信息指引，也可以使用微信支付平台证书公钥加密，参考获取平台证书序列号、平台证书加密敏感信息指引
    pub account_number: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UboInfo {
    /// 证件类型
    /// 请填写受益人的证件类型。
    /// 枚举值：
    /// IDENTIFICATION_TYPE_MAINLAND_IDCARD: 中国大陆居民-身份证
    /// IDENTIFICATION_TYPE_OVERSEA_PASSPORT: 其他国家或地区居民-护照
    /// IDENTIFICATION_TYPE_HONGKONG: 中国香港居民--来往内地通行证
    /// IDENTIFICATION_TYPE_MACAO: 中国澳门居民--来往内地通行证
    /// IDENTIFICATION_TYPE_TAIWAN: 中国台湾居民--来往大陆通行证
    /// IDENTIFICATION_TYPE_FOREIGN_RESIDENT: 外国人居留证
    /// IDENTIFICATION_TYPE_HONGKONG_MACAO_RESIDENT: 港澳居民证
    /// IDENTIFICATION_TYPE_TAIWAN_RESIDENT: 台湾居民证
    pub ubo_id_doc_type: Option<IdDocType>,

    /// 证件正面照片
    /// 1、请上传受益人证件的正面照片。
    /// 2、若证件类型为身份证，请上传人像面照片。
    /// 3、正面拍摄、清晰、四角完整、无反光或遮挡；不得翻拍、截图、镜像、PS。
    /// 4、请上传彩色照片or彩色扫描件，复印件需加盖公章鲜章，可添加“微信支付”相关水印（如微信支付认证），见【指引文档】
    /// 5、可上传1张图片，请填写通过图片上传API预先上传图片生成好的MediaID 。
    pub ubo_id_doc_copy: Option<String>,

    /// 证件反面照片
    /// 1、请上传受益人证件的反面照片。
    /// 2、若证件类型为身份证，请上传国徽面照片。
    /// 3、若证件类型为护照，无需上传反面照片。
    /// 4、正面拍摄、清晰、四角完整、无反光或遮挡；不得翻拍、截图、镜像、PS。
    /// 5、请上传彩色照片or彩色扫描件，复印件需加盖公章鲜章，可添加“微信支付”相关水印（如微信支付认证），见【指引文档】
    /// 6、可上传1张图片，请填写通过图片上传API预先上传图片生成好的MediaID。
    pub ubo_id_doc_copy_back: Option<String>,

    /// 证件姓名
    /// 1、长度为2-100个字符
    /// 2、前后不能有空格、制表符、换行符
    /// 3、不能仅含数字、特殊字符
    /// 4、仅能填写数字、英文字母、汉字及特殊字符
    /// 5、该字段需要使用微信支付公钥加密（推荐），请参考获取微信支付公钥ID说明以及微信支付公钥加密敏感信息指引，也可以使用微信支付平台证书公钥加密，参考获取平台证书序列号、平台证书加密敏感信息指引
    pub ubo_id_doc_name: Option<String>,

    /// 证件号码
    /// 1、可传身份证、来往内地通行证、来往大陆通行证、护照等证件号码，号码规范如下：
    /// 身份证（限中国大陆居民)：17位数字+1位数字|X
    /// 护照（限境外人士）：4-15位 数字|字母|连字符
    /// 中国香港居民--来往内地通行证：H/h开头+8或10位数字/字母
    /// 中国澳门居民--来往内地通行证：M/m开头+8或10位数字/字母
    /// 中国台湾居民--来往大陆通行证：8位数字或10位数字
    /// 外国人居留证：15位 数字|字母
    /// 港澳居住证/台湾居住证：17位数字+1位数字|X
    /// 2、该字段需要使用微信支付公钥加密（推荐），请参考获取微信支付公钥ID说明以及微信支付公钥加密敏感信息指引，也可以使用微信支付平台证书公钥加密，参考获取平台证书序列号、平台证书加密敏感信息指引
    pub ubo_id_doc_number: Option<String>,

    /// 证件居住地址
    /// 1、请按照身份证住址填写，如广东省深圳市南山区xx路xx号xx室
    /// 2、长度为4-128个字符
    /// 3、前后不能有空格、制表符、换行符
    /// 4、不能仅含数字、特殊字符
    /// 5、仅能填写数字、英文字母、汉字及特殊字符
    /// 6、仅支持utf-8格式
    /// 7、 该字段需要使用微信支付公钥加密（推荐），请参考获取微信支付公钥ID说明以及微信支付公钥加密敏感信息指引，也可以使用微信支付平台证书公钥加密，参考获取平台证书序列号、平台证书加密敏感信息指引
    pub ubo_id_doc_address: Option<String>,

    /// 证件有效期开始时间
    /// 1、日期格式应满足合法的YYYY-MM-DD格式
    /// 2、开始时间不能小于1900-01-01
    /// 3、开始时间不能大于等于当前日期
    pub ubo_id_doc_period_begin: Option<String>,

    /// 证件有效期结束时间
    /// 1、日期格式应满足合法的YYYY-MM-DD格式或长期
    /// 2、结束时间大于开始时间
    pub ubo_id_doc_period_end: Option<String>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum IdDocType {
    IDENTIFICATION_TYPE_MAINLAND_IDCARD,
    IDENTIFICATION_TYPE_OVERSEA_PASSPORT,
    IDENTIFICATION_TYPE_HONGKONG,
    IDENTIFICATION_TYPE_MACAO,
    IDENTIFICATION_TYPE_TAIWAN,
    IDENTIFICATION_TYPE_FOREIGN_RESIDENT,
    IDENTIFICATION_TYPE_HONGKONG_MACAO_RESIDENT,
    IDENTIFICATION_TYPE_TAIWAN_RESIDENT,
}

/// 经营者/法人其他类型证件信息
#[derive(Serialize, Deserialize, Debug)]
pub struct IdDocInfo {
    /// 证件姓名
    /// 1、请填写经营者/法定姓名
    /// 2、长度为2-100个字符
    /// 3、前后不能有空格、制表符、换行符
    /// 4、不能仅含数字、特殊字符
    /// 5、仅能填写数字、英文字母、汉字及特殊字符
    /// 6、该字段需要使用微信支付公钥加密（推荐），请参考获取微信支付公钥ID说明以及微信支付公钥加密敏感信息指引，也可以使用微信支付平台证书公钥加密，参考获取平台证书序列号、平台证书加密敏感信息指引
    pub id_doc_name: String,

    /// 证件号码
    /// 1、请填写经营者/法定代表人的证件号码。
    /// 护照（限境外人士）：4-15位 数字|字母|连字符
    /// 中国香港居民--来往内地通行证：H/h开头+8或10位数字/字母
    /// 中国澳门居民--来往内地通行证：M/m开头+8或10位数字/字母
    /// 中国台湾居民--来往大陆通行证：8位数字或10位数字
    /// 外国人居留证：15位 数字|字母
    /// 港澳居住证/台湾居住证：17位数字+1位数字|X
    /// 2、该字段需要使用微信支付公钥加密（推荐），请参考获取微信支付公钥ID说明以及微信支付公钥加密敏感信息指引，也可以使用微信支付平台证书公钥加密，参考获取平台证书序列号、平台证书加密敏感信息指引
    pub id_doc_number: String,

    /// 证件正面照片
    /// 1、证件类型不为“身份证”时，上传证件正面照片。
    /// 2、可上传1张图片，请填写通过图片上传API预先上传图片生成好的MediaID。
    /// 3、正面拍摄、清晰、四角完整、无反光或遮挡；不得翻拍、截图、镜像、PS。
    /// 4、请上传彩色照片or彩色扫描件or复印件（需加盖公章鲜章），可添加“微信支付”相关水印（如微信支付认证），见【指引文档】。
    pub id_doc_copy: String,

    /// 证件反面照片
    /// 1、若证件类型为来往通行证、外国人居留证、港澳居住证、台湾居住证时，上传证件反面照片。
    /// 2、若证件类型为护照，无需上传反面照片
    /// 3、可上传1张图片，请填写通过图片上传API预先上传图片生成好的MediaID。
    /// 4、正面拍摄、清晰、四角完整、无反光或遮挡；不得翻拍、截图、镜像、PS。
    /// 5、请上传彩色照片or彩色扫描件or复印件（需加盖公章鲜章），可添加“微信支付”相关水印（如微信支付认证），见【指引文档】。
    pub id_doc_copy_back: Option<String>,

    /// 证件开始日期
    /// 1、日期格式应满足合法的YYYY-MM-DD格式
    /// 2、开始时间不能小于1900-01-01
    /// 3、开始时间不能大于等于当前日期
    pub doc_period_begin: String,

    /// 证件结束日期
    /// 1、日期格式应满足合法的YYYY-MM-DD格式或长期
    /// 2、结束时间大于开始时间
    pub doc_period_end: String,
}

/// 经营者/法人身份证信息
#[derive(Debug, Serialize, Deserialize)]
pub struct IdCardInfo {
    /// 身份证人像面照片
    /// 请上传个体户经营者/法人的身份证人像面照片。
    /// 请填写通过图片上传API预先上传图片生成好的MediaID。
    pub id_card_copy: String,

    /// 身份证国徽面照片
    /// 请上传个体户经营者/法定代表人的身份证国徽面照片。
    /// 请填写通过图片上传API预先上传图片生成好的MediaID。
    pub id_card_national: String,

    /// 身份证姓名
    /// 请填写个体户经营者/法定代表人对应身份证的姓名。
    /// 长度为2-100个字符，前后不能有空格、制表符、换行符。
    /// 不能仅含数字、特殊字符，仅能填写数字、英文字母、汉字及特殊字符。
    /// 该字段需要使用微信支付公钥加密或平台证书公钥加密。
    pub id_card_name: String,

    /// 身份证号码
    /// 请填写经营者/法定代表人对应身份证的号码。
    /// 格式：7位数字+1位数字|X。
    /// 该字段需要使用微信支付公钥加密或平台证书公钥加密。
    pub id_card_number: String,

    /// 身份证开始时间
    /// 日期格式应满足合法的YYYY-MM-DD格式。
    /// 开始时间不能小于1900-01-01，且不能大于等于当前日期。
    pub id_card_valid_time_begin: String,

    /// 身份证结束时间
    /// 日期格式应满足合法的YYYY-MM-DD格式或“长期”。
    /// 结束时间需大于开始时间。
    pub id_card_valid_time: String,
}

/// 枚举表示证件持有人类型
#[derive(Debug, Serialize, Deserialize)]
pub enum IdHolderType {
    Legal, // 法人
    Super, // 经办人
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinanceInstitutionInfo {
    /// 金融机构类型
    /// 需与营业执照/登记证书一致，参考选择金融机构指引。
    pub finance_type: FinanceType,

    /// 金融机构许可证图片
    /// 根据所属金融机构类型的许可证要求提供。
    pub finance_license_pics: Vec<String>,
}

/// 枚举表示金融机构类型
#[derive(Debug, Serialize, Deserialize)]
pub enum FinanceType {
    /// 商业银行、政策性银行等
    BankAgent,
    /// 非银行类支付机构
    PaymentAgent,
    /// 保险类业务
    Insurance,
    /// 交易所、登记结算类机构等
    TradeAndSettle,
    /// 其他金融业务
    Other,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum OrganizationType {
    #[serde(rename = "2401")]
    MicroMerchant, // 小微商户
    #[serde(rename = "2500")]
    IndividualSeller, // 个人卖家
    #[serde(rename = "4")]
    IndividualBusiness, // 个体工商户
    #[serde(rename = "2")]
    Enterprise, // 企业
    #[serde(rename = "3")]
    Institution, // 事业单位
    #[serde(rename = "2502")]
    GovernmentAgency, // 政府机关
    #[serde(rename = "1708")]
    SocialOrganization, // 社会组织
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BusinessLicenseInfo {
    pub cert_type: Option<String>,       // 证书类型
    pub business_license_copy: String,   // 营业执照扫描件
    pub business_license_number: String, // 营业执照注册号
}
