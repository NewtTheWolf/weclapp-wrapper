/*
 * weclapp api
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * Contact: support@weclapp.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Campaign {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "campaignEndDate", skip_serializing_if = "Option::is_none")]
    pub campaign_end_date: Option<i32>,
    #[serde(rename = "campaignName")]
    pub campaign_name: String,
    #[serde(rename = "campaignNumber", skip_serializing_if = "Option::is_none")]
    pub campaign_number: Option<String>,
    #[serde(rename = "campaignStartDate", skip_serializing_if = "Option::is_none")]
    pub campaign_start_date: Option<i32>,
    #[serde(rename = "campaignType", skip_serializing_if = "Option::is_none")]
    pub campaign_type: Option<CampaignType>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "customAttributes", skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Vec<crate::models::CustomAttribute>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "responsibleUserId", skip_serializing_if = "Option::is_none")]
    pub responsible_user_id: Option<String>,
    #[serde(
        rename = "responsibleUserUsername",
        skip_serializing_if = "Option::is_none"
    )]
    pub responsible_user_username: Option<String>,
}

impl Campaign {
    pub fn new(campaign_name: String) -> Campaign {
        Campaign {
            id: None,
            version: None,
            campaign_end_date: None,
            campaign_name,
            campaign_number: None,
            campaign_start_date: None,
            campaign_type: None,
            created_date: None,
            custom_attributes: None,
            description: None,
            last_modified_date: None,
            responsible_user_id: None,
            responsible_user_username: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CampaignType {
    #[serde(rename = "ADVERTISEMENT")]
    Advertisement,
    #[serde(rename = "BULKMAIL")]
    Bulkmail,
    #[serde(rename = "EMAIL")]
    Email,
    #[serde(rename = "EVENT")]
    Event,
    #[serde(rename = "EXPOSITION")]
    Exposition,
    #[serde(rename = "FAIR")]
    Fair,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "PUBLICRELATION")]
    Publicrelation,
    #[serde(rename = "TELEMARKETING")]
    Telemarketing,
    #[serde(rename = "WEBINAR")]
    Webinar,
}

impl Default for CampaignType {
    fn default() -> CampaignType {
        Self::Advertisement
    }
}
