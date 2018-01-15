namespace OrderTaking.Domain

// Product code info

type WidgetCode = WidgetCode of string
    // starting with "W" then 4 digits

type GizmoCode = GizmoCode of string
    // starting with "G" then 3 digits

type ProductCode =
    | Widget of WidgetCode
    | Gizmo of GizmoCode


// Order quantity info
type UnitQuantity = UnitQuantity of int
type KilogramQuantity = KilogramQuantity of decimal

type OrderQuantity =
    | Unit of UnitQuantity
    | Kilogram of KilogramQuantity


// Helper -- we'll replace this later
type Undefined = exn


type OrderId = Undefined
type OrderLineId = Undefined
type CustomerId = CustomerId of int

type CustomerInfo = Undefined
type ShippingAddress = Undefined
type BillingAddress = Undefined
type Price = Undefined
type BillingAmount = Undefined

type Order = {
    Id : OrderId
    CustomerInfo : CustomerInfo
    ShippingAddress : ShippingAddress
    BillingAddress : BillingAddress
    OrderLines : OrderLine list
    AmountToBill: BillingAmount
    }

and OrderLine = {
    Id : OrderLineId
    OrderId : OrderId
    ProductCode : ProductCode
    OrderQuantity : OrderQuantity
    Price : Price
    }


type AcknowledgementSent = Undefined
type OrderPlaced = Undefined
type BillableOrderPlaced = Undefined

type PlaceOrderEvents = {
    AcknowledgementSent : AcknowledgementSent
    OrderPlaced : OrderPlaced
    BillableOrderPlaced : BillableOrderPlaced
    }

type PlaceOrderError =
    | ValidationError of ValidationError list

and ValidationError = {
    FieldName : string
    ErrorDescription : string
    }

type UnvalidatedOrderLine = {
    ProductCode : string
    OrderQuantity : int
    }

type UnvalidatedOrder = {
    OrderId : string
    CustomerInfo : string // maybe?
    ShippingAddress : string
    OrderLines : UnvalidatedOrderLine list
    }

type ValidatedOrder = Undefined

type ValidationResponse<'a> = Async<Result<'a,ValidationError list>>

type ValidateOrder =
    UnvalidatedOrder -> ValidationResponse<ValidatedOrder>


type PlaceOrder = UnvalidatedOrder -> Result<PlaceOrderEvents,PlaceOrderError>


type QuoteForm = Undefined
type OrderForm = Undefined

type CategorizedMail =
    | Quote of QuoteForm
    | Order of OrderForm


type ProductCatalog = Undefined
type PricedOrder = Undefined

type CalculatePrices = OrderForm -> ProductCatalog -> PricedOrder

type InvoiceId = InvoiceId of int

type UnpaidInvoice = {
    InvoiceId : InvoiceId
}

type PaidInvoice = {
    InvoiceId : InvoiceId
}

type Invoice =
    | Paid of PaidInvoice
    | Unpaid of UnpaidInvoice
