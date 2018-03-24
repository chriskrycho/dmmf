type kilogramQuantity =
  | KilogramQuantity(float);

let create = qty =>
  if (qty < 0.05) {
    Js.Result.Error("`KilogramQuantity` cannot be less than 0.05");
  } else if (qty > 1000.0) {
    Js.Result.Error("`KilogramQuantity` cannot be more than 1000.00");
  } else {
    Js.Result.Ok(KilogramQuantity(qty));
  };

/* let value: kilogramQuantity => float; */
let value = (KilogramQuantity(qty)) => qty;

let minimum = KilogramQuantity(0.05);