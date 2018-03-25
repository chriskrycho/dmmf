module Ordering.GizmoCode exposing (GizmoCode)

import Regex exposing (contains, regex)


type GizmoCode
    = GizmoCode String


create : String -> Result String GizmoCode
create code =
    if contains (regex "G\\d{3}") code then
        Ok (GizmoCode code)
    else
        Err "`GizmoCode` must begin with a 'G' and be followed by 3 digits"


value : GizmoCode -> String
value (GizmoCode code) =
    code
