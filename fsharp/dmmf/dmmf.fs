module DMMF

open Ordering

[<EntryPoint>]
let main _argv =
    let _wc = Ordering.WidgetCode.create "W1234"
    let unit =
        match (UnitQuantity.create 23u) with
        | Result.Ok value -> value
        | Result.Error _ -> UnitQuantity.minimum

    let value = UnitQuantity.value unit
    printfn "%i" value
    0 // return an integer exit code
