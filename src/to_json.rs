use json::JsonValue;
use json::number::Number;

pub trait ToJSON {
  fn to_json(self: &Self) -> JsonValue;
}

impl ToJSON for bool {
  fn to_json(self: &Self) -> JsonValue {
    JsonValue::Boolean(*self)
  }
}

impl ToJSON for u8 {
  fn to_json(self: &Self) -> JsonValue {
    JsonValue::Number(<Number as From<u8>>::from(*self))
  }
}

impl ToJSON for f32 {
  fn to_json(self: &Self) -> JsonValue {
    JsonValue::Number(<Number as From<f32>>::from(*self))
  }
}

impl ToJSON for f64 {
  fn to_json(self: &Self) -> JsonValue {
    JsonValue::Number(<Number as From<f64>>::from(*self))
  }
}

impl ToJSON for String {
  fn to_json(self: &Self) -> JsonValue {
    JsonValue::from(self.as_str())
  }
}

impl<T> ToJSON for Option<T>
where T: ToJSON {
  fn to_json(self: &Self) -> JsonValue {
    match self {
      Some(v) => v.to_json(),
      None => JsonValue::Null
    }
  }
}

impl<T> ToJSON for Vec<T>
where T: ToJSON {
  fn to_json(self: &Self) -> JsonValue {
    JsonValue::Array(self.iter()
                         .map(|e| e.to_json())
                         .collect())
  }
}