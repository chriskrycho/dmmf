extern crate futures;
extern crate regex;

mod domain;

#[cfg(test)]
mod tests {
    use domain;

    #[test]
    fn it_works() {
        assert_eq!(
            domain::GizmoCode::create("123"),
            Err(String::from(
                "`GizmoCode` must begin with a 'G' and be followed by 3 digits"
            ))
        );

        let should_be_valid = domain::GizmoCode::create("G123");
        assert!(should_be_valid.is_ok());
        assert_eq!(should_be_valid.unwrap().value(), "G123",);
    }
}
