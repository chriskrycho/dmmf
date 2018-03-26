namespace Ordering

open System.Text.RegularExpressions

type WidgetCode = private WidgetCode of string
module WidgetCode =
    let create (code : string) : Result<WidgetCode, string> =
        if Regex.IsMatch(code, @"W\d{4}") then
            Ok (WidgetCode code)
        else
            Error "`WidgetCode` must begin with a 'W' and be followed by 4 digits"

    let value (WidgetCode code) = code
