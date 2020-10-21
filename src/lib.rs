//! Strongly-typed, zero-cost indexes wrapping integers.
//!
//! This crate is just one macro: [`new`]. It creates a wrapper around `usize` to make
//! type-safe indexes. That is, the indexes for your clients that you use to retrieve information
//! efficiently from the vector of client information do not have the same type as the indexes for
//! the files you have about your clients. [The example below](#example) illustrates this crate in
//! that context.
//!
//! The index type created implements
//!
//! - `Deref` and `From` for `usize`,
//! - `Debug`, `Clone`, `Copy`, `PartialOrd`, `Ord`, `PartialEq`, `Eq`, `Hash` and `Display`.
//!
//! # Usage
//!
//! The most basic use of `new` is just to wrap something:
//!
//! ```
//! safe_index::new!{
//!     /// Arity.
//!     Arity
//! }
//! assert_eq! { std::mem::size_of::<Arity>(), std::mem::size_of::<usize>() }
//! ```
//!
//! This is not very useful however, there's nothing for our index to index. Thankfully `new`
//! can provide more types. After the mandatory identifier `Idx` for the type of indexes, you can
//! add these:
//!
//! - `range <Range>`: creates an iterator named `<Range>` between two `Idx`s (the upper bound is
//!   exclusive). If this constructor is present, `Idx` will have a `up_to` function that creates a
//!   range between two `Idx`s. This constructor can only appear once.
//! - `map <Map> with iter: <MapIter>`: creates a wrapper named `<Map>` around a vector, indexed by
//!   `Idx`. `<MapIter>` is the type of iterators over `<Map>`.
//! - `btree set <Set>`: alias type for a binary tree set of `Idx`s.
//! - `btree map <Map>`: alias type for a binary tree map from `Idx` to something.
//!
//!
//! See the [`examples` module] and the example below for illustrations of the `new` macro.
//!
//! # Example
//!
//! All the code for this example is in [`examples::clients`]. Say we have a `Data` structure that
//! stores some clients in a vector. It also stores files about these clients. A client can be
//! associated to several files, and a file can be about several clients. Let's handle everything
//! by indexes:
//!
//! ```rust
//! # use std::collections::BTreeSet;
//! /// Client information.
//! pub struct ClientInfo {
//!     /// Name of the client.
//!     pub name: String,
//!     /// Indices of files associated with the client.
//!     pub files: BTreeSet<usize>,
//! }
//! /// File information.
//! pub struct FileInfo {
//!     /// Name of the file.
//!     pub name: String,
//!     /// Indices of clients concerned by the file.
//!     pub clients: BTreeSet<usize>,
//! }
//!
//! /// Aggregates clients and files info.
//! pub struct Data {
//!     /// Map from client indexes to client information.
//!     pub clients: Vec<ClientInfo>,
//!     /// Map from file indexes to file information.
//!     pub files: Vec<FileInfo>,
//! }
//! ```
//!
//! Now, implementing `Data`'s functionalities is going to be painful. Client and file indexes are
//! both `usize`, terrible things are bound to happen.
//!
//! So let's instead create an index type for each.
//!
//! ```rust
//! /// Indices.
//! pub mod idx {
//!     safe_index::new! {
//!         /// Indices of clients.
//!         Client,
//!         /// Map from clients to something (really a vector).
//!         map: Clients with iter: ClientIter,
//!         /// Set of clients.
//!         btree set: ClientSet,
//!     }
//!
//!     safe_index::new! {
//!         /// Indices of files.
//!         File,
//!         /// Map from files to something (really a vector).
//!         map: Files with iter: FileIter,
//!         /// Set of files.
//!         btree set: FileSet,
//!     }
//! }
//!
//! use idx::*;
//!
//! # use std::collections::BTreeSet;
//! /// Client information.
//! pub struct ClientInfo {
//!     /// Name of the client.
//!     pub name: String,
//!     /// Indices of files associated with the client.
//!     pub files: ClientSet,
//! }
//! /// File information.
//! pub struct FileInfo {
//!     /// Name of the file.
//!     pub name: String,
//!     /// Indices of clients concerned by the file.
//!     pub clients: FileSet,
//! }
//!
//! /// Aggregates clients and files info.
//! pub struct Data {
//!     /// Map from client indexes to client information.
//!     pub clients: Clients<ClientInfo>,
//!     /// Map from file indexes to file information.
//!     pub files: Files<FileInfo>,
//! }
//! ```
//!
//! The full code is available [here][clients src], and you can see it used in the documentation of
//! [`examples::clients`]. Here are a few functions on `Data` to (hopefully) show that `Client` and
//! `File` behave as (and in fact are) `usize` indexes.
//!
//! ```rust
//! # use safe_index::examples::clients::{idx::*, ClientInfo, FileInfo};
//! /// Aggregates clients and files info.
//! pub struct Data {
//!     /// Map from client indexes to client information.
//!     pub clients: Clients<ClientInfo>,
//!     /// Map from file indexes to file information.
//!     pub files: Files<FileInfo>,
//! }
//! impl Data {
//!     /// Adds a file, updates the clients concerned.
//!     pub fn add_file(&mut self, file: FileInfo) -> File {
//!         let idx = self.files.next_index();
//!         for client in &file.clients {
//!             let is_new = self.clients[*client].files.insert(idx);
//!             debug_assert! { is_new }
//!         }
//!         let nu_idx = self.files.push(file);
//!         debug_assert_eq! { idx, nu_idx }
//!         idx
//!     }
//!
//!     /// Adds a client to a file.
//!     pub fn add_client_to_file(&mut self, client: Client, file: File) {
//!         let is_new = self.files[file].clients.insert(client);
//!         debug_assert! { is_new }
//!         let is_new = self.clients[client].files.insert(file);
//!         debug_assert! { is_new }
//!     }
//! }
//! ```
//!
//! [`new`]: ../../macro.new.html (new macro)
//! [`examples` module]: examples/index.html (safe_index examples)
//! [`examples::clients`]: examples/clients/index.html (clients example)
//! [clients src]: examples/clients.rs.html (Code of the clients example)

/// Wraps a `usize` into a struct (zero-cost). Also generates the relevant collections indexed by
/// the wrapper.
///
/// See the [module-level documentation](index.html) for more.
#[macro_export]
macro_rules! new {
    // Btree set (internal).
    ( @internal $t:ident, $(#[$cmt:meta])* btree set: $set:ident $($tail:tt)* ) => (
        $(#[$cmt])*
        pub type $set = std::collections::BTreeSet<$t> ;
        $crate::new!{ @internal $t $($tail)* }
    ) ;

    // Btree map (internal).
    ( @internal $t:ident, $(#[$cmt:meta])* btree map: $map:ident $($tail:tt)* ) => (
        $(#[$cmt])*
        pub type $map<T> = std::collections::BTreeMap<$t, T> ;
        $crate::new!{ @internal $t $($tail)* }
    ) ;

    // Range (internal).
    ( @internal $t:ident, $(#[$cmt:meta])* range: $range:ident $($tail:tt)* ) => (
        impl $t {
            /// Creates a range between two indexes (upper bound is exclusive).
            pub fn up_to(self, end : $t) -> $range {
                $range { start: self, end }
            }
        }
        $(#[$cmt])*
        #[derive(Debug)]
        pub struct $range {
            start: $t,
            end: $t,
        }
        impl $range {
            /// Creates a new range, exclusive on the upper bound.
            pub fn new<
                T1: std::convert::Into<$t>,
                T2: std::convert::Into<$t>,
            >(start: T1, end: T2) -> Self {
                $range { start: start.into(), end: end.into() }
            }
            /// Creates a range from `0` to something.
            pub fn zero_to<T: std::convert::Into<$t>>(end: T) -> Self {
                $range { start: 0.into(), end: end.into() }
            }
        }
        impl std::iter::Iterator for $range {
            type Item = $t ;
            fn next(&mut self) -> Option<$t> {
                if self.start >= self.end { None } else {
                    let res = Some(self.start) ;
                    self.start.val += 1 ;
                    res
                }
            }
        }
        $crate::new!{ @internal $t $($tail)* }
    ) ;

    // Map: vector indexed by `$t` (internal).
    (
        @internal $t:ident, $(#[$cmt:meta])*
        map: $map:ident with iter: $iter:ident
        $($tail:tt)*
    ) => (
        $(#[$cmt])*
        #[derive(Debug, PartialOrd, Ord)]
        pub struct $map<T> {
            vec: Vec<T>
        }
        impl<T> Default for $map<T> {
            fn default() -> Self { Self::new() }
        }
        impl<T: Clone> Clone for $map<T> {
            fn clone(& self) -> Self {
                $map { vec: self.vec.clone() }
            }
        }
        impl<T> $map<T> {
            /// Creates an empty map from an existing one.
            #[inline]
            pub fn of(vec: Vec<T>) -> Self {
                $map { vec: vec }
            }
            /// Creates an empty map.
            #[inline]
            pub fn new() -> Self {
                $map { vec: Vec::new() }
            }
            /// Creates an empty map with some capacity.
            #[inline]
            pub fn with_capacity(capacity: usize) -> Self {
                $map { vec: Vec::with_capacity(capacity) }
            }

            /// Reserves some space for the map.
            #[inline]
            pub fn reserve(&mut self, capa: usize) {
                self.vec.reserve(capa)
            }

            /// Retrieves an entry in the map.
            #[inline]
            pub fn get(&self, uid: $t) -> Option<&T> {
                self.vec.get(uid.get())
            }
            /// Retrieves an entry in the map.
            #[inline]
            pub fn get_mut(&mut self, uid: $t) -> Option<&mut T> {
                self.vec.get_mut(uid.get())
            }

            /// Retrieves the last entry in the map.
            #[inline]
            pub fn last(&self) -> Option<&T> {
                self.vec.last()
            }
            /// Retrieves the last entry in the map.
            #[inline]
            pub fn last_mut(&self) -> Option<&T> {
                self.vec.last()
            }

            /// Number of elements in the map.
            #[inline]
            pub fn len(& self) -> usize {
                self.vec.len()
            }
            /// Capacity of the map.
            #[inline]
            pub fn capacity(& self) -> usize {
                self.vec.capacity()
            }

            /// The next free index (wrapped `self.len()`).
            #[inline]
            pub fn next_index(& self) -> $t {
                self.len().into()
            }
            /// The last index in the map.
            #[inline]
            pub fn last_index(& self) -> Option<$t> {
                let len = self.len();
                if len > 0 { Some((len - 1).into()) } else { None }
            }

            /// Pushes an element.
            #[inline]
            pub fn push(&mut self, elem: T) -> $t {
                let idx = self.next_index();
                self.vec.push(elem);
                idx
            }
            /// Pops an element.
            ///
            /// This function is unsafe for the logics of safe indices. This function voids indices
            /// previously created (indices for the last element on entry) and should be used with
            /// great care.
            #[inline]
            pub fn pop(&mut self) -> Option<T> {
                self.vec.pop()
            }

            /// Clears a map.
            #[inline]
            pub fn clear(&mut self) {
                self.vec.clear()
            }

            /// Iterates over the elements.
            #[inline]
            pub fn iter(& self) -> std::slice::Iter<T> {
                self.vec.iter()
            }
            /// Iterates over the elements with the index.
            #[inline]
            pub fn index_iter<'a>(&'a self) -> $iter<&'a $map<T>>
            where T: 'a {
                $iter::mk_ref(self)
            }
            /// Iterates over the elements with the index, mutable version.
            #[inline]
            pub fn index_iter_mut<'a>(&'a mut self) -> $iter<
                std::slice::IterMut<'a, T>
            >
            where T: 'a {
                $iter::mk_ref_mut(self)
            }
            /// Iterates over the elements with the index.
            #[inline]
            pub fn into_index_iter(self) -> $iter<$map<T>> {
                $iter::new(self)
            }
            /// Iterates over the elements (mutable version).
            #[inline]
            pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
                self.vec.iter_mut()
            }

            /// Shrinks the capacity as much as possible.
            #[inline]
            pub fn shrink_to_fit(&mut self) {
                self.vec.shrink_to_fit()
            }
            /// Swap from `Vec`.
            #[inline]
            pub fn swap(&mut self, a: $t, b: $t) {
                self.vec.swap(* a, *b)
            }
            /// Swap remove from `Vec`.
            ///
            /// This function is unsafe for the logics of safe indices. This function voids indices
            /// previously created (indices for the last element on entry) and should be used with
            /// great care.
            #[inline]
            pub fn swap_remove(&mut self, idx: $t) -> T {
                self.vec.swap_remove(* idx)
            }

            /// Splits the map into the elements before and after some index.
            ///
            /// More precisely, returns a tuple of
            ///
            /// - an iterator over the elements *before* `idx`,
            /// - the element at position `idx`, and
            /// - an iterator over the elements *after* `idx`.
            #[inline]
            pub fn split(&self, idx: $t) -> (impl Iterator<Item = &T>, &T, impl Iterator<Item = &T>) {
                let before = self.vec[0..idx.val].iter();
                let after = if idx.val < self.vec.len() {
                    self.vec[idx.val + 1 ..].iter()
                } else {
                    self.vec[0..0].iter()
                };
                (before, &self.vec[idx.val], after)
            }
        }

        impl<T: Clone> $map<T> {
            /// Creates an empty vector with some capacity.
            #[inline]
            pub fn of_elems(elem: T, size: usize) -> Self {
                $map { vec: vec![ elem ; size ] }
            }
        }
        impl<T: PartialEq> PartialEq for $map<T> {
            fn eq(& self, other: & Self) -> bool {
                self.vec.eq( & other.vec )
            }
        }

        impl<T: Eq> Eq for $map<T> {}

        impl<T> std::convert::From< Vec<T> > for $map<T> {
            fn from(vec: Vec<T>) -> Self {
                $map { vec }
            }
        }
        impl<T> std::iter::IntoIterator for $map<T> {
            type Item = T ;
            type IntoIter = std::vec::IntoIter<T> ;
            fn into_iter(self) -> std::vec::IntoIter<T> {
                self.vec.into_iter()
            }
        }
        impl<'a, T> std::iter::IntoIterator for &'a $map<T> {
            type Item = &'a T ;
            type IntoIter = std::slice::Iter<'a, T> ;
            fn into_iter(self) -> std::slice::Iter<'a, T> {
                self.iter()
            }
        }
        impl<'a, T> std::iter::IntoIterator for &'a mut $map<T> {
            type Item = &'a mut T ;
            type IntoIter = std::slice::IterMut<'a, T> ;
            fn into_iter(self) -> std::slice::IterMut<'a, T> {
                self.iter_mut()
            }
        }
        impl<T> std::iter::FromIterator<T> for $map<T> {
            fn from_iter<
                I: std::iter::IntoIterator<Item = T>
            >(iter: I) -> Self {
                $map { vec: iter.into_iter().collect() }
            }
        }
        impl<T> std::ops::Index<$t> for $map<T> {
            type Output = T ;
            fn index(& self, index: $t) -> & T {
                & self.vec[ index.get() ]
            }
        }
        impl<T> std::ops::Index<std::ops::RangeFrom<$t>> for $map<T> {
            type Output = [T];
            fn index(& self, std::ops::RangeFrom { start }: std::ops::RangeFrom<$t>) -> &[T] {
                & self.vec[ start.get() .. ]
            }
        }
        impl<T> std::ops::Index<std::ops::Range<$t>> for $map<T> {
            type Output = [T];
            fn index(& self, std::ops::Range { start, end }: std::ops::Range<$t>) -> &[T] {
                & self.vec[ start.get() .. end.get() ]
            }
        }
        impl<T> std::ops::Index<std::ops::RangeInclusive<$t>> for $map<T> {
            type Output = [T];
            fn index(& self, range: std::ops::RangeInclusive<$t>) -> &[T] {
                & self.vec[ range.start().get() ..= range.end().get() ]
            }
        }
        impl<T> std::ops::Index<std::ops::RangeFull> for $map<T> {
            type Output = [T];
            fn index(& self, _: std::ops::RangeFull) -> &[T] {
                & self.vec[..]
            }
        }
        impl<T> std::ops::Index<std::ops::RangeTo<$t>> for $map<T> {
            type Output = [T];
            fn index(& self, std::ops::RangeTo { end }: std::ops::RangeTo<$t>) -> &[T] {
                & self.vec[..end.get()]
            }
        }
        impl<T> std::ops::Index<std::ops::RangeToInclusive<$t>> for $map<T> {
            type Output = [T];
            fn index(
                & self,
                std::ops::RangeToInclusive { end }: std::ops::RangeToInclusive<$t>
            ) -> &[T] {
                & self.vec[..=end.get()]
            }
        }
        impl<T> std::ops::IndexMut<$t> for $map<T> {
            fn index_mut(&mut self, index: $t) -> &mut T {
                &mut self.vec[ index.get() ]
            }
        }
        impl<T> std::ops::Index<
            std::ops::Range<usize>
        > for $map<T> {
            type Output = [T] ;
            fn index(& self, index: std::ops::Range<usize>) -> & [T] {
                self.vec.index(index)
            }
        }
        impl<T> std::ops::Index<
          std::ops::RangeInclusive<usize>
        > for $map<T> {
          type Output = [T] ;
          fn index(& self, index: std::ops::RangeInclusive<usize>) -> & [T] {
            self.vec.index(index)
          }
        }
        impl<T> std::ops::Index<
            std::ops::RangeFrom<usize>
        > for $map<T> {
            type Output = [T] ;
            fn index(& self, index: std::ops::RangeFrom<usize>) -> & [T] {
                self.vec.index(index)
            }
        }
        impl<T> std::ops::Index<
            std::ops::RangeTo<usize>
        > for $map<T> {
            type Output = [T] ;
            fn index(& self, index: std::ops::RangeTo<usize>) -> & [T] {
                self.vec.index(index)
            }
        }
        // impl<T> std::ops::Index<
        //   std::ops::RangeToInclusive<usize>
        // > for $map<T> {
        //   type Output = [T] ;
        //   fn index(& self, index: std::ops::RangeToInclusive<usize>) -> & [T] {
        //     self.vec.index(index)
        //   }
        // }
        impl<T> std::ops::Deref for $map<T> {
            type Target = Vec<T> ;
            fn deref(& self) -> & Vec<T> {
                & self.vec
            }
        }
        /// Structure allowing to iterate over the elements of a map and their
        /// index.
        #[derive(Clone)]
        pub struct $iter<T> {
            cursor: $t,
            map: T,
        }
        impl<'a, T> $iter<&'a $map<T>> {
            /// Creates an iterator starting at 0.
            fn mk_ref(map: &'a $map<T>) -> Self {
                $iter { cursor: $t::zero(), map: map }
            }
        }
        impl<'a, T: 'a> std::iter::Iterator for $iter<&'a $map<T>> {
            type Item = ($t, &'a T) ;
            fn next(&mut self) -> Option< ($t, &'a T) > {
                if self.cursor >= self.map.len() {
                    None
                } else {
                    let res = (self.cursor, & self.map[self.cursor]) ;
                    self.cursor.inc() ;
                    Some(res)
                }
            }
        }
        impl<'a, T: 'a> $iter<std::slice::IterMut<'a, T>> {
            /// Creates an iterator starting at 0, mutable version.
            fn mk_ref_mut(map: &'a mut $map<T>) -> Self {
                $iter { cursor: $t::zero(), map: map.vec.iter_mut() }
            }
        }
        impl<'a, T: 'a> std::iter::Iterator for $iter<
            std::slice::IterMut<'a, T>
        > {
            type Item = ($t, &'a mut T) ;
            fn next(&mut self) -> Option< ($t, &'a mut T) > {
                self.map.next().map(
                    |res| {
                        let index = self.cursor ;
                        self.cursor.inc() ;
                        (index, res)
                    }
                )
            }
        }
        impl<T> $iter<$map<T>> {
            /// Creates an iterator starting at 0.
            fn new(mut map: $map<T>) -> Self {
                map.vec.reverse() ;
                $iter { cursor: $t::zero(), map: map }
            }
        }
        impl<T> std::iter::Iterator for $iter<$map<T>> {
            type Item = ($t, T) ;
            fn next(&mut self) -> Option< ($t, T) > {
                if let Some(elem) = self.map.pop() {
                    let res = (self.cursor, elem) ;
                    self.cursor.inc() ;
                    Some(res)
                } else {
                    None
                }
            }
        }
        $crate::new!{ @internal $t $($tail)* }
    ) ;

    // Terminal case (internal).
    ( @internal $t:ident $(,)? ) => () ;

    // Entry point.
    (
        $(#[$cmt:meta])* $t:ident
        $($tail:tt)*
    ) => (
        $(#[$cmt])*
        #[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
        pub struct $t {
            val: usize
        }

        impl $t {
            /// Wraps an int.
            #[inline]
            pub fn new(val: usize) -> Self {
                $t { val: val }
            }
            /// Zero.
            #[inline]
            pub fn zero() -> Self {
                $t { val: 0 }
            }
            /// One.
            #[inline]
            pub fn one() -> Self {
                $t { val: 1 }
            }
            /// Accessor.
            #[inline]
            pub fn get(& self) -> usize {
                self.val
            }
            /// Increments the int.
            #[inline]
            pub fn inc(&mut self) {
                self.val += 1
            }
            /// Decrements the int.
            #[inline]
            pub fn dec(&mut self) {
                self.val -= 1
            }
        }
        impl std::convert::From<usize> for $t {
            #[inline]
            fn from(val: usize) -> Self {
                $t::new(val)
            }
        }
        impl<'a> std::convert::From<&'a usize> for $t {
            #[inline]
            fn from(val: &'a usize) -> Self {
                $t::new(* val)
            }
        }
        impl std::convert::Into<usize> for $t {
            #[inline]
            fn into(self) -> usize {
                self.val
            }
        }
        impl<'a> std::convert::Into<usize> for &'a $t {
            #[inline]
            fn into(self) -> usize {
                self.val
            }
        }
        impl<T: std::convert::Into<usize>> std::ops::AddAssign<T> for $t {
            #[inline]
            fn add_assign(&mut self, rhs: T) {
                self.val += rhs.into()
            }
        }
        impl<T: std::convert::Into<usize>> std::ops::Add<T> for $t {
            type Output = $t ;
            #[inline]
            fn add(mut self, rhs: T) -> $t {
                self.val += rhs.into() ;
                self
            }
        }
        impl std::ops::Deref for $t {
            type Target = usize ;
            #[inline]
            fn deref(& self) -> & usize {
                & self.val
            }
        }
        impl std::fmt::Display for $t {
            #[inline]
            fn fmt(& self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(fmt, "{}", self.val)
            }
        }
        impl std::cmp::PartialEq<usize> for $t {
            #[inline]
            fn eq(& self, int: & usize) -> bool {
                self.val.eq(int)
            }
        }
        impl std::cmp::PartialOrd<usize> for $t {
            #[inline]
            fn partial_cmp(& self, int: & usize) -> Option<
                std::cmp::Ordering
            > {
                self.val.partial_cmp(int)
            }
        }
        $crate::new!{ @internal $t $($tail)* }
    ) ;
}

pub mod examples;
