extern crate futures;
extern crate regex;

mod ordering;

#[cfg(test)]
mod tests {
    use ordering;

    #[test]
    fn it_works() {
        assert_eq!(
            ordering::GizmoCode::create("123"),
            Err(String::from(
                "`GizmoCode` must begin with a 'G' and be followed by 3 digits"
            ))
        );

        let should_be_valid = ordering::GizmoCode::create("G123");
        assert!(should_be_valid.is_ok());
        assert_eq!(should_be_valid.unwrap().value(), "G123",);
    }
}
