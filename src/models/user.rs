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
pub struct User {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "birthDate", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<i32>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "customAttributes", skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Vec<crate::models::CustomAttribute>>,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "faxNumber", skip_serializing_if = "Option::is_none")]
    pub fax_number: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "mobilePhoneNumber", skip_serializing_if = "Option::is_none")]
    pub mobile_phone_number: Option<String>,
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "userRoles", skip_serializing_if = "Option::is_none")]
    pub user_roles: Option<Vec<crate::models::OnlyId>>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl User {
    pub fn new(email: String) -> User {
        User {
            id: None,
            version: None,
            birth_date: None,
            created_date: None,
            custom_attributes: None,
            email,
            fax_number: None,
            first_name: None,
            image_id: None,
            last_modified_date: None,
            last_name: None,
            mobile_phone_number: None,
            phone_number: None,
            status: None,
            user_roles: None,
            username: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DEPARTURE")]
    Departure,
    #[serde(rename = "NOT_ACTIVE")]
    NotActive,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
