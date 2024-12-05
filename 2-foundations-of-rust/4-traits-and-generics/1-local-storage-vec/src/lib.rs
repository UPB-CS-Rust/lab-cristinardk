/// A growable, generic list that resides on the stack if it's small,
/// but is moved to the heap to grow larger if needed.
/// This list is generic over the items it contains as well as the
/// size of its buffer if it's on the stack.

pub enum LocalStorageVec<T, const N: usize> {
    Stack { buf: [T; N], len: usize },
    Heap(Vec<T>),
}

// **Below `From` implementation is used in the tests and are therefore given. However,
// you should have a thorough look at it as they contain various new concepts.**
// This implementation is generic not only over the type `T`, but also over the
// constants `N` and 'M', allowing us to support conversions from arrays of any
// length to `LocalStorageVec`s of with any stack buffer size.
// In Rust, we call this feature 'const generics'
impl<T, const N: usize, const M: usize> From<[T; N]> for LocalStorageVec<T, M>
where
    T: Default + Clone,
{
    fn from(array: [T; N]) -> Self {
        if N <= M {
            let mut buf = [(); M].map(|_| T::default());
            for (i, value) in array.iter().cloned().enumerate() {
                buf[i] = value;
            }
            Self::Stack { buf, len: N }
        } else {
            Self::Heap(Vec::from(array))
        }
    }
}

// Implementarea `From<Vec<T>>` pentru conversia unui vector in `LocalStorageVec`.
impl<T, const N: usize> From<Vec<T>> for LocalStorageVec<T, N>
where
    T: Default + Clone,
{
    fn from(vec: Vec<T>) -> Self {
        let len = vec.len(); //salvez dimensiunea inainte de a muta vec
        if len <= N {
            let mut buf = [(); N].map(|_| T::default());
            for (i, value) in vec.into_iter().enumerate() {
                buf[i] = value;
            }
            Self::Stack { buf, len }
        } else {
            Self::Heap(vec)
        }
    }
}

// Implementarea metodelor pentru `LocalStorageVec`.
impl<T, const N: usize> LocalStorageVec<T, N>
where
    T: Default + Clone,
{
    /// creez o lista goala
    pub fn new() -> Self {
        Self::Stack {
            buf: [(); N].map(|_| T::default()), //initializare buffer stack
            len: 0,
        }
    }

    //returnez dimensiunea listei
    pub fn len(&self) -> usize {
        match self {
            Self::Stack { len, .. } => *len,
            Self::Heap(vec) => vec.len(),
        }
    }

    /// elimin ultimul element
    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::Stack { buf, len } => {
                if *len == 0 {
                    None
                } else {
                    *len -= 1;
                    Some(std::mem::take(&mut buf[*len]))
                }
            }
            Self::Heap(vec) => vec.pop(),
        }
    }

    /// adaug un elem in lista
    pub fn push(&mut self, value: T) {
        match self {
            Self::Stack { buf, len } => {
                if *len < N {
                    buf[*len] = value;
                    *len += 1;
                } else {
                    let mut heap = Vec::with_capacity(N + 1);
                    heap.extend_from_slice(&buf[..*len]);
                    heap.push(value);
                    *self = Self::Heap(heap);
                }
            }
            Self::Heap(vec) => vec.push(value), //daca stack e plin, se muta elem in heap
        }
    }
}

// Implementarea `AsRef` pentru a permite conversia in referinte la slice-uri
impl<T, const N: usize> AsRef<[T]> for LocalStorageVec<T, N> {
    fn as_ref(&self) -> &[T] {
        match self {
            Self::Stack { buf, len } => &buf[..*len],
            Self::Heap(vec) => vec.as_slice(),
        }
    }
}

// Implementarea `AsMut` pentru a permite conversia in referinte mutable la slice-uri
impl<T, const N: usize> AsMut<[T]> for LocalStorageVec<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        match self {
            Self::Stack { buf, len } => &mut buf[..*len],
            Self::Heap(vec) => vec.as_mut_slice(),
        }
    }
}



#[cfg(test)]
mod test {
    use crate::LocalStorageVec;

    #[test]
    // Don't remove the #[ignore] attribute or your tests will take forever!
    #[ignore = "This test is just to validate the definition of `LocalStorageVec`. If it compiles, all is OK"]
    #[allow(unreachable_code, unused_variables)]
    fn it_compiles() {
        // Here's a trick to 'initialize' a type while not actually
        // creating a value: an infinite `loop` expression diverges
        // and evaluates to the 'never type' `!`, which, as is can never
        // actually be instantiated, coerces to any other type.
        // Some other ways of diverging are by calling the `panic!` or the `todo!`
        // macros.
        // More info:
        // - https://doc.rust-lang.org/rust-by-example/fn/diverging.html
        // - https://doc.rust-lang.org/reference/expressions/loop-expr.html#infinite-loops
        let vec: LocalStorageVec<u32, 10> = loop {};
        match vec {
            LocalStorageVec::Stack { buf, len } => {
                let _buf: [u32; 10] = buf;
                let _len: usize = len;
            }
            LocalStorageVec::Heap(v) => {
                let _v: Vec<u32> = v;
            }
        }
    }

    // Uncomment me for part B
     #[test]
     fn it_from_vecs() {
    //     // The `vec!` macro creates a `Vec<T>` in a way that resembles
    //     // array-initialization syntax.
        let vec: LocalStorageVec<usize, 10> = LocalStorageVec::from(vec![1, 2, 3]);
    //     // Assert that the call to `from` indeed yields a `Heap` variant
      assert!(matches!(vec, LocalStorageVec::Stack{..}));

        let vec: LocalStorageVec<usize, 2> = LocalStorageVec::from(vec![1, 2, 3]);
    //
        assert!(matches!(vec, LocalStorageVec::Heap(_)));
     }

    // Uncomment me for part C
     #[test]
     fn it_as_refs() {
         let vec: LocalStorageVec<i32, 256> = LocalStorageVec::from([0; 128]);
         let slice: &[i32] = vec.as_ref();
        assert!(slice.len() == 128);
       let vec: LocalStorageVec<i32, 32> = LocalStorageVec::from([0; 128]);
        let slice: &[i32] = vec.as_ref();
         assert!(slice.len() == 128);
    
         let mut vec: LocalStorageVec<i32, 256> = LocalStorageVec::from([0; 128]);
         let slice_mut: &[i32] = vec.as_mut();
         assert!(slice_mut.len() == 128);
         let mut vec: LocalStorageVec<i32, 32> = LocalStorageVec::from([0; 128]);
         let slice_mut: &[i32] = vec.as_mut();
         assert!(slice_mut.len() == 128);
     }

    // Uncomment me for part D
     #[test]
     fn it_constructs() {
         let vec: LocalStorageVec<usize, 10> = LocalStorageVec::new();
         // Assert that the call to `new` indeed yields a `Stack` variant with zero length
        assert!(matches!(vec, LocalStorageVec::Stack { buf: _, len: 0 }));
     }

    // Uncomment me for part D
     #[test]
     fn it_lens() {
         let vec: LocalStorageVec<_, 3> = LocalStorageVec::from([0, 1, 2]);
         assert_eq!(vec.len(), 3);
         let vec: LocalStorageVec<_, 2> = LocalStorageVec::from([0, 1, 2]);
        assert_eq!(vec.len(), 3);
     }

     //Uncomment me for part D
     #[test]
     fn it_pushes() {
         let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::new();
         for value in 0..128 {
             vec.push(value);
         }
         assert!(matches!(vec, LocalStorageVec::Stack { len: 128, .. }));
        for value in 128..256 {
            vec.push(value);
         }
        assert!(matches!(vec, LocalStorageVec::Heap(v) if v.len() == 256))
     }

    // Uncomment me for part D
     #[test]
     fn it_pops() {
         let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 128]);
        for _ in 0..128 {
           assert_eq!(vec.pop(), Some(0))
        }
        assert_eq!(vec.pop(), None);
    
        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 256]);
        for _ in 0..256 {
            assert_eq!(vec.pop(), Some(0))
        }
        assert_eq!(vec.pop(), None);
    
        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from(vec![0; 256]);
        for _ in 0..256 {
            assert_eq!(vec.pop(), Some(0))
        }
         assert_eq!(vec.pop(), None);
     }

    // Uncomment me for part D
     /*#[test]
     fn it_inserts() {
        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2]);
        vec.insert(1, 3);
        assert!(matches!(
            vec,
            LocalStorageVec::Stack {
                buf: [0, 3, 1, 2],
                len: 4
            }
        ));
    
        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2, 3]);
        vec.insert(1, 3);
        assert!(matches!(vec, LocalStorageVec::Heap { .. }));
        assert_eq!(vec.as_ref(), &[0, 3, 1, 2, 3]);
    
        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2, 3, 4]);
        vec.insert(1, 3);
        assert!(matches!(vec, LocalStorageVec::Heap { .. }));
        assert_eq!(vec.as_ref(), &[0, 3, 1, 2, 3, 4])
    }*/

    // Uncomment me for part D
    // #[test]
    // fn it_removes() {
    //     let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2]);
    //     let elem = vec.remove(1);
    //     dbg!(&vec);
    //     assert!(matches!(
    //         vec,
    //         LocalStorageVec::Stack {
    //             buf: [0, 2, _, _],
    //             len: 2
    //         }
    //     ));
    //     assert_eq!(elem, 1);
    //
    //     let mut vec: LocalStorageVec<_, 2> = LocalStorageVec::from([0, 1, 2]);
    //     let elem = vec.remove(1);
    //     assert!(matches!(vec, LocalStorageVec::Heap(..)));
    //     assert_eq!(vec.as_ref(), &[0, 2]);
    //     assert_eq!(elem, 1);
    // }

    // Uncomment me for part D
    // #[test]
    // fn it_clears() {
    //     let mut vec: LocalStorageVec<_, 10> = LocalStorageVec::from([0, 1, 2, 3]);
    //     assert!(matches!(vec, LocalStorageVec::Stack { buf: _, len: 4 }));
    //     vec.clear();
    //     assert_eq!(vec.len(), 0);
    //
    //     let mut vec: LocalStorageVec<_, 3> = LocalStorageVec::from([0, 1, 2, 3]);
    //     assert!(matches!(vec, LocalStorageVec::Heap(_)));
    //     vec.clear();
    //     assert_eq!(vec.len(), 0);
    // }

    // Uncomment me for part E
    // #[test]
    // fn it_iters() {
    //     let vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 32]);
    //     let mut iter = vec.into_iter();
    //     for item in &mut iter {
    //         assert_eq!(item, 0);
    //     }
    //     assert_eq!(iter.next(), None);
    //
    //     let vec: LocalStorageVec<_, 128> = LocalStorageVec::from(vec![0; 128]);
    //     let mut iter = vec.into_iter();
    //     for item in &mut iter {
    //         assert_eq!(item, 0);
    //     }
    //     assert_eq!(iter.next(), None);
    // }

    // Uncomment me for part F
    // #[test]
    // fn it_indexes() {
    //     let vec: LocalStorageVec<i32, 10> = LocalStorageVec::from([0, 1, 2, 3, 4, 5]);
    //     assert_eq!(vec[1], 1);
    //     assert_eq!(vec[..2], [0, 1]);
    //     assert_eq!(vec[4..], [4, 5]);
    //     assert_eq!(vec[1..3], [1, 2]);
    // }

    // Uncomment me for part H
    // #[test]
    // fn it_borrowing_iters() {
    //     let vec: LocalStorageVec<String, 10> = LocalStorageVec::from([
    //         "0".to_owned(),
    //         "1".to_owned(),
    //         "2".to_owned(),
    //         "3".to_owned(),
    //         "4".to_owned(),
    //         "5".to_owned(),
    //     ]);
    //     let iter = vec.iter();
    //     for _ in iter {}
    //     // This requires the `vec` not to be consumed by the call to `iter()`
    //     drop(vec);
    // }

    // Uncomment me for part J
    // #[test]
     /*fn it_derefs() {
         use std::ops::{Deref, DerefMut};
        let vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 128]);
        // `chunks` is a method that's defined for slices `[T]`, that we can use thanks to `Deref`
        let chunks = vec.chunks(4);
        let slice: &[_] = vec.deref();
    //
        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 128]);
        let chunks = vec.chunks_mut(4);
         let slice: &mut [_] = vec.deref_mut();
     }*/
}
