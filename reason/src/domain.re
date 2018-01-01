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

type gizmoCode = undefined;

type productCode =
  | Widget(widgetCode)
  | Gizmo(gizmoCode);

type orderQuantity =
  | Unit(unitQuantity)
  | Kilogram(kilogramQuantity);

type acknowledgementSent = undefined;

type orderPlaced = undefined;

type billableOrderPlaced = undefined;

type placeOrderEvents = {
  acknowledgementSent,
  orderPlaced,
  billableOrderPlaced
};

type unvalidatedOrder = exn;

type validatedOrder = exn;

type validationError = {
  fieldName: string,
  errorDescription: string
};

type validationResponse('a) =
  Js.Promise.t(Js.Result.t('a, list(validationError)));

type validateOrder = unvalidatedOrder => validationResponse(validatedOrder);

type placeOrder = unvalidatedOrder => placeOrderEvents;

type quoteForm = exn;

type orderForm = exn;

type categorizedMail =
  | Quote(quoteForm)
  | Order(orderForm);

type productCatalog = exn;

type pricedOrder = exn;

type calculatePrices = (orderForm, productCatalog) => pricedOrder;