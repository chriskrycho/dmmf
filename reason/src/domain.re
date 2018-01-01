/* Product code info */
type widgetCode =
  | WidgetCode(string);

type gizmoCode =
  | GizmoCode(string);

type productCode =
  | Widget(widgetCode)
  | Gizmo(gizmoCode);

/* Order quantity info */
type kilogramQuantity =
  | KilogramQuantity(float);

type unitQuantity =
  | UnitQuantity(int);

type orderQuantity =
  | Unit(unitQuantity)
  | Kilogram(kilogramQuantity);

/* Helper -- we'll replace this later */
type undefined = exn;

type orderId = undefined;
type orderLineId = undefined;

type customerId =
  | CustomerId(int);

type customerInfo = undefined;

type shippingAddress = undefined;

type billingAddress = undefined;

type price = undefined;

type billingAmount = undefined;

type orderLine = {
  id: orderLineId,
  orderId,
  productCode,
  orderQuantity,
  price
};

type order = {
  id: orderId,
  customerInfo,
  shippingAddress,
  billingAddress,
  orderLine: list(orderLine),
  billingAmount
};

type acknowledgementSent = undefined;

type orderPlaced = undefined;

type billableOrderPlaced = undefined;

type placeOrderEvents = {
  acknowledgementSent,
  orderPlaced,
  billableOrderPlaced
};

type validationError = {
  fieldName: string,
  errorDescription: string
};

type placeOrderError =
  | ValidationError(validationError);

type unvalidatedOrderLine = {
  productCode: string,
  orderQuantity: int,
};

type unvalidatedOrder = {
  orderId: string,
  customerId: string, /* maybe? */
  shippingAddress: string,
  orderLines: list(unvalidatedOrderLine),
};

type validatedOrder = exn;

type validationResponse('a) =
  Js.Promise.t(Js.Result.t('a, list(validationError)));

type validateOrder = unvalidatedOrder => validationResponse(validatedOrder);

type placeOrder = unvalidatedOrder => Js.Result.t(placeOrderEvents, placeOrderError);

type quoteForm = exn;

type orderForm = exn;

type categorizedMail =
  | Quote(quoteForm)
  | Order(orderForm);

type productCatalog = exn;

type pricedOrder = exn;

type calculatePrices = (orderForm, productCatalog) => pricedOrder;

type invoiceId =
  | InvoiceId(int);

type unpaidInvoice = {invoiceId};

type paidInvoice = {invoiceId};

type invoice =
  | Paid(paidInvoice)
  | Unpaid(unpaidInvoice);