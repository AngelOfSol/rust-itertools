//! Arithmetic on **Iterator** *.size_hint()* values.
//!

use std::usize;
use std::cmp;

/// **SizeHint** is the return type of **Iterator::size_hint()**.
pub type SizeHint = (usize, Option<usize>);

/// Add **x** correctly to a **SizeHint**.
#[inline]
pub fn add_scalar(sh: SizeHint, x: usize) -> SizeHint
{
    let (mut low, mut hi) = sh;
    low = low.saturating_add(x);
    hi = hi.and_then(|elt| elt.checked_add(x));
    (low, hi)
}

/// Add **SizeHint** correctly.
#[inline]
pub fn add(a: SizeHint, b: SizeHint) -> SizeHint
{
    let min = a.0.checked_add(b.0).unwrap_or(usize::MAX);
    let max = match (a.1, b.1) {
        (Some(x), Some(y)) => x.checked_add(y),
        _ => None,
    };

    (min, max)
}

/// Multiply **x** correctly with a **SizeHint**.
///
/// # Examples
///
/// ```
/// use std::usize;
/// use itertools::size_hint;
///
/// assert_eq!(size_hint::mul_scalar((3, Some(4)), 3),
///            (9, Some(12)));
/// assert_eq!(size_hint::mul_scalar((3, Some(4)), usize::MAX),
///            (usize::MAX, None));
/// ```
#[inline]
pub fn mul_scalar(sh: SizeHint, x: usize) -> SizeHint
{
    let (mut low, mut hi) = sh;
    low = low.checked_mul(x).unwrap_or(usize::MAX);
    if x == 0 {
        hi = Some(0)
    } else {
        hi = hi.and_then(|elt| elt.checked_mul(x));
    }
    (low, hi)
}

/// Multiply **SizeHint** correctly
///
/// # Examples
///
/// ```
/// use std::usize;
/// use itertools::size_hint;
///
/// assert_eq!(size_hint::mul((3, Some(4)), (3, Some(4))),
///            (9, Some(16)));
///
/// assert_eq!(size_hint::mul((3, Some(4)), (usize::MAX, None)),
///            (usize::MAX, None));
///
/// assert_eq!(size_hint::mul((3, None), (0, Some(0))),
///            (0, Some(0)));
/// ```
#[inline]
pub fn mul(a: SizeHint, b: SizeHint) -> SizeHint
{
    let low = a.0.checked_mul(b.0).unwrap_or(usize::MAX);
    let hi = match (a.1, b.1) {
        (Some(x), Some(y)) => x.checked_mul(y),
        (Some(0), None) | (None, Some(0)) => Some(0),
        _ => None,
    };
    (low, hi)
}

/// Return the maximum
#[inline]
pub fn max(a: SizeHint, b: SizeHint) -> SizeHint
{
    let (a_lower, a_upper) = a;
    let (b_lower, b_upper) = b;

    let lower = cmp::max(a_lower, b_lower);

    let upper = match (a_upper, b_upper) {
        (Some(x), Some(y)) => Some(cmp::max(x,y)),
        _ => None
    };

    (lower, upper)
}

/// Return the minimum
#[inline]
pub fn min(a: SizeHint, b: SizeHint) -> SizeHint
{
    let (a_lower, a_upper) = a;
    let (b_lower, b_upper) = b;
    let lower = cmp::min(a_lower, b_lower);
    let upper = match (a_upper, b_upper) {
        (Some(u1), Some(u2)) => Some(cmp::min(u1, u2)),
        _ => a_upper.or(b_upper)
    };
    (lower, upper)
}
