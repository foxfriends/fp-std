//! Functions for manipulating functions.

/* BASIC FUNCTIONS ************************************************************/

/// Creates a function that always returns the same value.
///
/// # Examples
///
/// ```rust
/// # use fp_std::function::always;
/// let one = always(1);
/// assert_eq!(one(), 1);
/// ```
pub fn always<A: Clone>(a: A) -> impl Fn() -> A {
    move || a.clone()
}

/// Flips the arguments of a 2-arity function.
///
/// # Examples
///
/// ```rust
/// # use fp_std::function::flip;
/// # use std::ops::Sub;
/// let sub = flip(usize::sub);
/// assert_eq!(sub(3, 5), 2);
/// ```
pub fn flip<A, B, C, F>(f: F) -> impl Fn(B, A) -> C
where
    F: Fn(A, B) -> C,
{
    move |a, b| f(b, a)
}

/// Takes only the first argument of a 2-arity function.
///
/// # Examples
///
/// ```rust
/// # use fp_std::function::first_arg;
/// fn double(x: usize) -> usize { x * 2 }
/// let double_first = first_arg(double);
/// assert_eq!(double_first(3, 4), 6);
/// ```
pub fn first_arg<A, B, C, F>(f: F) -> impl Fn(A, B) -> C
where
    F: Fn(A) -> C,
{
    move |a, _| f(a)
}

/// Takes only the first argument of a 2-arity function.
///
/// # Examples
///
/// ```rust
/// # use fp_std::function::second_arg;
/// fn double(x: usize) -> usize { x * 2 }
/// let double_second = second_arg(double);
/// assert_eq!(double_second(3, 4), 8);
/// ```
pub fn second_arg<A, B, C, F>(f: F) -> impl Fn(A, B) -> C
where
    F: Fn(B) -> C,
{
    move |_, b| f(b)
}

/// Supplies the first argument to a 2-arity function.
///
/// # Examples
///
/// # use fp_std::function::apply_first;
/// # use std::ops::Add;
/// let add1 = apply_first(1, usize::add);
/// assert_eq!(add1(2), 3);
pub fn apply_first<F, A: Clone, B, C>(a: A, f: F) -> impl Fn(B) -> C
where
    F: Fn(A, B) -> C,
{
    move |b| f(a.clone(), b)
}

/// Supplies the second argument to a 2-arity function.
///
/// # Examples
///
/// # use fp_std::function::apply_second;
/// # use std::ops::Sub;
/// let sub1 = apply_second(1, usize::sub);
/// assert_eq!(sub1(2), 1);
pub fn apply_second<F, A, B: Clone, C>(b: B, f: F) -> impl Fn(A) -> C
where
    F: Fn(A, B) -> C,
{
    move |a| f(a, b.clone())
}
