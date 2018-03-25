use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct GizmoCode(String);

impl GizmoCode {
    pub fn create(code: &str) -> Result<GizmoCode, String> {
        let re = Regex::new(r"G\d{3}").expect(r"G\d{3} is a valid regex");
        if re.is_match(code) {
            Ok(GizmoCode(String::from(code)))
        } else {
            Err(String::from(
                "`GizmoCode` must begin with a 'G' and be followed by 3 digits",
            ))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
