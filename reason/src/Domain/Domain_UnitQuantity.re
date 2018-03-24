type unitQuantity =
  | UnitQuantity(int);

let create = qty =>
  if (qty < 1) {
    Js.Result.Error("`UnitQuantity` cannot be less than 1");
  } else if (qty > 1000) {
    Js.Result.Error("`UnitQuantity` cannot be greater than 1000");
  } else {
    Js.Result.Ok(UnitQuantity(qty));
  };

let value = (UnitQuantity(qty)) => qty;

let minimum = UnitQuantity(1);