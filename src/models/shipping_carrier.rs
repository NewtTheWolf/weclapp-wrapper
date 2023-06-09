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
pub struct ShippingCarrier {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(
        rename = "internalShippingCarrier",
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_shipping_carrier: Option<InternalShippingCarrier>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "positionNumber", skip_serializing_if = "Option::is_none")]
    pub position_number: Option<i32>,
    #[serde(rename = "trackingUrl", skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
}

impl ShippingCarrier {
    pub fn new(name: String) -> ShippingCarrier {
        ShippingCarrier {
            id: None,
            version: None,
            active: None,
            created_date: None,
            internal_shipping_carrier: None,
            last_modified_date: None,
            name,
            position_number: None,
            tracking_url: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InternalShippingCarrier {
    #[serde(rename = "ANGEL_DE")]
    AngelDe,
    #[serde(rename = "AUSTRIAN_POST_EMS")]
    AustrianPostEms,
    #[serde(rename = "AUSTRIAN_POST_PLUS_INTERNATIONAL")]
    AustrianPostPlusInternational,
    #[serde(rename = "AUSTRIAN_POST_PREMIUM")]
    AustrianPostPremium,
    #[serde(rename = "AUSTRIAN_POST_PREMIUM_SELECT")]
    AustrianPostPremiumSelect,
    #[serde(rename = "AUSTRIAN_POST_RETURNS")]
    AustrianPostReturns,
    #[serde(rename = "AUSTRIAN_POST_STANDARD")]
    AustrianPostStandard,
    #[serde(rename = "CARGO_INTERNATIONAL_EXPRESS")]
    CargoInternationalExpress,
    #[serde(rename = "CARGO_INTERNATIONAL_STANDARD")]
    CargoInternationalStandard,
    #[serde(rename = "DACHSER")]
    Dachser,
    #[serde(rename = "DEUTSCHE_POST_STANDARD")]
    DeutschePostStandard,
    #[serde(rename = "DEUTSCHE_POST_WARENPOST")]
    DeutschePostWarenpost,
    #[serde(rename = "DEUTSCHE_POST_WARENPOST_SIGNATURE")]
    DeutschePostWarenpostSignature,
    #[serde(rename = "DEUTSCHE_POST_WARENPOST_UNTRACKED")]
    DeutschePostWarenpostUntracked,
    #[serde(rename = "DHL_EUROPAKET")]
    DhlEuropaket,
    #[serde(rename = "DHL_EUROPAKET_EV")]
    DhlEuropaketEv,
    #[serde(rename = "DHL_EXPRESS_ONE_DAY")]
    DhlExpressOneDay,
    #[serde(rename = "DHL_EXPRESS_ONE_DAY_EARLY")]
    DhlExpressOneDayEarly,
    #[serde(rename = "DHL_PAKET_CONNECT_EV")]
    DhlPaketConnectEv,
    #[serde(rename = "DHL_PAKET_EV")]
    DhlPaketEv,
    #[serde(rename = "DHL_PAKET_INTERNATIONAL_EV")]
    DhlPaketInternationalEv,
    #[serde(rename = "DHL_RETURNS")]
    DhlReturns,
    #[serde(rename = "DHL_SAME_DAY")]
    DhlSameDay,
    #[serde(rename = "DHL_STANDARD")]
    DhlStandard,
    #[serde(rename = "DHL_WARENPOST")]
    DhlWarenpost,
    #[serde(rename = "DHL_WARENPOST_EV")]
    DhlWarenpostEv,
    #[serde(rename = "DHL_WARENPOST_INTERNATIONAL_EV")]
    DhlWarenpostInternationalEv,
    #[serde(rename = "DPD_EXPRESS_ONE_DAY")]
    DpdExpressOneDay,
    #[serde(rename = "DPD_EXPRESS_ONE_DAY_EARLY")]
    DpdExpressOneDayEarly,
    #[serde(rename = "DPD_RETURNS")]
    DpdReturns,
    #[serde(rename = "DPD_STANDARD")]
    DpdStandard,
    #[serde(rename = "EKOL")]
    Ekol,
    #[serde(rename = "FEDEX_EXPRESS_ONE_DAY_EARLY")]
    FedexExpressOneDayEarly,
    #[serde(rename = "FEDEX_INTERNATIONAL_ECONOMY")]
    FedexInternationalEconomy,
    #[serde(rename = "FEDEX_INTERNATIONAL_PRIORITY")]
    FedexInternationalPriority,
    #[serde(rename = "FEDEX_STANDARD")]
    FedexStandard,
    #[serde(rename = "GLS_EXPRESS_ONE_DAY")]
    GlsExpressOneDay,
    #[serde(rename = "GLS_PICK_AND_SHIP")]
    GlsPickAndShip,
    #[serde(rename = "GLS_RETURNS")]
    GlsReturns,
    #[serde(rename = "GLS_STANDARD")]
    GlsStandard,
    #[serde(rename = "GO_ONE_DAY")]
    GoOneDay,
    #[serde(rename = "HERMES_RETURNS")]
    HermesReturns,
    #[serde(rename = "HERMES_STANDARD")]
    HermesStandard,
    #[serde(rename = "ILOXX_STANDARD")]
    IloxxStandard,
    #[serde(rename = "LIEFERY_EXPRESS_ONE_DAY")]
    LieferyExpressOneDay,
    #[serde(rename = "MGH")]
    Mgh,
    #[serde(rename = "PARCEL_ONE")]
    ParcelOne,
    #[serde(rename = "SEVEN_SENDERS")]
    SevenSenders,
    #[serde(rename = "SITTNAK")]
    Sittnak,
    #[serde(rename = "SWISS_POST")]
    SwissPost,
    #[serde(rename = "TNT_ONE_DAY")]
    TntOneDay,
    #[serde(rename = "TNT_ONE_DAY_EARLY")]
    TntOneDayEarly,
    #[serde(rename = "UPS_EXPEDITED")]
    UpsExpedited,
    #[serde(rename = "UPS_EXPRESS_1200")]
    UpsExpress1200,
    #[serde(rename = "UPS_EXPRESS_ONE_DAY")]
    UpsExpressOneDay,
    #[serde(rename = "UPS_EXPRESS_ONE_DAY_EARLY")]
    UpsExpressOneDayEarly,
    #[serde(rename = "UPS_RETURNS")]
    UpsReturns,
    #[serde(rename = "UPS_STANDARD")]
    UpsStandard,
    #[serde(rename = "ZWECKFORM")]
    Zweckform,
}

impl Default for InternalShippingCarrier {
    fn default() -> InternalShippingCarrier {
        Self::AngelDe
    }
}
