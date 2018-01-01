module Domain exposing (..)


type CustomerId
    = CustomerId Int


type WidgetCode
    = WidgetCode String


type UnitQuantity
    = UnitQuantity Int


type KilogramQuantity
    = KilogramQuantity Float


type alias CustomerInfo =
    Never


type alias ShippingAddress =
    Never


type alias BillingAddress =
    Never


type alias OrderLine =
    Never


type alias BillingAmount =
    Never


type alias Order =
    { customer_info : CustomerInfo
    , shipping_address : ShippingAddress
    , billing_address : BillingAddress
    , order_lines : List OrderLine
    , billing_amount : BillingAmount
    }


type alias GizmoCode =
    Never


type ProductCode
    = Widget WidgetCode
    | Gizmo GizmoCode


type OrderQuantity
    = Unit UnitQuantity
    | Kilogram KilogramQuantity


type alias AcknowledgementSet =
    Never


type alias OrderPlaced =
    Never


type alias BillableOrderPlaced =
    Never


type alias PlaceOrderEvents =
    { acknowledgement_set : AcknowledgementSet
    , order_placed : OrderPlaced
    , billable_order_placed : BillableOrderPlaced
    }


type alias UnvalidatedOrder =
    Never


type alias ValidatedOrder =
    Never


type alias ValidationError =
    { field_name : String
    , error_description : String
    }


type ValidationResponse a
    = Task a (List ValidationError)


type alias ValidateOrder =
    UnvalidatedOrder -> ValidationResponse ValidatedOrder


type alias PlaceOrder =
    UnvalidatedOrder -> PlaceOrderEvents


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
