use std::cmp::Ordering;

use super::IDBKey;

#[derive(Debug, PartialEq)]
pub struct IDBKeyRange {
    lower_key: Option<IDBKey>,
    upper_key: Option<IDBKey>,
    lower_open: bool,
    upper_open: bool,
}

impl IDBKeyRange {
    pub fn bound(
        lower: Option<IDBKey>,
        upper: Option<IDBKey>,
        lower_open: Option<bool>,
        upper_open: Option<bool>,
    ) -> Result<Self, String> {
        if lower == upper && (lower_open == Some(true) || upper_open == Some(true)) {
            return Err("When lower and upper bounds match, neither may be open".to_string());
        }
        if let (Some(l), Some(u)) = (&lower, &upper) {
            if l > u {
                return Err("Lower bound must not be highter ".to_string());
            }
        }
        Ok(Self {
            lower_key: lower,
            upper_key: upper,
            lower_open: lower_open.unwrap_or(false),
            upper_open: upper_open.unwrap_or(false),
        })
    }

    pub fn unbound() -> Self {
        Self {
            lower_key: None,
            upper_key: None,
            lower_open: false,
            upper_open: false,
        }
    }

    pub fn includes(&self, key: &IDBKey) -> bool {
        if let Some(lower) = &self.lower_key {
            if let Some(cmp) = lower.partial_cmp(key) {
                if cmp == Ordering::Greater || (cmp == Ordering::Equal && self.lower_open) {
                    return false;
                }
            } else {
                return false;
            }
        }
        if let Some(upper) = &self.upper_key {
            if let Some(cmp) = upper.partial_cmp(key) {
                if cmp == Ordering::Less || (cmp == Ordering::Equal && self.upper_open) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::js::{
        api::index_db::{IDBKey, IDBKeyRange},
        JsNumberPrimitive,
    };

    fn valid_bounds(
        lower: Option<IDBKey>,
        upper: Option<IDBKey>,
        lower_open: Option<bool>,
        upper_open: Option<bool>,
    ) -> IDBKeyRange {
        IDBKeyRange::bound(lower, upper, lower_open, upper_open).unwrap()
    }

    fn n<V: Into<JsNumberPrimitive>>(value: V) -> IDBKey {
        IDBKey::Number(value.into())
    }

    fn no<V: Into<JsNumberPrimitive>>(value: V) -> Option<IDBKey> {
        Some(n(value))
    }

    fn s<V: Into<String>>(value: V) -> IDBKey {
        IDBKey::String(value.into())
    }

    fn so<V: Into<String>>(value: V) -> Option<IDBKey> {
        Some(s(value))
    }

    fn d<V: Into<JsNumberPrimitive>>(value: V) -> IDBKey {
        IDBKey::Date(value.into())
    }

    #[test]
    fn test_valid_bounds() {
        assert!(valid_bounds(no(1), no(3), None, None).includes(&n(2)));
        assert!(valid_bounds(no(1), no(3), Some(true), None).includes(&n(2)));
        assert!(!valid_bounds(no(1), no(3), Some(true), None).includes(&n(1)));
        assert!(valid_bounds(no(1), no(3), None, Some(true)).includes(&n(1)));
        assert!(valid_bounds(no(1), no(1), None, None).includes(&n(1)));
        assert!(!valid_bounds(no(1), no(1), None, None).includes(&n(2)));

        assert!(valid_bounds(None, no(1), None, None).includes(&n(-2)));
        assert!(!valid_bounds(no(3), None, None, None).includes(&n(-2)));

        assert!(valid_bounds(None, None, None, None).includes(&n(100)));
        assert_eq!(valid_bounds(None, None, None, None), IDBKeyRange::unbound());
    }

    /// Source: <https://w3c.github.io/IndexedDB/#key-construct>
    ///
    /// Number keys are less than date keys. Date keys are less than string keys. String keys are
    /// less than binary keys. Binary keys are less than array keys. There is no highest possible
    /// key value. This is because an array of any candidate highest key followed by another key iseven higher.
    #[test]
    fn test_weird_bounds() {
        assert!(valid_bounds(no(4), so("n"), None, None).includes(&d(-1)));
    }

    #[test]
    fn test_invalid_bounds() {
        assert!(IDBKeyRange::bound(no(1), no(1), Some(true), None).is_err());
        assert!(IDBKeyRange::bound(no(1), no(1), None, Some(true)).is_err());
        assert!(IDBKeyRange::bound(no(1), no(-1), None, None).is_err());
        assert!(IDBKeyRange::bound(no(1), no(-1), None, None).is_err());
    }
}
