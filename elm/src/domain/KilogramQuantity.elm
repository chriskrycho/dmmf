module Domain.KilogramQuantity exposing (KilogramQuantity, create, value, minimum)


type KilogramQuantity
    = KilogramQuantity Float


create : Float -> Result String KilogramQuantity
create qty =
    if qty < 0.05 then
        Err "`KilogramQuantity` cannot be less than 0.05"
    else if qty > 1000.0 then
        Err "`KilogramQuantity` cannot be more than 1000.00"
    else
        Ok (KilogramQuantity qty)


value : KilogramQuantity -> Float
value (KilogramQuantity qty) =
    qty


minimum : KilogramQuantity
minimum =
    KilogramQuantity 0.05
