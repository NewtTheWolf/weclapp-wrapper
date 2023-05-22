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
pub struct EmailAddresses {
    #[serde(rename = "bccAddresses", skip_serializing_if = "Option::is_none")]
    pub bcc_addresses: Option<String>,
    #[serde(rename = "ccAddresses", skip_serializing_if = "Option::is_none")]
    pub cc_addresses: Option<String>,
    #[serde(rename = "toAddresses", skip_serializing_if = "Option::is_none")]
    pub to_addresses: Option<String>,
}

impl EmailAddresses {
    pub fn new() -> EmailAddresses {
        EmailAddresses {
            bcc_addresses: None,
            cc_addresses: None,
            to_addresses: None,
        }
    }
}