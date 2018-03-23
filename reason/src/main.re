let g = Domain.WidgetCode.create("yay");

let f = Domain.UnitQuantity.create(12);

let h =
  switch f {
  | Js.Result.Ok(v) => Domain.UnitQuantity.value(v)
  | Js.Result.Error(_) => 0
  };