module WidgetCode = Ordering_WidgetCode;

module GizmoCode = Ordering_GizmoCode;

module UnitQuantity = Ordering_UnitQuantity;

module KilogramQuantity = Ordering_KilogramQuantity;

open WidgetCode;

open GizmoCode;

open UnitQuantity;

open KilogramQuantity;

type productCode =
  | Widget(widgetCode)
  | Gizmo(gizmoCode);

let valid = WidgetCode.create("W1234");

let invalid = WidgetCode.create("wat");

let unwrapped =
  switch valid {
  | Js.Result.Ok(code) => WidgetCode.value(code)
  | Js.Result.Error(_) => ""
  };

/* Order quantity info */
type orderQuantity =
  | Unit(unitQuantity)
  | Kilogram(kilogramQuantity);

/* Helper -- we'll replace this later */
type undefined = exn;

/*
  Use it throughout where we don't know the type yet. It will
  fail to compile anywhere we try to *use* this.
 */
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
  orderQuantity: int
};

type unvalidatedOrder = {
  orderId: string,
  customerId: string, /* maybe? */
  shippingAddress: string,
  orderLines: list(unvalidatedOrderLine)
};

type validatedOrder = exn;

type validationResponse('a) =
  Js.Promise.t(Js.Result.t('a, list(validationError)));

type validateOrder = unvalidatedOrder => validationResponse(validatedOrder);

type placeOrder =
  unvalidatedOrder => Js.Result.t(placeOrderEvents, placeOrderError);

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