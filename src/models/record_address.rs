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
pub struct RecordAddress {
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(rename = "company2", skip_serializing_if = "Option::is_none")]
    pub company2: Option<String>,
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(
        rename = "globalLocationNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub global_location_number: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "postOfficeBoxCity", skip_serializing_if = "Option::is_none")]
    pub post_office_box_city: Option<String>,
    #[serde(
        rename = "postOfficeBoxNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub post_office_box_number: Option<String>,
    #[serde(
        rename = "postOfficeBoxZipCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub post_office_box_zip_code: Option<String>,
    #[serde(rename = "salutation", skip_serializing_if = "Option::is_none")]
    pub salutation: Option<Salutation>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "street1", skip_serializing_if = "Option::is_none")]
    pub street1: Option<String>,
    #[serde(rename = "street2", skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "titleId", skip_serializing_if = "Option::is_none")]
    pub title_id: Option<String>,
    #[serde(rename = "zipcode", skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,
}

impl RecordAddress {
    pub fn new() -> RecordAddress {
        RecordAddress {
            city: None,
            company: None,
            company2: None,
            country_code: None,
            first_name: None,
            global_location_number: None,
            last_name: None,
            middle_name: None,
            phone_number: None,
            post_office_box_city: None,
            post_office_box_number: None,
            post_office_box_zip_code: None,
            salutation: None,
            state: None,
            street1: None,
            street2: None,
            title: None,
            title_id: None,
            zipcode: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Salutation {
    #[serde(rename = "COMPANY")]
    Company,
    #[serde(rename = "FAMILY")]
    Family,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MRS")]
    Mrs,
    #[serde(rename = "NO_SALUTATION")]
    NoSalutation,
}

impl Default for Salutation {
    fn default() -> Salutation {
        Self::Company
    }
}
