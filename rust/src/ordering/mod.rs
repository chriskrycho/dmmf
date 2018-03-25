#![allow(dead_code)] // for our sanity while building things!

mod gizmo_code;
mod kilogram_quantity;
mod unit_quantity;
mod widget_code;

use futures::Future;

pub(crate) use self::gizmo_code::GizmoCode;
pub(crate) use self::kilogram_quantity::KilogramQuantity;
pub(crate) use self::unit_quantity::UnitQuantity;
pub(crate) use self::widget_code::WidgetCode;

#[derive(PartialEq, Debug)]
pub enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
}

fn demo_it() {
    let valid = WidgetCode::create("W1234");
    let invalid = WidgetCode::create("wat");

    let unwrapped = match valid {
        Ok(ref code) => code.value(),
        Err(_) => "",
    };
}

#[derive(PartialEq, Debug)]
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
