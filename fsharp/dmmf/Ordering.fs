module Ordering

open System
open Ordering

// Product code info

type ProductCode =
    | Widget of WidgetCode
    | Gizmo of GizmoCode

type OrderQuantity =
    | Unit of UnitQuantity
    | Kilogram of KilogramQuantity

    
// Helper -- we'll replace this later
type Undefined = exn

type OrderId = Undefined
type OrderLineId = Undefined

type CustomerId = CustomerId of int

type CustomerInfo = Undefined

type Address = Undefined
type ShippingAddress = Address
type BillingAddress = Address
type UnvalidatedAddress = UnvalidatedAddress of string

type Price = Undefined
type BillingAmount = Undefined

type PricedOrder = {
    Id : OrderId
    CustomerInfo : CustomerInfo
    ShippingAddress : ShippingAddress
    BillingAddress : BillingAddress
    OrderLines : PricedOrderLine list
    AmountToBill: BillingAmount
    }

and PricedOrderLine = {
    Id : OrderLineId
    OrderId : OrderId
    ProductCode : ProductCode
    OrderQuantity : OrderQuantity
    Price : Price
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
    ShippingAddress : UnvalidatedAddress
    OrderLines : UnvalidatedOrderLine list
    }

type ValidatedOrder = {
    OrderId : OrderId
    CustomerInfo : CustomerInfo
    ShippingAddress : ShippingAddress
    BillingAddress : BillingAddress
    }

type ValidationResponse<'a> = Async<Result<'a,ValidationError list>>

type CheckProductCodeExists = ProductCode -> bool
type CheckedAddress = CheckedAddress of UnvalidatedAddress

type InvalidOrder = Undefined

type AddressValidationError = AddressValidationError of string

type CheckAddressExists = UnvalidatedAddress -> Result<CheckedAddress, AddressValidationError>

type ValidateOrder =
    CheckProductCodeExists
        -> CheckAddressExists
        -> UnvalidatedOrder
        -> Result<ValidatedOrder, ValidationError>


type GetProductPrice = ProductCode -> Price

type PriceOrder =
    GetProductPrice
        -> ValidatedOrder
        -> PricedOrder


type EmailAddress = EmailAddress of string
type HtmlString = HtmlString of string

type CreateOrderAcknowledgementLetter =
    PricedOrder -> HtmlString

type OrderAcknowledgement = {
    EmailAddress : EmailAddress
    Letter : HtmlString
    }

type SendResult = Sent | NotSent

type SendOrderAcknowledgment =
    OrderAcknowledgement -> SendResult

type OrderAcknowledgementSent = {
    OrderId : OrderId
    EmailAddress : EmailAddress
    }

type AcknowledgeOrder =
    CreateOrderAcknowledgementLetter
        -> SendOrderAcknowledgment
        -> PricedOrder
        -> OrderAcknowledgementSent option




type Command<'data> = {
    Data : 'data
    Timestamp : DateTime
    UserId : string
    }

// Commands
type PlaceOrder = Command<UnvalidatedOrder>

type ChangeOrder = Command<ValidatedOrder>

type AnyOrder = UnvalidatedOrder | ValidatedOrder

type CancelOrder = Command<AnyOrder>

type OrderTakingCommand =
    | Place of PlaceOrder
    | Change of ChangeOrder
    | Cancel of CancelOrder

type QuoteForm = Undefined
type OrderForm = Undefined

type CategorizedMail =
    | Quote of QuoteForm
    | Order of OrderForm


type ProductCatalog = Undefined

type Order =
    | Unvalidated of UnvalidatedOrder
    | Validated of ValidatedOrder
    | Priced of PricedOrder


type OrderPlaced = PricedOrder

type BillableOrderPlaced = {
    OrderId : OrderId
    BillingAddress : BillingAddress
    AmountToBill : BillingAmount
    }

type PlaceOrderEvent =
    | OrderPlaced of OrderPlaced
    | BillableOrderPlaced of BillableOrderPlaced
    | AcknowledgementSent of OrderAcknowledgementSent


type CreateEvents =
    PricedOrder -> PlaceOrderEvent list


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
