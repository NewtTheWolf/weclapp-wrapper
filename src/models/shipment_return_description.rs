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
pub struct ShipmentReturnDescription {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "customerReturn", skip_serializing_if = "Option::is_none")]
    pub customer_return: Option<bool>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(rename = "supplierReturn", skip_serializing_if = "Option::is_none")]
    pub supplier_return: Option<bool>,
}

impl ShipmentReturnDescription {
    pub fn new(name: String) -> ShipmentReturnDescription {
        ShipmentReturnDescription {
            id: None,
            version: None,
            created_date: None,
            customer_return: None,
            last_modified_date: None,
            name,
            position: None,
            supplier_return: None,
        }
    }
}
