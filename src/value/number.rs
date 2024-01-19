use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

/// A numeric value wrapper, supports u8-u128, i8-i128, f32, & f64
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Number {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(F32),
    F64(F64),
}

macro_rules! float_ty {
    ($ty:ident($float:ty)) => {
        #[doc = concat!(
                    "A wrapper for [`", stringify!($float), "`], which implements [`Eq`], ",
                    "[`Hash`] and [`Ord`] using [`", stringify!($float), "::total_cmp`] ",
                    "for a total order comparison",
                )]
        #[derive(Copy, Clone, Debug)] // GRCOV_EXCL_LINE
        pub struct $ty(pub $float);

        impl $ty {
            #[doc = concat!("Construct a new [`", stringify!($ty), "`].")]
            #[must_use]
            pub fn new(v: $float) -> Self {
                Self(v)
            }

            #[doc = concat!("Returns the wrapped [`", stringify!($float), "`].")]
            #[must_use]
            pub fn get(self) -> $float {
                self.0
            }
        }

        impl From<$float> for $ty {
            fn from(v: $float) -> Self {
                Self::new(v)
            }
        }

        /// Partial equality comparison
        ///
        #[doc = concat!(
                    "In order to be able to use [`", stringify!($ty), "`] as a mapping key, ",
                    "floating values use [`", stringify!($float), "::total_cmp`] for a total ",
                    "order comparison.",
                )]
        ///
        /// See the [`Ord`] implementation.
        impl PartialEq for $ty {
            fn eq(&self, other: &Self) -> bool {
                self.cmp(other).is_eq()
            }
        }

        /// Equality comparison
        ///
        #[doc = concat!(
                    "In order to be able to use [`", stringify!($ty), "`] as a mapping key, ",
                    "floating values use [`", stringify!($float), "::total_cmp`] for a total ",
                    "order comparison.",
                )]
        ///
        /// See the [`Ord`] implementation.
        impl Eq for $ty {}

        impl Hash for $ty {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.0.to_bits().hash(state);
            }
        }

        /// Partial ordering comparison
        ///
        #[doc = concat!(
                    "In order to be able to use [`", stringify!($ty), "`] as a mapping key, ",
                    "floating values use [`", stringify!($float), "::total_cmp`] for a total ",
                    "order comparison.",
                )]
        ///
        /// See the [`Ord`] implementation.
        impl PartialOrd for $ty {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        /// Ordering comparison
        ///
        #[doc = concat!(
                    "In order to be able to use [`", stringify!($ty), "`] as a mapping key, ",
                    "floating values use [`", stringify!($float), "::total_cmp`] for a total ",
                    "order comparison.",
                )]
        ///
        /// ```
        #[doc = concat!("use ron::value::", stringify!($ty), ";")]
        #[doc = concat!(
                    "assert!(", stringify!($ty), "::new(", stringify!($float), "::NAN) > ",
                    stringify!($ty), "::new(", stringify!($float), "::INFINITY));",
                )]
        #[doc = concat!(
                    "assert!(", stringify!($ty), "::new(-", stringify!($float), "::NAN) < ",
                    stringify!($ty), "::new(", stringify!($float), "::NEG_INFINITY));",
                )]
        #[doc = concat!(
                    "assert!(", stringify!($ty), "::new(", stringify!($float), "::NAN) == ",
                    stringify!($ty), "::new(", stringify!($float), "::NAN));",
                )]
        /// ```
        impl Ord for $ty {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.total_cmp(&other.0)
            }
        }
    };
}

float_ty! { F32(f32) }
float_ty! { F64(f64) }

impl Default for Number {
    fn default() -> Self {
        Self::I32(0)
    }
}

impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<u128> for Number {
    fn from(value: u128) -> Self {
        Self::U128(value)
    }
}

impl From<i8> for Number {
    fn from(value: i8) -> Self {
        Self::I8(value)
    }
}

impl From<i16> for Number {
    fn from(value: i16) -> Self {
        Self::I16(value)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<i128> for Number {
    fn from(value: i128) -> Self {
        Self::I128(value)
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Self::F32(F32::new(value))
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::F64(F64::new(value))
    }
}

impl Number {
    #[inline]
    pub fn is_u8(&self) -> bool {
        matches!(self, Number::U8(_))
    }

    #[inline]
    pub fn as_u8(&self) -> Option<u8> {
        match self {
            Number::U8(val) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_u16(&self) -> bool {
        matches!(self, Number::U16(_))
    }

    #[inline]
    pub fn as_u16(&self) -> Option<u16> {
        match self {
            Number::U16(val) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_u32(&self) -> bool {
        matches!(self, Number::U32(_))
    }

    #[inline]
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Number::U32(val) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_u64(&self) -> bool {
        matches!(self, Number::U64(_))
    }

    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Number::U64(val) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_u128(&self) -> bool {
        matches!(self, Number::U128(_))
    }

    #[inline]
    pub fn as_u128(&self) -> Option<u128> {
        match self {
            Number::U128(val) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_i8(&self) -> bool {
        matches!(self, Number::I8(_))
    }

    #[inline]
    pub fn as_i8(&self) -> Option<i8> {
        match self {
            Number::I8(val) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_i16(&self) -> bool {
        matches!(self, Number::I16(_))
    }

    #[inline]
    pub fn as_i16(&self) -> Option<i16> {
        match self {
            Number::I16(val) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_i32(&self) -> bool {
        matches!(self, Number::I32(_))
    }

    #[inline]
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Number::I32(val) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_i64(&self) -> bool {
        matches!(self, Number::I64(_))
    }

    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Number::I64(val) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_i128(&self) -> bool {
        matches!(self, Number::I128(_))
    }

    #[inline]
    pub fn as_i128(&self) -> Option<i128> {
        match self {
            Number::I128(val) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_f32(&self) -> bool {
        matches!(self, Number::F32(_))
    }

    #[inline]
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Number::F32(val) => Some(val.0),
            _ => None,
        }
    }

    #[inline]
    pub fn is_f64(&self) -> bool {
        matches!(self, Number::F64(_))
    }

    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Number::F64(val) => Some(val.0),
            _ => None,
        }
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::U8(val) => write!(f, "{val}u8"),
            Number::U16(val) => write!(f, "{val}u16"),
            Number::U32(val) => write!(f, "{val}u32"),
            Number::U64(val) => write!(f, "{val}u64"),
            Number::U128(val) => write!(f, "{val}u128"),
            Number::I8(val) => write!(f, "{val}i8"),
            Number::I16(val) => write!(f, "{val}i16"),
            Number::I32(val) => write!(f, "{val}i32"),
            Number::I64(val) => write!(f, "{val}i64"),
            Number::I128(val) => write!(f, "{val}i128"),
            Number::F32(val) => write!(f, "{}f32", val.0),
            Number::F64(val) => write!(f, "{}f64", val.0),
        }
    }
}