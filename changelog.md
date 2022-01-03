# v0.9.19

- maps (`Vec`s) using indices as keys now have a `map.index_from_usize(n)` function that generates
    an index from a `usize` if and only if it is a legal index for this map; typically useful when
    parsing element indices as `usize`s.

# v0.9.17

- new `strict` feature, off by default; when active
    - removes all bridges from `usize` to indices
    - removes functions over maps that decrease their size
    - this means that, as long as you only create one map value, any index you manipulate is
        guaranteed to be legal for this map.
- safe_index is no `no_std`
- macro input
    - `map MapType` is not followed by `with iter IterType` anymore
    - `range RangeType` has been removed, use `..` and `..=` operators instead
- maps:
    - no dedicated iterator type anymore
    - function `of` constructing a map from a vector is gone, use the `From` implementation
        instead;
    - `last` now also returns the index of the last element, if any;
    - `last_mut` has been fixed and also returns the last element's index;
    - new `push_idx` function: like `push`, but takes an element constructor taking the element's
        index as argument;
    - `split` now produces iterators that yield indices
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