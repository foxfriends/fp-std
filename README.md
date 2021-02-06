# fp-std

A "standard library" of helper functions for expressing functional programming in Rust.
Works great with the traits defined in [fp-core][]!

[fp-core]: https://crates.io/crates/fp-core

Notice that using universal function call syntax, all existing Rust functions can be passed
to other functions without issue. This allows functions often included in a library such as
this to be omitted in favour of existing ones. Unfortunately, we are still challenged by
the fact that Rust does not provide any means of auto-currying. This can be somewhat mitigated
by the use of the `fp_std::function::apply_second` function.

```rust
let double = |x| x * 2;
let double_all = apply_second(double, Iterator::map);
let doubled: Vec<_> = vec![vec![1], vec![2], vec![3]]
    .into_iter()
    .map(IntoIterator::into_iter)
    .map(double_all)
    .map(Iterator::collect::<Vec<_>>)
    .collect();
assert_eq!(doubled, vec![vec![2], vec![4], vec![6]]);
```

Despite this all working, it's really not all that pleasant an experience overall. This
project exists mostly as an experiment. If it provides value to you, feel free to use it,
but do not consider anything you see here to be best practice.
