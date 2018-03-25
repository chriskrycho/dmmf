namespace Ordering

type UnitQuantity = private UnitQuantity of uint32 // between 1 and 1000
module UnitQuantity =
    let create qty =
        if qty < 1u then
            Error "`UnitQuantity` cannot be less than 1"
        else if qty > 1000u then
            Error "`UnitQuantity` cannot be greater than 1000"
        else
            Ok (UnitQuantity qty)

    let value (UnitQuantity qty) = qty
    
    let minimum = UnitQuantity 1u
