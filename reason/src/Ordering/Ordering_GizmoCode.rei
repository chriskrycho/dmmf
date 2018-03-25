type gizmoCode = pri | GizmoCode(string);

let create: string => Js.Result.t(gizmoCode, string);

let value: gizmoCode => string;