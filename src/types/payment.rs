use super::User;

#[derive(serde::Deserialize)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(serde::Deserialize)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

#[derive(serde::Deserialize)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub otal_amount: usize,
}

#[derive(serde::Deserialize)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}

#[derive(serde::Deserialize)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: usize,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

#[derive(serde::Deserialize)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: usize,
}

#[derive(serde::Deserialize)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}

#[derive(serde::Deserialize)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: usize,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
}
