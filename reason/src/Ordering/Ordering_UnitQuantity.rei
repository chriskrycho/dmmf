type unitQuantity = pri | UnitQuantity(int);

let create: int => Js.Result.t(unitQuantity, string);

let value: unitQuantity => int;

let minimum: unitQuantity;