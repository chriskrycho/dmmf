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


type GizmoCode
    = Never


type ProductCode
    = Widget WidgetCode
    | Gizmo GizmoCode


type OrderQuantity
    = Unit UnitQuantity
    | Kilogram KilogramQuantity
