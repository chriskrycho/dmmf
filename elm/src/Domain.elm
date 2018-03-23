module Domain exposing (..)

import Domain.WidgetCode exposing (WidgetCode, create)

-- Product code info

type GizmoCode
    = GizmoCode String


type ProductCode
    = Widget WidgetCode
    | Gizmo GizmoCode



-- Order quantity info


type UnitQuantity
    = UnitQuantity Int


type KilogramQuantity
    = KilogramQuantity Float


type OrderQuantity
    = Unit UnitQuantity
    | Kilogram KilogramQuantity


type alias OrderId =
    Never


type alias OrderLineId =
    Never


type CustomerId
    = CustomerId Int


type alias CustomerInfo =
    Never


type alias ShippingAddress =
    Never


type alias BillingAddress =
    Never


type alias Price =
    Never


type alias BillingAmount =
    Never


type alias OrderLine =
    { id : OrderLineId
    , orderId : OrderId
    , productCode : ProductCode
    , orderQuantity : OrderQuantity
    , price : Price
    }


type alias Order =
    { id : OrderId
    , customerInfo : CustomerInfo
    , shippingAddress : ShippingAddress
    , billingAddress : BillingAddress
    , orderLines : List OrderLine
    , billingAmount : BillingAmount
    }


type alias AcknowledgementSet =
    Never


type alias OrderPlaced =
    Never


type alias BillableOrderPlaced =
    Never


type alias PlaceOrderEvents =
    { acknowledgementSet : AcknowledgementSet
    , orderPlaced : OrderPlaced
    , billableOrderPlaced : BillableOrderPlaced
    }


type alias ValidationError =
    { fieldName : String
    , errorDescription : String
    }


type PlaceOrderError
    = ValidationErrors List ValidationError


type alias UnvalidatedOrderLine =
    { productCode : ProductCode
    , orderQuantity : OrderQuantity
    }


type alias UnvalidatedOrder =
    { orderId : OrderId
    , customerInfo : CustomerInfo
    , shippingAddress : ShippingAddress
    , orderLines : List OrderLine
    }


type alias ValidatedOrder =
    Never


type ValidationResponse a
    = Task a (List ValidationError)


type alias ValidateOrder =
    UnvalidatedOrder -> ValidationResponse ValidatedOrder


type alias PlaceOrder =
    UnvalidatedOrder -> Result PlaceOrderError PlaceOrderEvents


type alias QuoteForm =
    Never


type alias OrderForm =
    Never


type CategorizedMail
    = QuoteMail QuoteForm
    | OrderMail OrderForm


type alias ProductCatalog =
    Never


type PricedOrder
    = Never


type alias CalculatePrices =
    OrderForm -> ProductCatalog -> PricedOrder


type InvoiceId
    = InvoiceId Int


type alias PaidInvoice =
    { invoiceId : InvoiceId }


type alias UnpaidInvoice =
    { invoiceId : InvoiceId }


type Invoice
    = Paid PaidInvoice
    | Unpaid UnpaidInvoice
