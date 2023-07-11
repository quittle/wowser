use std::cmp::Ordering;

use crate::js::{api::date::Date, JsNumberPrimitive};

#[derive(Debug, PartialEq, Clone)]
pub enum IDBKey {
    Number(JsNumberPrimitive),
    Date(Date),
    String(String),
    Binary(Vec<u8>),
    Array(Vec<IDBKey>),
}

impl PartialOrd for IDBKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Number(s), Self::Number(o)) | (Self::Date(s), Self::Date(o)) => s.partial_cmp(o),
            (Self::String(s), Self::String(o)) => s.partial_cmp(o),
            (Self::Array(s), Self::Array(o)) => {
                let len = s.len().min(o.len());
                for i in 0..len {
                    let cmp = s[i].partial_cmp(&o[i]);
                    if !matches!(cmp, Some(Ordering::Equal)) {
                        return cmp;
                    }
                }
                s.len().partial_cmp(&o.len())
            }
            (Self::Binary(s), Self::Binary(o)) => s.partial_cmp(o),
            (Self::Array(_), _) => Some(Ordering::Greater),
            (_, Self::Array(_)) => Some(Ordering::Less),
            (Self::Binary(_), _) => Some(Ordering::Greater),
            (_, Self::Binary(_)) => Some(Ordering::Less),
            (Self::String(_), _) => Some(Ordering::Greater),
            (_, Self::String(_)) => Some(Ordering::Less),
            (Self::Date(_), _) => Some(Ordering::Greater),
            (_, Self::Date(_)) => Some(Ordering::Less),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::js::{api::date::Date, JsNumberPrimitive};

    use super::IDBKey;

    #[track_caller]
    fn assert_greater(a: IDBKey, b: IDBKey) {
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Greater));
    }

    #[track_caller]
    fn assert_less(a: IDBKey, b: IDBKey) {
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
    }

    #[track_caller]
    fn assert_equal(a: IDBKey, b: IDBKey) {
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal));
    }

    fn n<V: Into<JsNumberPrimitive>>(value: V) -> IDBKey {
        IDBKey::Number(value.into())
    }

    fn d<V: Into<Date>>(value: V) -> IDBKey {
        IDBKey::Date(value.into())
    }

    fn s(value: &str) -> IDBKey {
        IDBKey::String(value.to_string())
    }

    fn b<const LENGTH: usize>(value: [u8; LENGTH]) -> IDBKey {
        IDBKey::Binary(value.to_vec())
    }

    fn a<const LENGTH: usize>(value: [IDBKey; LENGTH]) -> IDBKey {
        IDBKey::Array(value.to_vec())
    }

    #[test]
    fn test_number() {
        assert_equal(n(1), n(1));
        assert_greater(n(3), n(1));
        assert_less(n(-1), n(1));
    }

    #[test]
    fn test_date() {
        assert_equal(d(1), d(1));
        assert_greater(d(3), d(1));
        assert_less(d(-1), d(1));
    }

    #[test]
    fn test_string() {
        assert_equal(s(""), s(""));
        assert_equal(s("abc"), s("abc"));

        assert_greater(s("zzz"), s("aaa"));
        assert_greater(s("zaa"), s("aaz"));
        assert_greater(s("abcd"), s("abc"));

        assert_less(s("abc"), s("xyz"));
        assert_less(s("abc"), s("abcd"));
    }

    #[test]
    fn test_binary() {
        assert_equal(b([]), b([]));
        assert_equal(b([1, 2, 3]), b([1, 2, 3]));

        assert_greater(b([9]), b([3]));
        assert_greater(b([9, 1, 1]), b([1]));
        assert_greater(b([9, 1, 1]), b([1, 1, 9]));
        assert_greater(b([9]), b([8, 9, 9, 9]));

        assert_less(b([1, 2, 3]), b([4, 5, 6]));
        assert_less(b([]), b([1]));
        assert_less(b([1, 2]), b([1, 2, 3]));
    }

    #[test]
    fn test_array() {
        assert_equal(a([]), a([]));
        assert_equal(a([n(1)]), a([n(1)]));
        assert_equal(
            a([n(1), s("abc"), a([d(3)])]),
            a([n(1), s("abc"), a([d(3)])]),
        );

        assert_less(a([n(1)]), a([n(3)]));
        assert_greater(a([s("bc")]), a([s("abc")]));

        assert_less(a([]), a([n(1)]));
        assert_greater(a([n(1)]), a([]));

        assert_less(a([n(9)]), a([a([n(9)])]));
    }

    #[test]
    fn test_mix() {
        assert_greater(a([]), s("abc"));
        assert_less(n(123), a([]));
        assert_greater(b([]), s("abc"));
        assert_less(d(999), b([]));
        assert_greater(s(""), n(999));
        assert_less(n(999), s(""));
        assert_greater(d(-9), n(999));
        assert_less(n(999), d(-9));
    }
}
