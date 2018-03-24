module DMMF

open OrderTaking

[<EntryPoint>]
let main _argv =
    let wc = Domain.WidgetCode.create
    let unit =
        match (Domain.UnitQuantity.create 23u) with
        | Result.Ok value -> value
        | Result.Error _ -> Domain.UnitQuantity.minimum

    let value = Domain.UnitQuantity.value unit
    printfn "%i" value
    0 // return an integer exit code
