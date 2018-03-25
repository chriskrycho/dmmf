let valid = Ordering.WidgetCode.create("W1234");

let f = Ordering.UnitQuantity.create(12);

let unwrapped =
  switch valid {
  | Js.Result.Ok(v) => Ordering.WidgetCode.value(v)
  | Js.Result.Error(_) => ""
  };