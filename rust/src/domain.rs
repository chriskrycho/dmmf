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
