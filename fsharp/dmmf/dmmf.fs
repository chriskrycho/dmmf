module DMMF

[<EntryPoint>]
let main _argv =
    let unit = Domain.UnitQuantity 32
    let (Domain.UnitQuantity value) = unit;
    printfn "%i" value
    0 // return an integer exit code
