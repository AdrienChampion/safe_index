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
//! - `Debug`, `Default`, `Clone`, `Copy`, `PartialOrd`, `Ord`, `PartialEq`, `Eq`, `Hash` and `Display`.
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
//! assert_eq! { core::mem::size_of::<Arity>(), core::mem::size_of::<usize>() }
//! ```
//!
//! This is not very useful however, there's nothing for our index to index. Thankfully `new`
//! can provide more types. After the mandatory identifier `Idx` for the type of indexes, you can
//! add these:
//!
//! - `map <Map>`: creates a wrapper named `<Map>` around a vector, indexed by `Idx`.
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
//!         map: Clients,
//!         /// Set of clients.
//!         btree set: ClientSet,
//!     }
//!
//!     safe_index::new! {
//!         /// Indices of files.
//!         File,
//!         /// Map from files to something (really a vector).
//!         map: Files,
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
//!         let idx = self.files.push(file);
//!         let file = &self.files[idx];
//!         for client in &file.clients {
//!             let is_new = self.clients[*client].files.insert(idx);
//!             debug_assert! { is_new }
//!         }
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

#![no_std]

pub extern crate alloc;

mod map;

/// Discards its input if the `strict` feature is active.
#[macro_export]
#[doc(hidden)]
#[cfg(feature = "strict")]
macro_rules! non_strict {
    ( $($stuff:tt)* ) => {};
}
/// Discards its input if the `strict` feature is active.
#[macro_export]
#[doc(hidden)]
#[cfg(not(feature = "strict"))]
macro_rules! non_strict {
    (
        #[doc = $doc:literal]
        $($tail:tt)*
    ) => {
        #[doc = concat!("[`non_strict`] ", $doc)]
        $($tail)*
    };
    (
        $($item:item)*
    ) => {
        $(
            #[doc = "[`non_strict`] "]
            $item
        )*
    };
}

/// Generates an alias type for [`alloc::collections::BTreeSet`] of indices.
#[macro_export]
#[doc(hidden)]
macro_rules! btree_set_codegen {
    { $t:ident,
        $(#[$meta:meta])*
        $set:ident $($tail:tt)*
    } => {
        $(#[$meta])*
        pub type $set = $crate::alloc::collections::BTreeSet<$t> ;
        $crate::handle!{ $t $($tail)* }
    };
}

/// Generates an alias type for [`alloc::collections::BTreeMap`] of indices.
#[macro_export]
#[doc(hidden)]
macro_rules! btree_map_codegen {
    { $t:ident,
        $(#[$meta:meta])*
        $map:ident $($tail:tt)*
    } => {
        $(#[$meta])*
        pub type $map<T> = $crate::alloc::collections::BTreeMap<$t, T> ;
        $crate::handle!{ $t $($tail)* }
    };
}

/// Handles some user input and decides what to do.
#[macro_export]
#[doc(hidden)]
macro_rules! handle {
    { $t:ident, $(#[$meta:meta])* btree set: $($tail:tt)* } => {
        $crate::btree_set_codegen! { $t, $(#[$meta])* $($tail)* }
    };
    { $t:ident, $(#[$meta:meta])* btree map: $($tail:tt)* } => {
        $crate::btree_map_codegen! { $t, $(#[$meta])* $($tail)* }
    };
    { $t:ident, $(#[$meta:meta])* map: $($tail:tt)* } => {
        $crate::map_codegen! { $t, $(#[$meta])* $($tail)* }
    };
    { $t:ident $(,)? } => {};

    { $t:ident with iter: $iter:ident $($tail:tt)* } => {
        compile_error!(concat!(
            "maps do not have dedicated iterator structures anymore, remove `with iter: ",
            stringify!($iter),
            "` from your input",
        ));
    };
    { $t:ident, range: $range:ident $($tail:tt)* } => {
        compile_error!(concat!(
            "`range` does not exist anymore, use `..` and `..=` operators instead and remove `range: ",
            stringify!($range),
            "` from your input",
        ));
    };
    { $t:ident, $token:tt $($tail:tt)* } => {
        compile_error!(concat!(
            "expected `btree set`, `btree map` or `map` but found unexpected token `",
            stringify!($token),
            "`",
        ));
    };
    { $t:ident $token:tt $($tail:tt)* } => {
        compile_error!(concat!(
            "expected comma, found unexpected token `",
            stringify!($token),
            "`",
        ));
    };
}

/// Wraps a `usize` into a struct (zero-cost). Also generates the relevant collections indexed by
/// the wrapper.
///
/// See the [module-level documentation](index.html) for more.
#[macro_export]
macro_rules! new {
    (
        $(#[$meta:meta])*
        $t:ident
        $($tail:tt)*
    ) => (
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
        pub struct $t {
            val: usize
        }

        impl $t {
            $crate::non_strict! {
                /// Wraps a [`usize`].
                #[inline]
                pub const fn new(val: usize) -> Self {
                    $t { val: val }
                }
            }
            $crate::non_strict! {
                /// Zero.
                #[inline]
                pub const fn zero() -> Self {
                    $t { val: 0 }
                }
            }
            $crate::non_strict! {
                /// One.
                #[inline]
                pub const fn one() -> Self {
                    $t { val: 1 }
                }
            }
            $crate::non_strict! {
                /// Increments the int.
                #[inline]
                pub fn inc(&mut self) {
                    self.val += 1
                }
            }
            $crate::non_strict! {
                /// Decrements the int.
                #[inline]
                pub fn dec(&mut self) {
                    self.val -= 1
                }
            }
            /// Underlying index accessor.
            #[inline]
            pub const fn get(& self) -> usize {
                self.val
            }
        }
        impl core::convert::Into<usize> for $t {
            #[inline]
            fn into(self) -> usize {
                self.val
            }
        }
        impl<'a> core::convert::Into<usize> for &'a $t {
            #[inline]
            fn into(self) -> usize {
                self.val
            }
        }
        impl core::ops::Deref for $t {
            type Target = usize ;
            #[inline]
            fn deref(& self) -> & usize {
                & self.val
            }
        }
        impl core::fmt::Display for $t {
            #[inline]
            fn fmt(& self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(fmt, "{}", self.val)
            }
        }
        impl core::cmp::PartialEq<usize> for $t {
            #[inline]
            fn eq(& self, int: & usize) -> bool {
                self.val.eq(int)
            }
        }
        impl core::cmp::PartialOrd<usize> for $t {
            #[inline]
            fn partial_cmp(& self, int: & usize) -> Option<
                core::cmp::Ordering
            > {
                self.val.partial_cmp(int)
            }
        }
        $crate::non_strict! {
            impl<T: core::convert::Into<usize>> core::ops::Add<T> for $t {
                type Output = $t ;
                #[inline]
                fn add(mut self, rhs: T) -> $t {
                    self.val += rhs.into() ;
                    self
                }
            }
            impl core::convert::From<usize> for $t {
                #[inline]
                fn from(val: usize) -> Self {
                    $t::new(val)
                }
            }
            impl<'a> core::convert::From<&'a usize> for $t {
                #[inline]
                fn from(val: &'a usize) -> Self {
                    $t::new(* val)
                }
            }
            impl<T: core::convert::Into<usize>> core::ops::AddAssign<T> for $t {
                #[inline]
                fn add_assign(&mut self, rhs: T) {
                    self.val += rhs.into()
                }
            }
            impl Default for $t {
                #[inline]
                fn default() -> Self {
                    Self::zero()
                }
            }
        }
        $crate::handle!{ $t $($tail)* }
    ) ;
}

pub mod examples;
