use crate::enums::{EnumItem, EnumItemExt};
use sea_orm::{DeriveActiveEnum, EnumIter, Iterable};
use serde::{Deserialize, Serialize};

/// 社交平台的类型枚举（对应 Java SocialTypeEnum）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i8", db_type = "Integer")]
pub enum SocialTypeEnum {
    /**
     * Gitee
     *
     * @see <a href="https://gitee.com/api/v5/oauth_doc#/">接入文档</a>
     */
    #[sea_orm(num_value = 10)]
    GitEE,
    /**
     * 钉钉
     *
     * @see <a href="https://developers.dingtalk.com/document/app/obtain-identity-credentials">接入文档</a>
     */
    #[sea_orm(num_value = 20)]
    DingTalk,
    /**
     * 企业微信
     *
     * @see <a href="https://xkcoding.com/2019/08/06/use-justauth-integration-wechat-enterprise.html">接入文档</a>
     */
    #[sea_orm(num_value = 30)]
    WechatEnterprise,
    /**
     * 微信公众平台 - 移动端 H5
     *
     * @see <a href="https://www.cnblogs.com/juewuzhe/p/11905461.html">接入文档</a>
     */
    #[sea_orm(num_value = 31)]
    WechatMp,
    /**
     * 微信开放平台 - 网站应用 PC 端扫码授权登录
     *
     * @see <a href="https://justauth.wiki/guide/oauth/wechat_open/#_2-申请开发者资质认证">接入文档</a>
     */
    #[sea_orm(num_value = 32)]
    WechatOpen,
    /**
     * 微信小程序
     *
     * @see <a href="https://developers.weixin.qq.com/miniprogram/dev/framework/open-ability/login.html">接入文档</a>
     */
    #[sea_orm(num_value = 34)]
    WechatMiniApp,
    /**
     * 微信公众平台 - 关注公众号
     *
     * @see <a href="https://www.cnblogs.com/juewuzhe/p/11905461.html">接入文档</a>
     */
    #[sea_orm(num_value = 36)]
    WechatSubMp,
}

// 实现自定义反序列化
impl<'de> Deserialize<'de> for SocialTypeEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // 先解析为 i8
        let value = i8::deserialize(deserializer)?;

        // 映射到对应枚举值
        match value {
            10 => Ok(SocialTypeEnum::GitEE),
            20 => Ok(SocialTypeEnum::DingTalk),
            30 => Ok(SocialTypeEnum::WechatEnterprise),
            31 => Ok(SocialTypeEnum::WechatMp),
            32 => Ok(SocialTypeEnum::WechatOpen),
            34 => Ok(SocialTypeEnum::WechatMiniApp),
            36 => Ok(SocialTypeEnum::WechatSubMp),
            _ => {
                // 无效值处理
                Err(serde::de::Error::custom(format!(
                    "Invalid SocialTypeEnum value: {}",
                    value
                )))
            }
        }
    }
}
impl EnumItemExt<i8> for SocialTypeEnum {
    fn item(&self) -> EnumItem<i8> {
        match self {
            SocialTypeEnum::GitEE => EnumItem::new(10, "GITEE"),
            SocialTypeEnum::DingTalk => EnumItem::new(20, "DINGTALK"),
            SocialTypeEnum::WechatEnterprise => EnumItem::new(30, "WECHAT_ENTERPRISE"),
            SocialTypeEnum::WechatMp => EnumItem::new(31, "WECHAT_MP"),
            SocialTypeEnum::WechatOpen => EnumItem::new(32, "WECHAT_OPEN"),
            SocialTypeEnum::WechatMiniApp => EnumItem::new(34, "WECHAT_MINI_APP"),
            SocialTypeEnum::WechatSubMp => EnumItem::new(36, "WECHAT_SUB_MP"),
        }
    }

    fn value_items() -> Vec<i8> {
        SocialTypeEnum::iter()
            .map(|item| item.item().value)
            .collect()
    }

    fn items() -> Vec<Self>
    where
        Self: Sized,
    {
        SocialTypeEnum::iter().collect()
    }
}
