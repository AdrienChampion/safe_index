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