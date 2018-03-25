use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct WidgetCode(String);

impl WidgetCode {
    pub fn create(code: &str) -> Result<WidgetCode, String> {
        let re = Regex::new(r"W\d{4}").expect(r"W\d{4} is a valid regex");
        if re.is_match(code) {
            Ok(WidgetCode(String::from(code)))
        } else {
            Err(String::from(
                "`WidgetCode` must begin with a 'W' and be followed by 4 digits",
            ))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
