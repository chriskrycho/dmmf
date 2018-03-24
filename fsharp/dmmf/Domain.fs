namespace OrderTaking.Domain

open Microsoft.FSharp.Data.UnitSystems.SI.UnitNames
open System.Text.RegularExpressions

// Product code info

type WidgetCode = private WidgetCode of string
module WidgetCode =
    let create code =
        if Regex.IsMatch(code, @"W\d{4}") then
            Ok (WidgetCode code)
        else
            Error "`WidgetCode` must begin with a 'W' and be followed by 4 digits"

    let value (WidgetCode code) = code


type GizmoCode = private GizmoCode of string
module GizmoCode = 
    let create code =
        if Regex.IsMatch(code, @"G\d{3}") then
            Ok (GizmoCode code)
        else
            Error "`GizmoCode` must begin with a 'G' and be followed by 3 digits"

    let value (GizmoCode code) = code


type ProductCode =
    | Widget of WidgetCode
    | Gizmo of GizmoCode


type UnitQuantity = private UnitQuantity of uint32 // between 1 and 1000
module UnitQuantity =
    let create qty =
        if qty < 1u then
            Error "`UnitQuantity` cannot be less than 1"
        else if qty > 1000u then
            Error "`UnitQuantity` cannot be greater than 1000"
        else
            Ok (UnitQuantity qty)
    
    let minimum = UnitQuantity 1u

    let value (UnitQuantity qty) = qty


type KilogramQuantity = private KilogramQuantity of decimal<kilogram>
module KilogramQuantity =
    let create qty =
        if qty < 0.05m then
            Error "`KilogramQuantity` cannot be less than 0.05"
        else if qty > 1000.00m then
            Error "`KilogramQuantity` cannot be more than 1000.00"
        else
            let kgs = qty * 1.0m<kilogram>
            Ok (KilogramQuantity kgs)

    let value (KilogramQuantity qty) = qty

    let minimum = KilogramQuantity 0.05m<kilogram>


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
