use json::JsonValue;
use json::number::Number;

pub enum ParseError {
  TypeMismatch,
  UnexpectedNull
}

type JsonResult<T> = Result<T, ParseError>;

pub trait FromJSON {
  fn parse(json: &JsonValue) -> JsonResult<Self>
    where Self: std::marker::Sized;
}

impl FromJSON for f32 {
  fn parse(json: &JsonValue) -> JsonResult<f32> {
    match json {
      JsonValue::Number(num) => Ok(<f32 as From<Number>>::from(*num)),
      JsonValue::Null => Err(ParseError::UnexpectedNull),
      _ => Err(ParseError::TypeMismatch)
    }
  }
}

impl FromJSON for f64 {
  fn parse(json: &JsonValue) -> JsonResult<f64> {
    match json {
      JsonValue::Number(num) => Ok(<f64 as From<Number>>::from(*num)),
      JsonValue::Null => Err(ParseError::UnexpectedNull),
      _ => Err(ParseError::TypeMismatch)
    }
  }
}

impl FromJSON for String {
  fn parse(json: &JsonValue) -> JsonResult<String> {
    match json {
      JsonValue::String(s) => Ok(s.clone()),
      JsonValue::Short(s) => Ok(s.as_str().to_string()),
      JsonValue::Null => Err(ParseError::UnexpectedNull),
      _ => Err(ParseError::TypeMismatch)
    }
  }
}

impl FromJSON for bool {
  fn parse(json: &JsonValue) -> JsonResult<bool> {
    match json {
      JsonValue::Boolean(b) => Ok(*b),
      JsonValue::Null => Err(ParseError::UnexpectedNull),
      _ => Err(ParseError::TypeMismatch)
    }
  }
}

impl<T> FromJSON for Vec<T>
where T: FromJSON {
  fn parse(json: &JsonValue) -> JsonResult<Vec<T>> {
    match json {
      JsonValue::Array(v) => v.iter()
                              .map(|e| FromJSON::parse(e))
                              .collect(),
      JsonValue::Null => Err(ParseError::UnexpectedNull),
      _ => Err(ParseError::TypeMismatch)
    }
  }
}

impl<T> FromJSON for Option<T>
where T: FromJSON {
  fn parse(json: &JsonValue) -> JsonResult<Option<T>> {
    match json {
      JsonValue::Null => Ok(None),
      v => FromJSON::parse(v)
    }
  }
}