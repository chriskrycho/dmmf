namespace Domain

open System.Text.RegularExpressions

type GizmoCode = private GizmoCode of string
module GizmoCode = 
    let create code =
        if Regex.IsMatch(code, @"G\d{3}") then
            Ok (GizmoCode code)
        else
            Error "`GizmoCode` must begin with a 'G' and be followed by 3 digits"

    let value (GizmoCode code) = code

