//! Functions for manipulating tuples (particularly 2-tuples).

/* BASIC FUNCTIONS ************************************************************/

/// Constructs a tuple of two copies of the same value.
///
/// # Examples
///
/// ```rust
/// # use fp_std::tuple::duplicate;
/// assert_eq!(duplicate(1), (1, 1));
/// ```
pub fn duplicate<A: Clone>(a: A) -> (A, A) {
    (a.clone(), a)
}

/// Constructs a 2-tuple from the arguments.
///
/// # Examples
///
/// ```rust
/// # use fp_std::tuple::cons;
/// assert_eq!(cons(1, 2), (1, 2));
/// ```
pub fn cons<A, B>(a: A, b: B) -> (A, B) {
    (a, b)
}

/// Spreads a 2-tuple into the arguments of a 2-arity function.
///
/// # Examples
///
/// ```rust
/// # use fp_std::tuple::spread;
/// # use std::ops::Add;
/// let add = spread(usize::add);
/// assert_eq!(add((1, 2)), 3);
/// ```
pub fn spread<A, B, C, F>(f: F) -> impl Fn((A, B)) -> C
where
    F: Fn(A, B) -> C,
{
    move |(a, b)| f(a, b)
}

/// Returns the first element in a 2-tuple.
///
/// # Examples
///
/// ```rust
/// # use fp_std::tuple::first;
/// assert_eq!(first((1, 2)), 1);
/// ```
pub fn first<A, B>((a, ..): (A, B)) -> A {
    a
}

/// Returns the second element in a 2-tuple.
///
/// # Examples
///
/// ```rust
/// # use fp_std::tuple::second;
/// assert_eq!(second((1, 2)), 2);
/// ```
pub fn second<A, B>((.., b): (A, B)) -> B {
    b
}

/// Transforms the second element in a 2-tuple.
///
/// # Examples
///
/// ```rust
/// # use fp_std::tuple::map_first;
/// fn double(x: usize) -> usize { x * 2 }
/// assert_eq!(map_first(double)((1, 2)), (2, 2));
/// ```
pub fn map_first<A, B, C, F>(f: F) -> impl Fn((A, B)) -> (C, B)
where
    F: Fn(A) -> C,
{
    move |(a, b)| (f(a), b)
}

/// Transforms the second element in a 2-tuple.
///
/// # Examples
///
/// ```rust
/// # use fp_std::tuple::map_second;
/// fn double(x: usize) -> usize { x * 2 }
/// assert_eq!(map_second(double)((1, 2)), (1, 4));
/// ```
pub fn map_second<A, B, C, F>(f: F) -> impl Fn((A, B)) -> (A, C)
where
    F: Fn(B) -> C,
{
    move |(a, b)| (a, f(b))
}

/* LENSES *********************************************************************/
use fp_core::lens::Lens;

/// A [Lens](fp_core::lens::Lens) for the first element in a 2-tuple.
///
/// # Examples
///
/// ```rust
/// # use fp_std::tuple::LensFirst;
/// use fp_core::lens::Lens;
/// assert_eq!(LensFirst::get(&(1, 2)), Some(&1));
/// assert_eq!(LensFirst::set(3, &(1, 2)), (3, 2));
/// ```
pub struct LensFirst;

impl<A, B: Clone> Lens<(A, B), A> for LensFirst {
    fn get((a, ..): &(A, B)) -> Option<&A> {
        Some(a)
    }

    fn set(a: A, (.., b): &(A, B)) -> (A, B) {
        (a, b.clone())
    }
}

/// A [Lens](fp_core::lens::Lens) for the second element in a 2-tuple.
///
/// # Examples
///
/// ```rust
/// # use fp_std::tuple::LensSecond;
/// use fp_core::lens::Lens;
/// assert_eq!(LensSecond::get(&(1, 2)), Some(&2));
/// assert_eq!(LensSecond::set(3, &(1, 2)), (1, 3));
/// ```
pub struct LensSecond;

impl<A: Clone, B> Lens<(A, B), B> for LensSecond {
    fn get((.., b): &(A, B)) -> Option<&B> {
        Some(b)
    }

    fn set(b: B, (a, ..): &(A, B)) -> (A, B) {
        (a.clone(), b)
    }
}
