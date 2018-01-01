module Domain

type CustomerId = CustomerId of int
type WidgetCode = WidgetCode of string
type UnitQuantity = UnitQuantity of int
type KilogramQuantity = KilogramQuantity of decimal

type Undefined = exn

type CustomerInfo = Undefined
type ShippingAddress = Undefined
type BillingAddress = Undefined
type OrderLine = Undefined
type BillingAmount = Undefined

type Order = {
    CustomerInfo : CustomerInfo
    ShippingAddress : ShippingAddress
    BillingAddress : BillingAddress
    OrderLines : OrderLine list
    AmountToBill: BillingAmount
    }

type GizmoCode = Undefined

type ProductCode =
    | Widget of WidgetCode
    | Gizmo of GizmoCode

type OrderQuantity =
    | Unit of UnitQuantity
    | Kilogram of KilogramQuantity


type AcknowledgementSent = exn
type OrderPlaced = exn
type BillableOrderPlaced = exn

type PlaceOrderEvents = {
    AcknowledgementSent : AcknowledgementSent
    OrderPlaced : OrderPlaced
    BillableOrderPlaced : BillableOrderPlaced
    }

type UnvalidatedOrder = exn
type ValidatedOrder = exn

type ValidationResponse<'a> = Async<Result<'a,ValidationError list>>

and ValidationError = {
    FieldName : string
    ErrorDescription : string
    }


type ValidateOrder =
    UnvalidatedOrder -> ValidationResponse<ValidatedOrder>


type PlaceOrder = UnvalidatedOrder -> PlaceOrderEvents


type QuoteForm = exn
type OrderForm = exn

type CategorizedMail =
    | Quote of QuoteForm
    | Order of OrderForm


type ProductCatalog = exn
type PricedOrder = exn

type CalculatePrices = OrderForm -> ProductCatalog -> PricedOrder