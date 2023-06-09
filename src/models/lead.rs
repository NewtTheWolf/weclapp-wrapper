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
pub struct Lead {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<crate::models::Address>>,
    #[serde(rename = "annualRevenue", skip_serializing_if = "Option::is_none")]
    pub annual_revenue: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "birthDate", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<i32>,
    #[serde(
        rename = "commercialLanguageId",
        skip_serializing_if = "Option::is_none"
    )]
    pub commercial_language_id: Option<String>,
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(rename = "company2", skip_serializing_if = "Option::is_none")]
    pub company2: Option<String>,
    #[serde(rename = "companySizeId", skip_serializing_if = "Option::is_none")]
    pub company_size_id: Option<String>,
    #[serde(rename = "companySizeName", skip_serializing_if = "Option::is_none")]
    pub company_size_name: Option<String>,
    #[serde(rename = "contacts", skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<crate::models::Contact>>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "currencyId", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(rename = "currencyName", skip_serializing_if = "Option::is_none")]
    pub currency_name: Option<String>,
    #[serde(rename = "customAttributes", skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Vec<crate::models::CustomAttribute>>,
    #[serde(rename = "customerCategoryId", skip_serializing_if = "Option::is_none")]
    pub customer_category_id: Option<String>,
    #[serde(
        rename = "customerCategoryName",
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_category_name: Option<String>,
    #[serde(rename = "deliveryAddressId", skip_serializing_if = "Option::is_none")]
    pub delivery_address_id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "fax", skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "invoiceAddressId", skip_serializing_if = "Option::is_none")]
    pub invoice_address_id: Option<String>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "leadNumber", skip_serializing_if = "Option::is_none")]
    pub lead_number: Option<String>,
    #[serde(rename = "leadRatingId", skip_serializing_if = "Option::is_none")]
    pub lead_rating_id: Option<String>,
    #[serde(rename = "leadRatingName", skip_serializing_if = "Option::is_none")]
    pub lead_rating_name: Option<String>,
    #[serde(rename = "leadSourceId", skip_serializing_if = "Option::is_none")]
    pub lead_source_id: Option<String>,
    #[serde(rename = "leadSourceName", skip_serializing_if = "Option::is_none")]
    pub lead_source_name: Option<String>,
    #[serde(rename = "leadStatus", skip_serializing_if = "Option::is_none")]
    pub lead_status: Option<LeadStatus>,
    #[serde(rename = "leadTopics", skip_serializing_if = "Option::is_none")]
    pub lead_topics: Option<Vec<crate::models::Entity>>,
    #[serde(rename = "lossDescription", skip_serializing_if = "Option::is_none")]
    pub loss_description: Option<String>,
    #[serde(rename = "lossReasonId", skip_serializing_if = "Option::is_none")]
    pub loss_reason_id: Option<String>,
    #[serde(rename = "lossReasonName", skip_serializing_if = "Option::is_none")]
    pub loss_reason_name: Option<String>,
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "mobilePhone1", skip_serializing_if = "Option::is_none")]
    pub mobile_phone1: Option<String>,
    #[serde(rename = "oldLeadNumber", skip_serializing_if = "Option::is_none")]
    pub old_lead_number: Option<String>,
    #[serde(rename = "onlineAccounts", skip_serializing_if = "Option::is_none")]
    pub online_accounts: Option<Vec<crate::models::OnlineAccount>>,
    #[serde(rename = "optIn", skip_serializing_if = "Option::is_none")]
    pub opt_in: Option<bool>,
    #[serde(rename = "optInLetter", skip_serializing_if = "Option::is_none")]
    pub opt_in_letter: Option<bool>,
    #[serde(rename = "optInPhone", skip_serializing_if = "Option::is_none")]
    pub opt_in_phone: Option<bool>,
    #[serde(rename = "optInSms", skip_serializing_if = "Option::is_none")]
    pub opt_in_sms: Option<bool>,
    #[serde(rename = "parentPartyId", skip_serializing_if = "Option::is_none")]
    pub parent_party_id: Option<String>,
    #[serde(rename = "partyType")]
    pub party_type: PartyType,
    #[serde(rename = "paymentMethodId", skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(rename = "paymentMethodName", skip_serializing_if = "Option::is_none")]
    pub payment_method_name: Option<String>,
    #[serde(rename = "personCompany", skip_serializing_if = "Option::is_none")]
    pub person_company: Option<String>,
    #[serde(rename = "personDepartmentId", skip_serializing_if = "Option::is_none")]
    pub person_department_id: Option<String>,
    #[serde(rename = "personRoleId", skip_serializing_if = "Option::is_none")]
    pub person_role_id: Option<String>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "primaryAddressId", skip_serializing_if = "Option::is_none")]
    pub primary_address_id: Option<String>,
    #[serde(rename = "primaryContactId", skip_serializing_if = "Option::is_none")]
    pub primary_contact_id: Option<String>,
    #[serde(
        rename = "responsibleUserFixed",
        skip_serializing_if = "Option::is_none"
    )]
    pub responsible_user_fixed: Option<bool>,
    #[serde(rename = "responsibleUserId", skip_serializing_if = "Option::is_none")]
    pub responsible_user_id: Option<String>,
    #[serde(
        rename = "responsibleUserUsername",
        skip_serializing_if = "Option::is_none"
    )]
    pub responsible_user_username: Option<String>,
    #[serde(rename = "salesChannel", skip_serializing_if = "Option::is_none")]
    pub sales_channel: Option<String>,
    #[serde(rename = "salesStageId", skip_serializing_if = "Option::is_none")]
    pub sales_stage_id: Option<String>,
    #[serde(rename = "salesStageName", skip_serializing_if = "Option::is_none")]
    pub sales_stage_name: Option<String>,
    #[serde(rename = "salutation", skip_serializing_if = "Option::is_none")]
    pub salutation: Option<Salutation>,
    #[serde(rename = "sectorId", skip_serializing_if = "Option::is_none")]
    pub sector_id: Option<String>,
    #[serde(rename = "sectorName", skip_serializing_if = "Option::is_none")]
    pub sector_name: Option<String>,
    #[serde(rename = "shipmentMethodId", skip_serializing_if = "Option::is_none")]
    pub shipment_method_id: Option<String>,
    #[serde(rename = "shipmentMethodName", skip_serializing_if = "Option::is_none")]
    pub shipment_method_name: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "termOfPaymentId", skip_serializing_if = "Option::is_none")]
    pub term_of_payment_id: Option<String>,
    #[serde(rename = "termOfPaymentName", skip_serializing_if = "Option::is_none")]
    pub term_of_payment_name: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "titleId", skip_serializing_if = "Option::is_none")]
    pub title_id: Option<String>,
    #[serde(
        rename = "vatRegistrationNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub vat_registration_number: Option<String>,
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

impl Lead {
    pub fn new(party_type: PartyType) -> Lead {
        Lead {
            id: None,
            version: None,
            addresses: None,
            annual_revenue: None,
            birth_date: None,
            commercial_language_id: None,
            company: None,
            company2: None,
            company_size_id: None,
            company_size_name: None,
            contacts: None,
            created_date: None,
            currency_id: None,
            currency_name: None,
            custom_attributes: None,
            customer_category_id: None,
            customer_category_name: None,
            delivery_address_id: None,
            description: None,
            email: None,
            fax: None,
            first_name: None,
            invoice_address_id: None,
            last_modified_date: None,
            last_name: None,
            lead_number: None,
            lead_rating_id: None,
            lead_rating_name: None,
            lead_source_id: None,
            lead_source_name: None,
            lead_status: None,
            lead_topics: None,
            loss_description: None,
            loss_reason_id: None,
            loss_reason_name: None,
            middle_name: None,
            mobile_phone1: None,
            old_lead_number: None,
            online_accounts: None,
            opt_in: None,
            opt_in_letter: None,
            opt_in_phone: None,
            opt_in_sms: None,
            parent_party_id: None,
            party_type,
            payment_method_id: None,
            payment_method_name: None,
            person_company: None,
            person_department_id: None,
            person_role_id: None,
            phone: None,
            primary_address_id: None,
            primary_contact_id: None,
            responsible_user_fixed: None,
            responsible_user_id: None,
            responsible_user_username: None,
            sales_channel: None,
            sales_stage_id: None,
            sales_stage_name: None,
            salutation: None,
            sector_id: None,
            sector_name: None,
            shipment_method_id: None,
            shipment_method_name: None,
            tags: None,
            term_of_payment_id: None,
            term_of_payment_name: None,
            title: None,
            title_id: None,
            vat_registration_number: None,
            website: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LeadStatus {
    #[serde(rename = "CONVERTED")]
    Converted,
    #[serde(rename = "DISQUALIFIED")]
    Disqualified,
    #[serde(rename = "NEW")]
    New,
    #[serde(rename = "PREQUALIFIED")]
    Prequalified,
    #[serde(rename = "QUALIFIED")]
    Qualified,
}

impl Default for LeadStatus {
    fn default() -> LeadStatus {
        Self::Converted
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PartyType {
    #[serde(rename = "ORGANIZATION")]
    Organization,
    #[serde(rename = "PERSON")]
    Person,
}

impl Default for PartyType {
    fn default() -> PartyType {
        Self::Organization
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
