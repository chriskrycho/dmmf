module Domain.UnitQuantity exposing (UnitQuantity, create, value)


type UnitQuantity
    = UnitQuantity Int


create : Int -> Result String UnitQuantity
create qty =
    if qty < 1 then
        Err "`UnitQuantity` cannot be less than 1"
    else if qty > 1000 then
        Err "`UnitQuantity` cannot be greater than 1000"
    else
        Ok (UnitQuantity qty)


value : UnitQuantity -> Int
value (UnitQuantity qty) =
    qty
