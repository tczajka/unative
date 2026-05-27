use core::fmt;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::inner::{INativeInner, UNativeInner};
use crate::{INative, UNative};

impl Serialize for UNative {
    #[inline]
    #[allow(clippy::useless_conversion)] // `u64::from` is a no-op when the inner is already u64.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(u64::from(self.0))
    }
}

impl Serialize for INative {
    #[inline]
    #[allow(clippy::useless_conversion)] // `i64::from` is a no-op when the inner is already i64.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(i64::from(self.0))
    }
}

struct UNativeVisitor;

impl<'de> Visitor<'de> for UNativeVisitor {
    type Value = UNative;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("an integer in the UNative range")
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<UNative, E> {
        UNativeInner::try_from(v)
            .map(UNative)
            .map_err(|_| E::custom("value out of range for UNative"))
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<UNative, E> {
        UNativeInner::try_from(v)
            .map(UNative)
            .map_err(|_| E::custom("value out of range for UNative"))
    }

    fn visit_u128<E: de::Error>(self, v: u128) -> Result<UNative, E> {
        UNativeInner::try_from(v)
            .map(UNative)
            .map_err(|_| E::custom("value out of range for UNative"))
    }

    fn visit_i128<E: de::Error>(self, v: i128) -> Result<UNative, E> {
        UNativeInner::try_from(v)
            .map(UNative)
            .map_err(|_| E::custom("value out of range for UNative"))
    }
}

impl<'de> Deserialize<'de> for UNative {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<UNative, D::Error> {
        deserializer.deserialize_u64(UNativeVisitor)
    }
}

struct INativeVisitor;

impl<'de> Visitor<'de> for INativeVisitor {
    type Value = INative;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("an integer in the INative range")
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<INative, E> {
        INativeInner::try_from(v)
            .map(INative)
            .map_err(|_| E::custom("value out of range for INative"))
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<INative, E> {
        INativeInner::try_from(v)
            .map(INative)
            .map_err(|_| E::custom("value out of range for INative"))
    }

    fn visit_u128<E: de::Error>(self, v: u128) -> Result<INative, E> {
        INativeInner::try_from(v)
            .map(INative)
            .map_err(|_| E::custom("value out of range for INative"))
    }

    fn visit_i128<E: de::Error>(self, v: i128) -> Result<INative, E> {
        INativeInner::try_from(v)
            .map(INative)
            .map_err(|_| E::custom("value out of range for INative"))
    }
}

impl<'de> Deserialize<'de> for INative {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<INative, D::Error> {
        deserializer.deserialize_i64(INativeVisitor)
    }
}
