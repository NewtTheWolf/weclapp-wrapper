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
pub struct PartyEmailAddresses {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "bccAddresses", skip_serializing_if = "Option::is_none")]
    pub bcc_addresses: Option<String>,
    #[serde(rename = "ccAddresses", skip_serializing_if = "Option::is_none")]
    pub cc_addresses: Option<String>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "toAddresses", skip_serializing_if = "Option::is_none")]
    pub to_addresses: Option<String>,
}

impl PartyEmailAddresses {
    pub fn new() -> PartyEmailAddresses {
        PartyEmailAddresses {
            id: None,
            version: None,
            bcc_addresses: None,
            cc_addresses: None,
            created_date: None,
            last_modified_date: None,
            to_addresses: None,
        }
    }
}
