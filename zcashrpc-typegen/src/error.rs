pub type TypegenResult<T> = Result<T, TypegenError>;
use derive_more::From as FromWrapped;

#[derive(Debug, FromWrapped)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TypegenError {
    Filesystem(FSError),
    Json(JsonError),
    Annotation(QuizfaceAnnotationError),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct FSError {
    message: String,
    location: Box<std::path::Path>,
}

impl FSError {
    pub(crate) fn from_io_error(
        location: &std::path::Path,
    ) -> Box<dyn Fn(std::io::Error) -> Self + '_> {
        Box::new(move |err: std::io::Error| Self {
            message: format!("{:?}", err.kind()),
            location: Box::from(location),
        })
    }
}

#[derive(Debug)]
pub struct JsonError {
    err: serde_json::Error,
    input: String,
}

impl JsonError {
    pub fn from_serde_json_error(
        err: serde_json::Error,
        input: String,
    ) -> Self {
        Self { err, input }
    }
}

#[derive(Debug, FromWrapped)]
#[cfg_attr(test, derive(PartialEq))]
pub struct QuizfaceAnnotationError {
    pub kind: InvalidAnnotationKind,
    pub location: String,
}

#[derive(Debug, PartialEq)]
pub enum InvalidAnnotationKind {
    Null,
    Bool(bool),
    Number(rust_decimal::Decimal),
    InvalidString(String),
    EmptyArray,
    Insufficient,
}

impl From<serde_json::Value> for InvalidAnnotationKind {
    fn from(val: serde_json::Value) -> Self {
        match val {
            serde_json::Value::Null => Self::Null,
            serde_json::Value::Bool(b) => Self::Bool(b),
            serde_json::Value::Number(n) => Self::Number(
                serde_json::from_str(&n.to_string())
                    .expect(&format!("Invalid number: {}", n)),
            ),
            serde_json::Value::String(s) => Self::InvalidString(s),
            val => panic!("valid serde_json item {} converted to error", val),
        }
    }
}

#[cfg(test)]
mod unit {
    use super::*;
    macro_rules! compare {
            ($a:ident, $b:ident: $($f:ident)|*) => {
                $a.input == $b.input $(&& $a.err.$f() == $b.err.$f())*
            }
        }

    impl PartialEq<Self> for JsonError {
        fn eq(&self, other: &Self) -> bool {
            compare!(self, other: line | column | classify)
        }
    }
    #[test]
    fn test_invalid_annotation() {
        let expected_err = QuizfaceAnnotationError::from((
            InvalidAnnotationKind::Null,
            "foo".to_string(),
        ));
        let err =
            crate::tokenize::value("foo", serde_json::Value::Null).unwrap_err();
        assert_eq!(TypegenError::Annotation(expected_err), err);
    }
    #[test]
    fn test_invalid_terminal() {
        let invalid_label = "NOT A VALID LABEL";
        let expected_invalid =
            serde_json::Value::String(invalid_label.to_string());
        let _iak = InvalidAnnotationKind::from(expected_invalid);
        //let err = crate::quote_terminal()
    }
}
