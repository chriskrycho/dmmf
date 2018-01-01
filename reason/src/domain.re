type customerId =
  | CustomerId(int);

type widgetCode =
  | WidgetCode(string);

type unitQuantity =
  | UnitQuantity(int);

type kilogramQuantity =
  | KilogramQuantity(float);

type undefined = exn;

type customerInfo = undefined;

type shippingAddress = undefined;

type billingAddress = undefined;

type orderLine = undefined;

type billingAmount = undefined;

type order = {
  customerInfo,
  shippingAddress,
  billingAddress,
  orderLine,
  billingAmount
};

type gizmoCode = exn;

type productCode =
  | Widget(widgetCode)
  | Gizmo(gizmoCode);

type orderQuantity =
  | Unit(unitQuantity)
  | Kilogram(kilogramQuantity);

