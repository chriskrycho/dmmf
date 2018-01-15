#![allow(dead_code)]  // for our sanity while building things!

use futures::Future;

pub struct WidgetCode(String);
pub struct GizmoCode(String);

pub enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
}

pub struct UnitQuantity(i32);
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
    ValidationError(Vec<ValidationError>)
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

#[allow(unused_variables)]  // until we implement this
fn validate_order(unvalidated: UnvalidatedOrder) -> Box<ValidationResponse<ValidatedOrder>> {
    unimplemented!()
}

#[allow(unused_variables)]  // until we implement this
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

fn calculate_prices(orderForm: OrderForm, catalog: ProductCatalog) -> PricedOrder {
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
