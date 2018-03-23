type widgetCode =
  | WidgetCode(string);

let create = code => {
  let isMatch =
    Js.Re.fromString("W\\d{4}") |> Js.Re.exec(code) |> Js.Option.isSome;
  if (isMatch) {
    Js.Result.Ok(WidgetCode(code));
  } else {
    Js.Result.Error(
      "`WidgetCode` must begin with a 'W' and be followed by 4 digits"
    );
  };
};

let value = (WidgetCode(code)) => code;