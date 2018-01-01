use futures::Future;

pub struct CustomerId(i32);
pub struct WidgetCode(String);
pub struct UnitQuantity(i32);
pub struct KilogramQuantity(f32);

enum Never {}

type CustomerInfo = Never;
type ShippingAddress = Never;
type BillingAddress = Never;
type OrderLine = Never;
type BillingAmount = Never;

struct Order {
    customer_info: CustomerInfo,
    shipping_address: ShippingAddress,
    billing_address: BillingAddress,
    order_lines: Vec<OrderLine>,
    billing_amount: BillingAmount,
}

type GizmoCode = Never;

enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
}

enum OrderQuantity {
    Unit(UnitQuantity),
    Kilogram(KilogramQuantity),
}

type AcknowledgementSent = Never;
type OrderPlaced = Never;
type BillableOrderPlaced = Never;

struct PlaceOrderEvents {
    acknowledgement_sent: AcknowledgementSent,
    order_placed: OrderPlaced,
    billable_order_placed: BillableOrderPlaced,
}

type UnvalidatedOrder = Never;
type ValidatedOrder = Never;

struct ValidationError {
    field_name: String,
    error_description: String,
}

// NOTE: this uses `Future`, and is therefore rather... more complicated.
type ValidationResponse<T> = Future<Item = T, Error = ValidationError>;

type ValidateOrder = FnMut(UnvalidatedOrder) -> ValidationResponse<ValidatedOrder>;

type PlaceOrder = FnMut(UnvalidatedOrder) -> PlaceOrderEvents;

type QuoteForm = Never;
type OrderForm = Never;

enum CategorizedMail {
    Quote(QuoteForm),
    Order(OrderForm),
}

type ProductCatalog = Never;
type PricedOrder = Never;

type CalculatePrices = FnMut(OrderForm, ProductCatalog) -> PricedOrder;
