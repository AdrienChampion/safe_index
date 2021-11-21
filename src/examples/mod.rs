//! Examples of using `safe_index`.
//!
//! Do not use the types in this module.

/// A basic example.
///
/// ```
/// safe_index::new!{
///     /// Index of a variable.
///     VarIndex,
///     /// Set of variable indexes.
///     btree set: VarBSet,
///     /// Map of variable indexes.
///     btree map: VarBMap,
///     /// Vector indexed by variable indexes.
///     map: VarMap,
/// }
/// fn main() {
///     use std::mem::size_of;
///     assert_eq!( size_of::<VarIndex>(), size_of::<usize>() );
///     assert_eq!( size_of::<VarMap<String>>(), size_of::<Vec<String>>() );
///
///     let mut var_values = VarMap::with_capacity(3);
///     let v_0 = var_values.push(7);
///     let v_1 = var_values.push(3);
///     let v_2 = var_values.push(11);
///     assert_eq! { var_values[v_0], 7  }
///     assert_eq! { var_values[v_1], 3  }
///     assert_eq! { var_values[v_2], 11 }
///
///     let mut iter = var_values[v_0..v_2].into_iter();
///     assert_eq! { iter.next(), Some(&7)  }
///     assert_eq! { iter.next(), Some(&3)  }
///     assert_eq! { iter.next(), None      }
///     let mut iter = var_values[v_0..=v_2].into_iter();
///     assert_eq! { iter.next(), Some(&7)  }
///     assert_eq! { iter.next(), Some(&3)  }
///     assert_eq! { iter.next(), Some(&11) }
///     assert_eq! { iter.next(), None      }
///     let mut iter = var_values[..=v_2].into_iter();
///     assert_eq! { iter.next(), Some(&7)  }
///     assert_eq! { iter.next(), Some(&3)  }
///     assert_eq! { iter.next(), Some(&11) }
///     assert_eq! { iter.next(), None      }
///     let mut iter = var_values[..=v_2].into_iter();
///     assert_eq! { iter.next(), Some(&7)  }
///     assert_eq! { iter.next(), Some(&3)  }
///     assert_eq! { iter.next(), Some(&11) }
///     assert_eq! { iter.next(), None      }
///
///     let mut check = vec![11, 3, 7];
///     for val in &var_values {
///         assert_eq! { *val, check.pop().unwrap() }
///     }
///
///     let mut check = vec![(v_2, 11), (v_1, 3), (v_0, 7)];
///     for (idx, val) in var_values.index_iter() {
///         let (i, v) = check.pop().unwrap();
///         assert_eq! {  idx, i }
///         assert_eq! { *val, v }
///     }
///
///     let mut check = vec![11, 3, 7];
///     for idx in var_values.indices() {
///         assert_eq! { var_values[idx], check.pop().unwrap() }
///     }
///
///     var_values.swap(v_0, v_2);
///     assert_eq! { var_values[v_0], 11 }
///     assert_eq! { var_values[v_1], 3  }
///     assert_eq! { var_values[v_2], 7  }
/// }
/// ```
pub mod basic {
    new! {
        /// Index of a variable.
        VarIndex,
        /// Set of variable indexes.
        btree set: VarBSet,
        /// Map of variable indexes.
        btree map: VarBMap,
        /// Vector indexed by variable indexes.
        map: VarMap,
    }

    #[test]
    fn run() {
        use core::mem::size_of;
        assert_eq!(size_of::<VarIndex>(), size_of::<usize>());
        assert_eq!(
            size_of::<VarMap<alloc::string::String>>(),
            size_of::<alloc::vec::Vec<alloc::string::String>>()
        );

        let mut var_values = VarMap::with_capacity(3);
        let v_0 = var_values.push(7);
        let v_1 = var_values.push(3);
        let v_2 = var_values.push(11);
        assert_eq! { var_values[v_0], 7  }
        assert_eq! { var_values[v_1], 3  }
        assert_eq! { var_values[v_2], 11 }

        let mut iter = var_values[v_0..v_2].into_iter();
        assert_eq! { iter.next(), Some(&7)  }
        assert_eq! { iter.next(), Some(&3)  }
        assert_eq! { iter.next(), None      }
        let mut iter = var_values[v_0..=v_2].into_iter();
        assert_eq! { iter.next(), Some(&7)  }
        assert_eq! { iter.next(), Some(&3)  }
        assert_eq! { iter.next(), Some(&11) }
        assert_eq! { iter.next(), None      }
        let mut iter = var_values[..=v_2].into_iter();
        assert_eq! { iter.next(), Some(&7)  }
        assert_eq! { iter.next(), Some(&3)  }
        assert_eq! { iter.next(), Some(&11) }
        assert_eq! { iter.next(), None      }
        let mut iter = var_values[..=v_2].into_iter();
        assert_eq! { iter.next(), Some(&7)  }
        assert_eq! { iter.next(), Some(&3)  }
        assert_eq! { iter.next(), Some(&11) }
        assert_eq! { iter.next(), None      }

        let mut check = alloc::vec![11, 3, 7];
        for val in &var_values {
            assert_eq! { *val, check.pop().unwrap() }
        }

        let mut check = alloc::vec![(v_2, 11), (v_1, 3), (v_0, 7)];
        for (idx, val) in var_values.index_iter() {
            let (i, v) = check.pop().unwrap();
            assert_eq! {  idx, i }
            assert_eq! { *val, v }
        }

        let mut check = alloc::vec![11, 3, 7];
        for idx in var_values.indices() {
            assert_eq! { var_values[idx], check.pop().unwrap() }
        }

        var_values.swap(v_0, v_2);
        assert_eq! { var_values[v_0], 11 }
        assert_eq! { var_values[v_1], 3  }
        assert_eq! { var_values[v_2], 7  }
    }
}

pub mod clients;
