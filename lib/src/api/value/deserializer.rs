use std::borrow::Cow;

use super::{Number, Object, Value};
use crate::api::Error;
use serde::{
	de::{
		DeserializeSeed, EnumAccess, Expected, IntoDeserializer, MapAccess, SeqAccess, Unexpected,
		VariantAccess, Visitor,
	},
	forward_to_deserialize_any, Deserialize, Deserializer,
};

impl<'de> Deserializer<'de> for Number {
	type Error = crate::error::Api;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		match self {
			Number::Integer(x) => visitor.visit_i64(x),
			Number::Float(x) => visitor.visit_f64(x),
			Number::Decimal(d) => {
				if let Ok(x) = i64::try_from(d) {
					visitor.visit_i64(x)
				} else if let Ok(x) = f64::try_from(d) {
					visitor.visit_f64(x)
				} else {
					visitor.visit_string(d.to_string())
				}
			}
		}
	}

	forward_to_deserialize_any! {
		bool char str string bytes byte_buf option unit unit_struct
		newtype_struct seq tuple tuple_struct map struct enum identifier
		ignored_any i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64
	}
}

impl<'de> Deserializer<'de> for &'de Number {
	type Error = crate::error::Api;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		match self {
			Number::Integer(x) => visitor.visit_i64(*x),
			Number::Float(x) => visitor.visit_f64(*x),
			Number::Decimal(d) => {
				if let Ok(x) = i64::try_from(*d) {
					visitor.visit_i64(x)
				} else if let Ok(x) = f64::try_from(*d) {
					visitor.visit_f64(x)
				} else {
					visitor.visit_string(d.to_string())
				}
			}
		}
	}

	forward_to_deserialize_any! {
		bool char str string bytes byte_buf option unit unit_struct
		newtype_struct seq tuple tuple_struct map struct enum identifier
		ignored_any i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64
	}
}

// Implementation below largely based from serde_json Value deserializer implementation

fn visit_array<'de, V>(array: Vec<Value>, visitor: V) -> Result<V::Value, Error>
where
	V: Visitor<'de>,
{
	let len = array.len();
	let mut deserializer = SeqDeserializer::new(array);
	let seq = visitor.visit_seq(&mut deserializer)?;
	let remaining = deserializer.iter.len();
	if remaining == 0 {
		Ok(seq)
	} else {
		Err(serde::de::Error::invalid_length(len, &"fewer elements in array"))
	}
}

fn visit_object<'de, V>(object: Object, visitor: V) -> Result<V::Value, Error>
where
	V: Visitor<'de>,
{
	let len = object.len();
	let mut deserializer = MapDeserializer::new(object);
	let map = visitor.visit_map(&mut deserializer)?;
	let remaining = deserializer.iter.len();
	if remaining == 0 {
		Ok(map)
	} else {
		Err(serde::de::Error::invalid_length(len, &"fewer elements in map"))
	}
}

macro_rules! deserialize_number {
	($method:ident) => {
		fn $method<V>(self, visitor: V) -> Result<V::Value, Error>
		where
			V: Visitor<'de>,
		{
			match self {
				Value::Number(n) => n.deserialize_any(visitor),
				_ => Err(self.invalid_type(&visitor)),
			}
		}
	};
}

impl<'de> Deserializer<'de> for Value {
	type Error = Error;

	#[inline]
	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self {
			Value::None => visitor.visit_unit(),
			Value::Bool(v) => visitor.visit_bool(v),
			Value::Number(n) => n.deserialize_any(visitor),
			Value::String(v) => visitor.visit_string(v),
			Value::Array(v) => visit_array(v, visitor),
			Value::Object(v) => visit_object(v, visitor),
			_ => todo!(),
		}
	}

	deserialize_number!(deserialize_i8);
	deserialize_number!(deserialize_i16);
	deserialize_number!(deserialize_i32);
	deserialize_number!(deserialize_i64);
	deserialize_number!(deserialize_i128);
	deserialize_number!(deserialize_u8);
	deserialize_number!(deserialize_u16);
	deserialize_number!(deserialize_u32);
	deserialize_number!(deserialize_u64);
	deserialize_number!(deserialize_u128);
	deserialize_number!(deserialize_f32);
	deserialize_number!(deserialize_f64);

	#[inline]
	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self {
			Value::None => visitor.visit_none(),
			_ => visitor.visit_some(self),
		}
	}

	#[inline]
	fn deserialize_enum<V>(
		self,
		_name: &str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		let (variant, value) = match self {
			Value::Object(value) => {
				let mut iter = value.into_iter();
				let (variant, value) = match iter.next() {
					Some(v) => v,
					None => {
						return Err(serde::de::Error::invalid_value(
							Unexpected::Map,
							&"map with a single key",
						));
					}
				};
				// enums are encoded in json as maps with a single key:value pair
				if iter.next().is_some() {
					return Err(serde::de::Error::invalid_value(
						Unexpected::Map,
						&"map with a single key",
					));
				}
				(variant, Some(value))
			}
			Value::String(variant) => (variant, None),
			other => {
				return Err(serde::de::Error::invalid_type(other.unexpected(), &"string or map"));
			}
		};

		visitor.visit_enum(EnumDeserializer {
			variant,
			value,
		})
	}

	#[inline]
	fn deserialize_newtype_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		visitor.visit_newtype_struct(self)
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self {
			Value::Bool(v) => visitor.visit_bool(v),
			_ => Err(self.invalid_type(&visitor)),
		}
	}

	fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		self.deserialize_string(visitor)
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		self.deserialize_string(visitor)
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self {
			Value::String(v) => visitor.visit_string(v),
			_ => Err(self.invalid_type(&visitor)),
		}
	}

	fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		self.deserialize_byte_buf(visitor)
	}

	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self {
			Value::String(v) => visitor.visit_string(v),
			Value::Array(v) => visit_array(v, visitor),
			_ => Err(self.invalid_type(&visitor)),
		}
	}

	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self {
			Value::None => visitor.visit_unit(),
			_ => Err(self.invalid_type(&visitor)),
		}
	}

	fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		self.deserialize_unit(visitor)
	}

	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self {
			Value::Array(v) => visit_array(v, visitor),
			_ => Err(self.invalid_type(&visitor)),
		}
	}

	fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		self.deserialize_seq(visitor)
	}

	fn deserialize_tuple_struct<V>(
		self,
		_name: &'static str,
		_len: usize,
		visitor: V,
	) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		self.deserialize_seq(visitor)
	}

	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self {
			Value::Object(v) => visit_object(v, visitor),
			_ => Err(self.invalid_type(&visitor)),
		}
	}

	fn deserialize_struct<V>(
		self,
		_name: &'static str,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self {
			Value::Array(v) => visit_array(v, visitor),
			Value::Object(v) => visit_object(v, visitor),
			_ => Err(self.invalid_type(&visitor)),
		}
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		self.deserialize_string(visitor)
	}

	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		drop(self);
		visitor.visit_unit()
	}
}

struct EnumDeserializer {
	variant: String,
	value: Option<Value>,
}

impl<'de> EnumAccess<'de> for EnumDeserializer {
	type Error = Error;
	type Variant = VariantDeserializer;

	fn variant_seed<V>(self, seed: V) -> Result<(V::Value, VariantDeserializer), Error>
	where
		V: DeserializeSeed<'de>,
	{
		let variant = self.variant.into_deserializer();
		let visitor = VariantDeserializer {
			value: self.value,
		};
		seed.deserialize(variant).map(|v| (v, visitor))
	}
}

impl<'de> IntoDeserializer<'de, Error> for Value {
	type Deserializer = Self;

	fn into_deserializer(self) -> Self::Deserializer {
		self
	}
}

impl<'de> IntoDeserializer<'de, Error> for &'de Value {
	type Deserializer = Self;

	fn into_deserializer(self) -> Self::Deserializer {
		self
	}
}

struct VariantDeserializer {
	value: Option<Value>,
}

impl<'de> VariantAccess<'de> for VariantDeserializer {
	type Error = Error;

	fn unit_variant(self) -> Result<(), Error> {
		match self.value {
			Some(value) => Deserialize::deserialize(value),
			None => Ok(()),
		}
	}

	fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
	where
		T: DeserializeSeed<'de>,
	{
		match self.value {
			Some(value) => seed.deserialize(value),
			None => {
				Err(serde::de::Error::invalid_type(Unexpected::UnitVariant, &"newtype variant"))
			}
		}
	}

	fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self.value {
			Some(Value::Array(v)) => {
				if v.is_empty() {
					visitor.visit_unit()
				} else {
					visit_array(v, visitor)
				}
			}
			Some(other) => {
				Err(serde::de::Error::invalid_type(other.unexpected(), &"tuple variant"))
			}
			None => Err(serde::de::Error::invalid_type(Unexpected::UnitVariant, &"tuple variant")),
		}
	}

	fn struct_variant<V>(
		self,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self.value {
			Some(Value::Object(v)) => visit_object(v, visitor),
			Some(other) => {
				Err(serde::de::Error::invalid_type(other.unexpected(), &"struct variant"))
			}
			None => Err(serde::de::Error::invalid_type(Unexpected::UnitVariant, &"struct variant")),
		}
	}
}

struct SeqDeserializer {
	iter: std::vec::IntoIter<Value>,
}

impl SeqDeserializer {
	fn new(vec: Vec<Value>) -> Self {
		SeqDeserializer {
			iter: vec.into_iter(),
		}
	}
}

impl<'de> SeqAccess<'de> for SeqDeserializer {
	type Error = Error;

	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
	where
		T: DeserializeSeed<'de>,
	{
		match self.iter.next() {
			Some(value) => seed.deserialize(value).map(Some),
			None => Ok(None),
		}
	}

	fn size_hint(&self) -> Option<usize> {
		match self.iter.size_hint() {
			(lower, Some(upper)) if lower == upper => Some(upper),
			_ => None,
		}
	}
}

struct MapDeserializer {
	iter: <Object as IntoIterator>::IntoIter,
	value: Option<Value>,
}

impl MapDeserializer {
	fn new(map: Object) -> Self {
		MapDeserializer {
			iter: map.into_iter(),
			value: None,
		}
	}
}

impl<'de> MapAccess<'de> for MapDeserializer {
	type Error = Error;

	fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
	where
		T: DeserializeSeed<'de>,
	{
		match self.iter.next() {
			Some((key, value)) => {
				self.value = Some(value);
				let key_de = MapKeyDeserializer {
					key: Cow::Owned(key),
				};
				seed.deserialize(key_de).map(Some)
			}
			None => Ok(None),
		}
	}

	fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Error>
	where
		T: DeserializeSeed<'de>,
	{
		match self.value.take() {
			Some(value) => seed.deserialize(value),
			None => Err(serde::de::Error::custom("value is missing")),
		}
	}

	fn size_hint(&self) -> Option<usize> {
		match self.iter.size_hint() {
			(lower, Some(upper)) if lower == upper => Some(upper),
			_ => None,
		}
	}
}

fn visit_array_ref<'de, V>(array: &'de [Value], visitor: V) -> Result<V::Value, Error>
where
	V: Visitor<'de>,
{
	let len = array.len();
	let mut deserializer = SeqRefDeserializer::new(array);
	let seq = visitor.visit_seq(&mut deserializer)?;
	let remaining = deserializer.iter.len();
	if remaining == 0 {
		Ok(seq)
	} else {
		Err(serde::de::Error::invalid_length(len, &"fewer elements in array"))
	}
}

fn visit_object_ref<'de, V>(object: &'de Object, visitor: V) -> Result<V::Value, Error>
where
	V: Visitor<'de>,
{
	let len = object.len();
	let mut deserializer = MapRefDeserializer::new(object);
	let map = visitor.visit_map(&mut deserializer)?;
	let remaining = deserializer.iter.len();
	if remaining == 0 {
		Ok(map)
	} else {
		Err(serde::de::Error::invalid_length(len, &"fewer elements in map"))
	}
}

impl<'de> serde::Deserializer<'de> for &'de Value {
	type Error = Error;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self {
			Value::None => visitor.visit_unit(),
			Value::Bool(v) => visitor.visit_bool(*v),
			Value::Number(n) => n.deserialize_any(visitor),
			Value::String(v) => visitor.visit_borrowed_str(v),
			Value::Array(v) => visit_array_ref(v, visitor),
			Value::Object(v) => visit_object_ref(v, visitor),
			_ => todo!(),
		}
	}

	deserialize_number!(deserialize_i8);
	deserialize_number!(deserialize_i16);
	deserialize_number!(deserialize_i32);
	deserialize_number!(deserialize_i64);
	deserialize_number!(deserialize_i128);
	deserialize_number!(deserialize_u8);
	deserialize_number!(deserialize_u16);
	deserialize_number!(deserialize_u32);
	deserialize_number!(deserialize_u64);
	deserialize_number!(deserialize_u128);
	deserialize_number!(deserialize_f32);
	deserialize_number!(deserialize_f64);

	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match *self {
			Value::None => visitor.visit_none(),
			_ => visitor.visit_some(self),
		}
	}

	fn deserialize_enum<V>(
		self,
		_name: &str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		let (variant, value) = match self {
			Value::Object(value) => {
				let mut iter = value.into_iter();
				let (variant, value) = match iter.next() {
					Some(v) => v,
					None => {
						return Err(serde::de::Error::invalid_value(
							Unexpected::Map,
							&"map with a single key",
						));
					}
				};
				// enums are encoded in json as maps with a single key:value pair
				if iter.next().is_some() {
					return Err(serde::de::Error::invalid_value(
						Unexpected::Map,
						&"map with a single key",
					));
				}
				(variant, Some(value))
			}
			Value::String(variant) => (variant, None),
			other => {
				return Err(serde::de::Error::invalid_type(other.unexpected(), &"string or map"));
			}
		};

		visitor.visit_enum(EnumRefDeserializer {
			variant,
			value,
		})
	}

	#[inline]
	fn deserialize_newtype_struct<V>(
		self,
		name: &'static str,
		visitor: V,
	) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		let _ = name;
		visitor.visit_newtype_struct(self)
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match *self {
			Value::Bool(v) => visitor.visit_bool(v),
			_ => Err(self.invalid_type(&visitor)),
		}
	}

	fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		self.deserialize_str(visitor)
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self {
			Value::String(v) => visitor.visit_borrowed_str(v),
			_ => Err(self.invalid_type(&visitor)),
		}
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		self.deserialize_str(visitor)
	}

	fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self {
			Value::String(v) => visitor.visit_borrowed_str(v),
			Value::Array(v) => visit_array_ref(v, visitor),
			_ => Err(self.invalid_type(&visitor)),
		}
	}

	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		self.deserialize_bytes(visitor)
	}

	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match *self {
			Value::None => visitor.visit_unit(),
			_ => Err(self.invalid_type(&visitor)),
		}
	}

	fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		self.deserialize_unit(visitor)
	}

	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self {
			Value::Array(v) => visit_array_ref(v, visitor),
			_ => Err(self.invalid_type(&visitor)),
		}
	}

	fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		self.deserialize_seq(visitor)
	}

	fn deserialize_tuple_struct<V>(
		self,
		_name: &'static str,
		_len: usize,
		visitor: V,
	) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		self.deserialize_seq(visitor)
	}

	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self {
			Value::Object(v) => visit_object_ref(v, visitor),
			_ => Err(self.invalid_type(&visitor)),
		}
	}

	fn deserialize_struct<V>(
		self,
		_name: &'static str,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self {
			Value::Array(v) => visit_array_ref(v, visitor),
			Value::Object(v) => visit_object_ref(v, visitor),
			_ => Err(self.invalid_type(&visitor)),
		}
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		self.deserialize_str(visitor)
	}

	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		visitor.visit_unit()
	}
}

struct EnumRefDeserializer<'de> {
	variant: &'de str,
	value: Option<&'de Value>,
}

impl<'de> EnumAccess<'de> for EnumRefDeserializer<'de> {
	type Error = Error;
	type Variant = VariantRefDeserializer<'de>;

	fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Error>
	where
		V: DeserializeSeed<'de>,
	{
		let variant = self.variant.into_deserializer();
		let visitor = VariantRefDeserializer {
			value: self.value,
		};
		seed.deserialize(variant).map(|v| (v, visitor))
	}
}

struct VariantRefDeserializer<'de> {
	value: Option<&'de Value>,
}

impl<'de> VariantAccess<'de> for VariantRefDeserializer<'de> {
	type Error = Error;

	fn unit_variant(self) -> Result<(), Error> {
		match self.value {
			Some(value) => Deserialize::deserialize(value),
			None => Ok(()),
		}
	}

	fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
	where
		T: DeserializeSeed<'de>,
	{
		match self.value {
			Some(value) => seed.deserialize(value),
			None => {
				Err(serde::de::Error::invalid_type(Unexpected::UnitVariant, &"newtype variant"))
			}
		}
	}

	fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self.value {
			Some(Value::Array(v)) => {
				if v.is_empty() {
					visitor.visit_unit()
				} else {
					visit_array_ref(v, visitor)
				}
			}
			Some(other) => {
				Err(serde::de::Error::invalid_type(other.unexpected(), &"tuple variant"))
			}
			None => Err(serde::de::Error::invalid_type(Unexpected::UnitVariant, &"tuple variant")),
		}
	}

	fn struct_variant<V>(
		self,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self.value {
			Some(Value::Object(v)) => visit_object_ref(v, visitor),
			Some(other) => {
				Err(serde::de::Error::invalid_type(other.unexpected(), &"struct variant"))
			}
			None => Err(serde::de::Error::invalid_type(Unexpected::UnitVariant, &"struct variant")),
		}
	}
}

struct SeqRefDeserializer<'de> {
	iter: std::slice::Iter<'de, Value>,
}

impl<'de> SeqRefDeserializer<'de> {
	fn new(slice: &'de [Value]) -> Self {
		SeqRefDeserializer {
			iter: slice.iter(),
		}
	}
}

impl<'de> SeqAccess<'de> for SeqRefDeserializer<'de> {
	type Error = Error;

	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
	where
		T: DeserializeSeed<'de>,
	{
		match self.iter.next() {
			Some(value) => seed.deserialize(value).map(Some),
			None => Ok(None),
		}
	}

	fn size_hint(&self) -> Option<usize> {
		match self.iter.size_hint() {
			(lower, Some(upper)) if lower == upper => Some(upper),
			_ => None,
		}
	}
}

struct MapRefDeserializer<'de> {
	iter: <&'de Object as IntoIterator>::IntoIter,
	value: Option<&'de Value>,
}

impl<'de> MapRefDeserializer<'de> {
	fn new(map: &'de Object) -> Self {
		MapRefDeserializer {
			iter: map.into_iter(),
			value: None,
		}
	}
}

impl<'de> MapAccess<'de> for MapRefDeserializer<'de> {
	type Error = Error;

	fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
	where
		T: DeserializeSeed<'de>,
	{
		match self.iter.next() {
			Some((key, value)) => {
				self.value = Some(value);
				let key_de = MapKeyDeserializer {
					key: Cow::Borrowed(&**key),
				};
				seed.deserialize(key_de).map(Some)
			}
			None => Ok(None),
		}
	}

	fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Error>
	where
		T: DeserializeSeed<'de>,
	{
		match self.value.take() {
			Some(value) => seed.deserialize(value),
			None => Err(serde::de::Error::custom("value is missing")),
		}
	}

	fn size_hint(&self) -> Option<usize> {
		match self.iter.size_hint() {
			(lower, Some(upper)) if lower == upper => Some(upper),
			_ => None,
		}
	}
}

struct MapKeyDeserializer<'de> {
	key: Cow<'de, str>,
}

macro_rules! deserialize_simple_numeric_key {
	($method:ident, $kind:ident) => {
		fn $method<V>(self, visitor: V) -> Result<V::Value, Error>
		where
			V: Visitor<'de>,
		{
			match self.key.parse() {
				Ok(x) => visitor.$kind(x),
				_ => return Err(Error::Deserializer("expected numeric key".to_owned())),
			}
		}
	};
}

impl<'de> serde::Deserializer<'de> for MapKeyDeserializer<'de> {
	type Error = Error;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		BorrowedCowStrDeserializer::new(self.key).deserialize_any(visitor)
	}

	deserialize_simple_numeric_key!(deserialize_i8, visit_i8);
	deserialize_simple_numeric_key!(deserialize_i16, visit_i16);
	deserialize_simple_numeric_key!(deserialize_i32, visit_i32);
	deserialize_simple_numeric_key!(deserialize_i64, visit_i64);
	deserialize_simple_numeric_key!(deserialize_u8, visit_u8);
	deserialize_simple_numeric_key!(deserialize_u16, visit_u16);
	deserialize_simple_numeric_key!(deserialize_u32, visit_u32);
	deserialize_simple_numeric_key!(deserialize_u64, visit_u64);
	deserialize_simple_numeric_key!(deserialize_f32, visit_f32);
	deserialize_simple_numeric_key!(deserialize_f64, visit_f64);
	deserialize_simple_numeric_key!(deserialize_i128, visit_i128);
	deserialize_simple_numeric_key!(deserialize_u128, visit_u128);

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		if self.key == "true" {
			visitor.visit_bool(true)
		} else if self.key == "false" {
			visitor.visit_bool(false)
		} else {
			Err(serde::de::Error::invalid_type(Unexpected::Str(&self.key), &visitor))
		}
	}

	#[inline]
	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		// Map keys cannot be null.
		visitor.visit_some(self)
	}

	#[inline]
	fn deserialize_newtype_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		visitor.visit_newtype_struct(self)
	}

	fn deserialize_enum<V>(
		self,
		name: &'static str,
		variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		self.key.into_deserializer().deserialize_enum(name, variants, visitor)
	}

	forward_to_deserialize_any! {
		char str string bytes byte_buf unit unit_struct seq tuple tuple_struct
		map struct identifier ignored_any
	}
}

/*
struct KeyClassifier;

enum KeyClass {
	Map(String),
}

impl<'de> DeserializeSeed<'de> for KeyClassifier {
	type Value = KeyClass;

	fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_str(self)
	}
}

impl<'de> Visitor<'de> for KeyClassifier {
	type Value = KeyClass;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a string key")
	}

	fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		match s {
			#[cfg(feature = "arbitrary_precision")]
			crate::number::TOKEN => Ok(KeyClass::Number),
			#[cfg(feature = "raw_value")]
			crate::raw::TOKEN => Ok(KeyClass::RawValue),
			_ => Ok(KeyClass::Map(s.to_owned())),
		}
	}

	#[cfg(any(feature = "std", feature = "alloc"))]
	fn visit_string<E>(self, s: String) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		match s.as_str() {
			#[cfg(feature = "arbitrary_precision")]
			crate::number::TOKEN => Ok(KeyClass::Number),
			#[cfg(feature = "raw_value")]
			crate::raw::TOKEN => Ok(KeyClass::RawValue),
			_ => Ok(KeyClass::Map(s)),
		}
	}
}
*/

impl Value {
	#[cold]
	fn invalid_type<E>(&self, exp: &dyn Expected) -> E
	where
		E: serde::de::Error,
	{
		serde::de::Error::invalid_type(self.unexpected(), exp)
	}

	#[cold]
	fn unexpected(&self) -> Unexpected {
		match self {
			Value::None => Unexpected::Unit,
			Value::Bool(b) => Unexpected::Bool(*b),
			Value::Number(n) => n.unexpected(),
			Value::String(s) => Unexpected::Str(s),
			Value::Array(_) => Unexpected::Seq,
			Value::Object(_) => Unexpected::Map,
			Value::Uuid(_) => Unexpected::Other("uuid"),
			Value::RecordId(_) => Unexpected::Other("record-id"),
			Value::Datetime(_) => Unexpected::Other("datetime"),
			Value::Duration(_) => Unexpected::Other("duration"),
			Value::Bytes(x) => Unexpected::Bytes(x),
		}
	}
}

impl Number {
	fn unexpected(&self) -> Unexpected {
		match self {
			Number::Float(x) => Unexpected::Float(*x),
			Number::Integer(i) => Unexpected::Signed(*i),
			Number::Decimal(_) => Unexpected::Other("decimal"),
		}
	}
}

struct BorrowedCowStrDeserializer<'de> {
	value: Cow<'de, str>,
}

impl<'de> BorrowedCowStrDeserializer<'de> {
	fn new(value: Cow<'de, str>) -> Self {
		BorrowedCowStrDeserializer {
			value,
		}
	}
}

impl<'de> Deserializer<'de> for BorrowedCowStrDeserializer<'de> {
	type Error = Error;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		match self.value {
			Cow::Borrowed(string) => visitor.visit_borrowed_str(string),
			Cow::Owned(string) => visitor.visit_string(string),
		}
	}

	fn deserialize_enum<V>(
		self,
		_name: &str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		visitor.visit_enum(self)
	}

	forward_to_deserialize_any! {
		bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
		bytes byte_buf option unit unit_struct newtype_struct seq tuple
		tuple_struct map struct identifier ignored_any
	}
}

impl<'de> EnumAccess<'de> for BorrowedCowStrDeserializer<'de> {
	type Error = Error;
	type Variant = UnitOnly;

	fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant), Error>
	where
		T: DeserializeSeed<'de>,
	{
		let value = seed.deserialize(self)?;
		Ok((value, UnitOnly))
	}
}

struct UnitOnly;

impl<'de> VariantAccess<'de> for UnitOnly {
	type Error = Error;

	fn unit_variant(self) -> Result<(), Error> {
		Ok(())
	}

	fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Error>
	where
		T: DeserializeSeed<'de>,
	{
		Err(serde::de::Error::invalid_type(Unexpected::UnitVariant, &"newtype variant"))
	}

	fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		Err(serde::de::Error::invalid_type(Unexpected::UnitVariant, &"tuple variant"))
	}

	fn struct_variant<V>(
		self,
		_fields: &'static [&'static str],
		_visitor: V,
	) -> Result<V::Value, Error>
	where
		V: Visitor<'de>,
	{
		Err(serde::de::Error::invalid_type(Unexpected::UnitVariant, &"struct variant"))
	}
}
