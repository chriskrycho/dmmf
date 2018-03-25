type kilogramQuantity = pri | KilogramQuantity(float);

let create: float => Js.Result.t(kilogramQuantity, string);

let value: kilogramQuantity => float;

let minimum: kilogramQuantity;