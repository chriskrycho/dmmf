type gizmoCode =
  | GizmoCode(string);

/* let create : string => Js.Result.t(gizmoCode, string) */
let create = code => {
  let isMatch =
    Js.Re.fromString("G\\d{3}") |> Js.Re.exec(code) |> Js.Option.isSome;
  isMatch ?
    Js.Result.Ok(GizmoCode(code)) :
    Js.Result.Error(
      "`GizmoCode` must begin with a 'G' and be followed by 3 digits"
    );
};

let value = (GizmoCode(code)) => code;