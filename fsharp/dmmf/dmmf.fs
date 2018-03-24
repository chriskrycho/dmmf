module DMMF

open Domain

[<EntryPoint>]
let main _argv =
    let wc = WidgetCode.create
    let unit =
        match (UnitQuantity.create 23u) with
        | Result.Ok value -> value
        | Result.Error _ -> UnitQuantity.minimum

    let value = UnitQuantity.value unit
    printfn "%i" value
    0 // return an integer exit code
