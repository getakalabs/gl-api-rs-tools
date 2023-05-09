use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayPal {
    pub id: Option<String>,
    pub create_time: Option<String>,
    pub resource_type: Option<String>,
    pub event_type: Option<String>,
    pub summary: Option<String>,
    pub resource: Option<Resource>,
    pub links: Option<Vec<Link2>>,
    pub event_version: Option<String>,
    pub resource_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    pub custom_id: Option<String>,
    pub quantity: Option<String>,
    pub subscriber: Option<Subscriber>,
    pub create_time: Option<String>,
    pub shipping_amount: Option<ShippingAmount>,
    pub start_time: Option<String>,
    pub update_time: Option<String>,
    pub billing_info: Option<BillingInfo>,
    pub links: Option<Vec<Link>>,
    pub id: Option<String>,
    pub plan_id: Option<String>,
    pub auto_renewal: Option<bool>,
    pub status: Option<String>,
    pub status_update_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subscriber {
    pub name: Option<Name>,
    pub email_address: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Name {
    pub given_name: Option<String>,
    pub surname: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShippingAddress {
    pub name: Option<Name2>,
    pub address: Option<Address>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Name2 {
    pub full_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub admin_area_2: Option<String>,
    pub admin_area_1: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShippingAmount {
    pub currency_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BillingInfo {
    pub outstanding_balance: Option<OutstandingBalance>,
    pub cycle_executions: Option<Vec<CycleExecution>>,
    pub last_payment: Option<LastPayment>,
    pub next_billing_time: Option<String>,
    pub final_payment_time: Option<String>,
    pub failed_payments_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutstandingBalance {
    pub currency_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CycleExecution {
    pub tenure_type: Option<String>,
    pub sequence: Option<i64>,
    pub cycles_completed: Option<i64>,
    pub cycles_remaining: Option<i64>,
    pub current_pricing_scheme_version: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastPayment {
    pub amount: Option<Amount>,
    pub time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Amount {
    pub currency_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub href: Option<String>,
    pub rel: Option<String>,
    pub method: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link2 {
    pub href: Option<String>,
    pub rel: Option<String>,
    pub method: Option<String>,
    #[serde(rename = "encType")]
    pub enc_type: Option<String>,
}