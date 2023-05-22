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
pub struct Customer {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<crate::models::Address>>,
    #[serde(rename = "amountInsured", skip_serializing_if = "Option::is_none")]
    pub amount_insured: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "annualRevenue", skip_serializing_if = "Option::is_none")]
    pub annual_revenue: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "bankAccounts", skip_serializing_if = "Option::is_none")]
    pub bank_accounts: Option<Vec<crate::models::PartyBankAccount>>,
    #[serde(rename = "birthDate", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<i32>,
    #[serde(rename = "blockNotice", skip_serializing_if = "Option::is_none")]
    pub block_notice: Option<String>,
    #[serde(rename = "blocked", skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(
        rename = "commercialLanguageId",
        skip_serializing_if = "Option::is_none"
    )]
    pub commercial_language_id: Option<String>,
    #[serde(
        rename = "commissionSalesPartners",
        skip_serializing_if = "Option::is_none"
    )]
    pub commission_sales_partners: Option<Vec<crate::models::CommissionSalesPartner>>,
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
    #[serde(rename = "creditLimit", skip_serializing_if = "Option::is_none")]
    pub credit_limit: Option<crate::models::custom_attribute_definition::AttributeType>,
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
    #[serde(rename = "customerNumber", skip_serializing_if = "Option::is_none")]
    pub customer_number: Option<String>,
    #[serde(rename = "customerRatingId", skip_serializing_if = "Option::is_none")]
    pub customer_rating_id: Option<String>,
    #[serde(rename = "customerRatingName", skip_serializing_if = "Option::is_none")]
    pub customer_rating_name: Option<String>,
    #[serde(
        rename = "customerSupplierNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_supplier_number: Option<String>,
    #[serde(rename = "customerTopics", skip_serializing_if = "Option::is_none")]
    pub customer_topics: Option<Vec<crate::models::Entity>>,
    #[serde(
        rename = "defaultHeaderDiscount",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_header_discount: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "defaultHeaderSurcharge",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_header_surcharge: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "deliveryAddressId", skip_serializing_if = "Option::is_none")]
    pub delivery_address_id: Option<String>,
    #[serde(rename = "deliveryBlock", skip_serializing_if = "Option::is_none")]
    pub delivery_block: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "fax", skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "insolvent", skip_serializing_if = "Option::is_none")]
    pub insolvent: Option<bool>,
    #[serde(rename = "insured", skip_serializing_if = "Option::is_none")]
    pub insured: Option<bool>,
    #[serde(rename = "invoiceAddressId", skip_serializing_if = "Option::is_none")]
    pub invoice_address_id: Option<String>,
    #[serde(rename = "invoiceBlock", skip_serializing_if = "Option::is_none")]
    pub invoice_block: Option<bool>,
    #[serde(rename = "invoiceRecipientId", skip_serializing_if = "Option::is_none")]
    pub invoice_recipient_id: Option<String>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "leadSourceId", skip_serializing_if = "Option::is_none")]
    pub lead_source_id: Option<String>,
    #[serde(rename = "leadSourceName", skip_serializing_if = "Option::is_none")]
    pub lead_source_name: Option<String>,
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
    #[serde(rename = "nonStandardTaxId", skip_serializing_if = "Option::is_none")]
    pub non_standard_tax_id: Option<String>,
    #[serde(rename = "oldCustomerNumber", skip_serializing_if = "Option::is_none")]
    pub old_customer_number: Option<String>,
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
    #[serde(rename = "referenceNumber", skip_serializing_if = "Option::is_none")]
    pub reference_number: Option<String>,
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
    #[serde(rename = "salesPartner", skip_serializing_if = "Option::is_none")]
    pub sales_partner: Option<bool>,
    #[serde(
        rename = "salesPartnerDefaultCommissionFix",
        skip_serializing_if = "Option::is_none"
    )]
    pub sales_partner_default_commission_fix:
        Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "salesPartnerDefaultCommissionPercentage",
        skip_serializing_if = "Option::is_none"
    )]
    pub sales_partner_default_commission_percentage:
        Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "salesPartnerDefaultCommissionType",
        skip_serializing_if = "Option::is_none"
    )]
    pub sales_partner_default_commission_type: Option<SalesPartnerDefaultCommissionType>,
    #[serde(rename = "salesStageId", skip_serializing_if = "Option::is_none")]
    pub sales_stage_id: Option<String>,
    #[serde(rename = "salesStageName", skip_serializing_if = "Option::is_none")]
    pub sales_stage_name: Option<String>,
    #[serde(rename = "salutation", skip_serializing_if = "Option::is_none")]
    pub salutation: Option<Salutation>,
    #[serde(rename = "satisfaction", skip_serializing_if = "Option::is_none")]
    pub satisfaction: Option<Satisfaction>,
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
        rename = "useCustomsTariffNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_customs_tariff_number: Option<bool>,
    #[serde(
        rename = "vatRegistrationNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub vat_registration_number: Option<String>,
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

impl Customer {
    pub fn new(party_type: PartyType) -> Customer {
        Customer {
            id: None,
            version: None,
            addresses: None,
            amount_insured: None,
            annual_revenue: None,
            bank_accounts: None,
            birth_date: None,
            block_notice: None,
            blocked: None,
            commercial_language_id: None,
            commission_sales_partners: None,
            company: None,
            company2: None,
            company_size_id: None,
            company_size_name: None,
            contacts: None,
            created_date: None,
            credit_limit: None,
            currency_id: None,
            currency_name: None,
            custom_attributes: None,
            customer_category_id: None,
            customer_category_name: None,
            customer_number: None,
            customer_rating_id: None,
            customer_rating_name: None,
            customer_supplier_number: None,
            customer_topics: None,
            default_header_discount: None,
            default_header_surcharge: None,
            delivery_address_id: None,
            delivery_block: None,
            description: None,
            email: None,
            fax: None,
            first_name: None,
            insolvent: None,
            insured: None,
            invoice_address_id: None,
            invoice_block: None,
            invoice_recipient_id: None,
            last_modified_date: None,
            last_name: None,
            lead_source_id: None,
            lead_source_name: None,
            loss_description: None,
            loss_reason_id: None,
            loss_reason_name: None,
            middle_name: None,
            mobile_phone1: None,
            non_standard_tax_id: None,
            old_customer_number: None,
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
            reference_number: None,
            responsible_user_fixed: None,
            responsible_user_id: None,
            responsible_user_username: None,
            sales_channel: None,
            sales_partner: None,
            sales_partner_default_commission_fix: None,
            sales_partner_default_commission_percentage: None,
            sales_partner_default_commission_type: None,
            sales_stage_id: None,
            sales_stage_name: None,
            salutation: None,
            satisfaction: None,
            sector_id: None,
            sector_name: None,
            shipment_method_id: None,
            shipment_method_name: None,
            tags: None,
            term_of_payment_id: None,
            term_of_payment_name: None,
            title: None,
            title_id: None,
            use_customs_tariff_number: None,
            vat_registration_number: None,
            website: None,
        }
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
pub enum SalesPartnerDefaultCommissionType {
    #[serde(rename = "FIX")]
    Fix,
    #[serde(rename = "FIX_AND_MARGIN")]
    FixAndMargin,
    #[serde(rename = "FIX_AND_REVENUE")]
    FixAndRevenue,
    #[serde(rename = "MARGIN")]
    Margin,
    #[serde(rename = "NO_COMMISSION")]
    NoCommission,
    #[serde(rename = "REVENUE")]
    Revenue,
}

impl Default for SalesPartnerDefaultCommissionType {
    fn default() -> SalesPartnerDefaultCommissionType {
        Self::Fix
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
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Satisfaction {
    #[serde(rename = "NEUTRAL")]
    Neutral,
    #[serde(rename = "SATISFIED")]
    Satisfied,
    #[serde(rename = "UNSATISFIED")]
    Unsatisfied,
}

impl Default for Satisfaction {
    fn default() -> Satisfaction {
        Self::Neutral
    }
}