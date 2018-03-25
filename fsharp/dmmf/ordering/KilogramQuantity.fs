namespace Ordering

open Microsoft.FSharp.Data.UnitSystems.SI.UnitNames

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
