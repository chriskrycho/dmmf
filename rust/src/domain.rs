#![allow(dead_code)] // for our sanity while building things!

use futures::Future;
use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct WidgetCode(String);

impl WidgetCode {
    pub fn create(code: &str) -> Result<WidgetCode, String> {
        let re = Regex::new(r"W\d{4}").expect(r"W\d{4} is a valid regex");
        if re.is_match(code) {
            Ok(WidgetCode(String::from(code)))
        } else {
            Err(String::from(
                "`WidgetCode` must begin with a 'W' and be followed by 4 digits",
            ))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(PartialEq, Debug)]
pub struct GizmoCode(String);

impl GizmoCode {
    pub fn create(code: &str) -> Result<GizmoCode, String> {
        let re = Regex::new(r"G\d{3}").expect(r"G\d{3} is a valid regex");
        if re.is_match(code) {
            Ok(GizmoCode(String::from(code)))
        } else {
            Err(String::from(
                "`GizmoCode` must begin with a 'G' and be followed by 3 digits",
            ))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

pub enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
}

pub struct UnitQuantity(u32);

impl UnitQuantity {
    pub fn create(qty: u32) -> Result<UnitQuantity, String> {
        match qty {
            0 => Err(String::from("`UnitQuantity` cannot be less than 1")),
            1...1000 => Ok(UnitQuantity(qty)),
            _ => Err(String::from("`UnitQuantity` cannot be greater than 1000")),
        }
    }
}

pub struct KilogramQuantity(f32);

enum OrderQuantity {
    Unit(UnitQuantity),
    Kilogram(KilogramQuantity),
}

// Helper -- we'll replace this later
enum Never {}

type OrderId = Never;
type OrderLineId = Never;
pub struct CustomerId(i32);

type CustomerInfo = Never;
type ShippingAddress = Never;
type BillingAddress = Never;
type Price = Never;
type BillingAmount = Never;

struct OrderLine {
    id: OrderLineId,
    order_id: OrderId,
    product_code: ProductCode,
    order_quantity: OrderQuantity,
    price: Price,
}

struct Order {
    id: OrderId,
    customer_info: CustomerInfo,
    shipping_address: ShippingAddress,
    billing_address: BillingAddress,
    order_lines: Vec<OrderLine>,
    billing_amount: BillingAmount,
}

type AcknowledgementSent = Never;
type OrderPlaced = Never;
type BillableOrderPlaced = Never;

struct PlaceOrderEvents {
    acknowledgement_sent: AcknowledgementSent,
    order_placed: OrderPlaced,
    billable_order_placed: BillableOrderPlaced,
}

struct ValidationError {
    field_name: String,
    error_description: String,
}

enum PlaceOrderError {
    ValidationError(Vec<ValidationError>),
}

struct UnvalidatedOrderLine {
    product_code: String,
    order_quantity: i32,
}

struct UnvalidatedOrder {
    order_id: String,
    customer_info: String, // maybe?
    shipping_address: String,
    order_lines: Vec<UnvalidatedOrderLine>,
}

struct ValidatedOrder {
    order_id: String,
}

// NOTE: this uses `Future`, and is therefore rather... more complicated.
type ValidationResponse<T> = Future<Item = T, Error = ValidationError>;

#[allow(unused_variables)] // until we implement this
fn validate_order(unvalidated: UnvalidatedOrder) -> Box<ValidationResponse<ValidatedOrder>> {
    unimplemented!()
}

#[allow(unused_variables)] // until we implement this
fn place_order(unvalidated: UnvalidatedOrder) -> Result<PlaceOrderEvents, PlaceOrderError> {
    unimplemented!()
}

type VaidateOrder = Fn(UnvalidatedOrder) -> Box<ValidationResponse<ValidatedOrder>>;

type QuoteForm = Never;
type OrderForm = Never;

enum CategorizedMail {
    Quote(QuoteForm),
    Order(OrderForm),
}

type ProductCatalog = Never;
type PricedOrder = Never;

fn calculate_prices(order_form: OrderForm, catalog: ProductCatalog) -> PricedOrder {
    unimplemented!()
}

struct InvoiceId(i32);

struct UnpaidInvoice {
    invoice_id: InvoiceId,
}

struct PaidInvoice {
    invoice_id: InvoiceId,
}

enum Invoice {
    Paid(PaidInvoice),
    Unpaid(UnpaidInvoice),
}
