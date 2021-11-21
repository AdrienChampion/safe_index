# v0.9.17

- macro input
    - `map MapType` is not followed by `with iter IterType` anymore
- maps:
    - no dedicated iterator type anymore
    - function `of` constructing a map from a vector is gone, use the `From` implementation
        instead;
    - `last` now also returns the index of the last element, if any;
    - `last_mut` has been fixed and also returns the last element's index;
    - new `push_idx` function: like `push`, but takes an element constructor taking the element's
        index as argument;
    - added implementation of `std::ops::Index<std::ops::RangeToInclusive<usize>>`

# v0.9.11

- `const` map constructors
- `const` map-iter constructors
- `const` index constructors and accessor

# v0.9.9

- indices now implement `Default` [#1](https://github.com/AdrienChampion/safe_index/issues/1)

# v0.9.6

- maps (vectors) now
    - implement `Index` for the various `std::ops::Range`s
    - have `reserve`, `get`, `get_mut`, `last` and `last_mut` methods

# v0.9.4

- added a `split(idx)` function over maps that splits a map into
    - an iterator over the elements *before* `idx`
    - the element at `idx`
    - an iterator over the elements *after* `idx`