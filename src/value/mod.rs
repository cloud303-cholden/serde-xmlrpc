use std::{collections::BTreeMap, convert::{TryInto, TryFrom}};

use iso8601::DateTime;

pub mod de;
pub mod ser;

pub use de::Deserializer;
pub use ser::Serializer;

/// Represents any single valid xmlrpc "Value"
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// A 32-bit signed integer (`<i4>` or `<int>`).
    Int(i32),
    /// A 64-bit signed integer (`<i8>`).
    Int64(i64),
    /// A boolean value (`<boolean>`, 0 == `false`, 1 == `true`).
    Bool(bool),
    /// A string (`<string>`).
    String(String),
    /// A double-precision IEEE 754 floating point number (`<double>`).
    Double(f64),
    /// An ISO 8601 formatted date/time value (`<dateTime.iso8601>`).
    DateTime(DateTime),
    /// Base64-encoded binary data (`<base64>`).
    Base64(Vec<u8>),
    /// A mapping of named values (`<struct>`).
    Struct(BTreeMap<String, Value>),
    /// A list of arbitrary (heterogeneous) values (`<array>`).
    Array(Vec<Value>),
    /// The empty (Unit) value (`<nil/>`).
    Nil,
}

// Public API definitions
impl Value {
    /// Returns an inner struct or array value indexed by `index`.
    ///
    /// Returns `None` if the member doesn't exist or `self` is neither a struct nor an array.
    ///
    /// You can also use Rust's square-bracket indexing syntax to perform this operation if you want
    /// a default value instead of an `Option`. Refer to the top-level [examples](#examples) for
    /// details.
    /*
    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        index.get(self)
    }
    */

    /// If the `Value` is a normal integer (`Value::Int`), returns associated value. Returns `None`
    /// otherwise.
    ///
    /// In particular, `None` is also returned if `self` is a `Value::Int64`. Use [`as_i64`] to
    /// handle this case.
    ///
    /// [`as_i64`]: #method.as_i64
    pub fn as_i32(&self) -> Option<i32> {
        match *self {
            Value::Int(i) => Some(i),
            _ => None,
        }
    }

    /// If the `Value` is an integer, returns associated value. Returns `None` otherwise.
    ///
    /// This works with both `Value::Int` and `Value::Int64`.
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Value::Int(i) => Some(i64::from(i)),
            Value::Int64(i) => Some(i),
            _ => None,
        }
    }

    /// If the `Value` is a boolean, returns associated value. Returns `None` otherwise.
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }

    /// If the `Value` is a string, returns associated value. Returns `None` otherwise.
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Value::String(ref s) => Some(s),
            _ => None,
        }
    }

    /// If the `Value` is a floating point number, returns associated value. Returns `None`
    /// otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Value::Double(d) => Some(d),
            _ => None,
        }
    }

    /// If the `Value` is a date/time, returns associated value. Returns `None` otherwise.
    pub fn as_datetime(&self) -> Option<DateTime> {
        match *self {
            Value::DateTime(dt) => Some(dt),
            _ => None,
        }
    }

    /// If the `Value` is base64 binary data, returns associated value. Returns `None` otherwise.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match *self {
            Value::Base64(ref data) => Some(data),
            _ => None,
        }
    }

    /// If the `Value` is a struct, returns associated map. Returns `None` otherwise.
    pub fn as_struct(&self) -> Option<&BTreeMap<String, Value>> {
        match *self {
            Value::Struct(ref map) => Some(map),
            _ => None,
        }
    }

    /// If the `Value` is an array, returns associated slice. Returns `None` otherwise.
    pub fn as_array(&self) -> Option<&[Value]> {
        match *self {
            Value::Array(ref array) => Some(array),
            _ => None,
        }
    }
}

// Conversions into and from Value

impl From<i32> for Value {
    fn from(other: i32) -> Self {
        Value::Int(other)
    }
}

impl<'a> TryFrom<&'a Value> for &'a i32 {
    type Error = ();
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(i) => Ok(i),
            _ => Err(())
        }
    }
}

impl From<i64> for Value {
    fn from(other: i64) -> Self {
        Value::Int64(other)
    }
}

impl<'a> TryFrom<&'a Value> for &'a i64 {
    type Error = ();
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int64(i) => Ok(i),
            _ => Err(())
        }
    }
}

impl From<bool> for Value {
    fn from(other: bool) -> Self {
        Value::Bool(other)
    }
}

impl<'a> TryFrom<&'a Value> for &'a bool {
    type Error = ();
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(i) => Ok(i),
            _ => Err(())
        }
    }
}

impl From<String> for Value {
    fn from(other: String) -> Self {
        Value::String(other)
    }
}

impl<'a> TryFrom<&'a Value> for &'a str {
    type Error = ();

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Some(val) = value.as_str() {
            Ok(val)
        }else{
            Err(())
        }
    }
}

impl From<&str> for Value {
    fn from(other: &str) -> Self {
        Value::String(other.to_string())
    }
}

impl From<f64> for Value {
    fn from(other: f64) -> Self {
        Value::Double(other)
    }
}

impl<'a> TryFrom<&'a Value> for &'a f64 {
    type Error = ();
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Double(i) => Ok(i),
            _ => Err(())
        }
    }
}

impl From<DateTime> for Value {
    fn from(other: DateTime) -> Self {
        Value::DateTime(other)
    }
}

impl<'a> TryFrom<&'a Value> for &'a DateTime {
    type Error = ();
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::DateTime(i) => Ok(i),
            _ => Err(())
        }
    }
}

impl From<Vec<Value>> for Value {
    fn from(other: Vec<Value>) -> Value {
        Value::Array(other)
    }
}

impl<'a> TryFrom<&'a Value> for &'a Vec<Value> {
    type Error = ();
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(i) => Ok(i),
            _ => Err(())
        }
    }
}

impl From<BTreeMap<String, Value>> for Value {
    fn from(other: BTreeMap<String, Value>) -> Value {
        Value::Struct(other)
    }
}

impl<'a> TryFrom<&'a Value> for &'a BTreeMap<String, Value> {
    type Error = ();
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Struct(i) => Ok(i),
            _ => Err(())
        }
    }
}

impl From<Vec<u8>> for Value {
    fn from(other: Vec<u8>) -> Self {
        Value::Base64(other)
    }
}

impl<'a> TryFrom<&'a Value> for &'a Vec<u8> {
    type Error = ();
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Base64(i) => Ok(i),
            _ => Err(())
        }
    }
}


/// Trait allowing for tuple destructuring iter of Value into a tuple
trait TryCollectValue<'a,T> {
    fn try_collect_value(&'a mut self) -> Option<T>;
}

// Due to lack of variadic traits in Rust we have do this kinda hacky
// See https://stackoverflow.com/questions/38863781/how-to-create-a-tuple-from-a-vector
// and https://gist.github.com/PoignardAzur/aea33f28e2c58ffe1a93b8f8d3c58667

// We define a macro that actually create the implementation of our TryCollect trait for
// a tuple of a given size.
macro_rules! impl_try_collect_value_tuple {
    () => { };
    ($A:ident $($I:ident)*) => {
        impl_try_collect_value_tuple!($($I)*);

        impl<$A: Iterator> TryCollectValue<($A::Item, $($I::Item),*)> for $A {
            fn try_collect_value(&mut self) -> Option<($A::Item, $($I::Item),*)> {
                let r = (try_opt!(self.next()),
                         // hack: we need to use $I in the expasion
                         $({ let a: $I::Item = try_opt!(self.next()); a}),* );
                Some(r)
            }
        }
    }
}
// Helper macro
macro_rules! try_opt {
    ($e:expr) => (match $e { Some(e) => e, None => return None })
}

// impl_try_collect_value_tuple!(A B);

impl<'a, A: TryFrom<&'a Value>, B: TryFrom<&'a Value>, I: Iterator<Item = &'a Value>> TryCollectValue<'a, (A, B)> for I
{
    fn try_collect_value(&'a mut self) -> Option<(A, B)> {
        let a = self.next()?.try_into().ok()?;
        let b = self.next()?.try_into().ok()?;
        Some((a,b))
    }

}


// We then invoke the macro for each size of tuple we want to support,
// convention in Rust is to support up to 12

// impl_try_collect_value_tuple!(A);
// impl_try_collect_value_tuple!(A A);
// impl_try_collect_value_tuple!(A A A);

#[cfg(test)]
mod test {
    use crate::Value;
    use super::TryCollectValue;

    #[test]
    fn test_tuple_destructure() {
        let vec = vec![Value::Int(1), Value::String("2".into(), )];

        let (a, b): (&i32, &str) = vec.iter().try_collect_value().unwrap();
        assert_eq!(*a, 1);
        assert_eq!(b, "2");
    }
}


