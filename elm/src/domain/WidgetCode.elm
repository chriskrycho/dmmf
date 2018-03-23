module Domain.WidgetCode exposing (WidgetCode, create)

import Regex exposing (contains, regex)


type WidgetCode
    = WidgetCode String


create : String -> Result String WidgetCode
create code =
    if contains (regex "W\\d{4}") code then
        Ok (WidgetCode code)
    else
        Err "`WidgetCode` must begin with a 'W' and be followed by 4 digits"


value : WidgetCode -> String
value (WidgetCode code) =
    code
