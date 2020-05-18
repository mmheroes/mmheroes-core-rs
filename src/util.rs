#![macro_use]

use core::cmp::Ordering;
use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::iter::{FromIterator, IntoIterator};
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};

/// https://stackoverflow.com/a/36259524
macro_rules! array {
    (@as_expr $e:expr) => { $e };

    (@accum (0, $($_es:expr),*) -> ($($body:tt)*)) => {
        array!(@as_expr [$($body)*])
    };

    (@accum (1, $($es:expr),*) -> ($($body:tt)*)) => {
        array!(@accum (0, $($es),*) -> ($($body)* $($es,)*))
    };

    (@accum (2, $($es:expr),*) -> ($($body:tt)*)) => {
        array!(@accum (0, $($es),*) -> ($($body)* $($es,)* $($es,)*))
    };

    (@accum (4, $($es:expr),*) -> ($($body:tt)*)) => {
        array!(@accum (2, $($es,)* $($es),*) -> ($($body)*))
    };

    (@accum (8, $($es:expr),*) -> ($($body:tt)*)) => {
        array!(@accum (4, $($es,)* $($es),*) -> ($($body)*))
    };

    (@accum (16, $($es:expr),*) -> ($($body:tt)*)) => {
        array!(@accum (8, $($es,)* $($es),*) -> ($($body)*))
    };

    (@accum (32, $($es:expr),*) -> ($($body:tt)*)) => {
        array!(@accum (16, $($es,)* $($es),*) -> ($($body)*))
    };

    (@accum (64, $($es:expr),*) -> ($($body:tt)*)) => {
        array!(@accum (32, $($es,)* $($es),*) -> ($($body)*))
    };

    (@accum (128, $($es:expr),*) -> ($($body:tt)*)) => {
        array!(@accum (64, $($es,)* $($es),*) -> ($($body)*))
    };

    (@accum (256, $($es:expr),*) -> ($($body:tt)*)) => {
        array!(@accum (128, $($es,)* $($es),*) -> ($($body)*))
    };

    (@accum (512, $($es:expr),*) -> ($($body:tt)*)) => {
        array!(@accum (256, $($es,)* $($es),*) -> ($($body)*))
    };

    (@accum (1024, $($es:expr),*) -> ($($body:tt)*)) => {
        array!(@accum (512, $($es,)* $($es),*) -> ($($body)*))
    };

    (@accum (2048, $($es:expr),*) -> ($($body:tt)*)) => {
        array!(@accum (1024, $($es,)* $($es),*) -> ($($body)*))
    };

    (@accum (4096, $($es:expr),*) -> ($($body:tt)*)) => {
        array!(@accum (2048, $($es,)* $($es),*) -> ($($body)*))
    };

    (@accum (8192, $($es:expr),*) -> ($($body:tt)*)) => {
        array!(@accum (4096, $($es,)* $($es),*) -> ($($body)*))
    };

    [$e:expr; $n:tt] => {
        array!(@accum ($n, $e) -> ())
    };
}

pub trait TinyVecStorage {
    type Element;
    fn as_slice(&self) -> &[MaybeUninit<Self::Element>];
    fn as_mut_slice(&mut self) -> &mut [MaybeUninit<Self::Element>];
}

pub(crate) struct TinyVec<Storage: TinyVecStorage> {
    storage: Storage,
    count: usize,
}

impl<Storage: TinyVecStorage> TinyVec<Storage> {
    pub(crate) fn len(&self) -> usize {
        self.count
    }
}

macro_rules! tiny_vec_ty {
    ($ty:ty; $capacity:literal) => {
        $crate::util::TinyVec::<[core::mem::MaybeUninit<$ty>; $capacity]>
    };
}

macro_rules! __tiny_vec_implementation {
    ($capacity:tt) => {
        impl<T> TinyVecStorage for [MaybeUninit<T>; $capacity] {
            type Element = T;

            fn as_slice(&self) -> &[MaybeUninit<Self::Element>] {
                self.as_ref()
            }

            fn as_mut_slice(&mut self) -> &mut [MaybeUninit<Self::Element>] {
                self.as_mut()
            }
        }

        impl<Element> tiny_vec_ty![Element; $capacity] {
            pub(crate) fn new() -> Self {
                Self {
                    storage: array![MaybeUninit::uninit(); $capacity],
                    count: 0,
                }
            }

            pub(crate) fn push(&mut self, value: Element) {
                assert!(self.count < self.storage.len(), "Capacity is exceeded");
                unsafe {
                    self.storage[self.count].as_mut_ptr().write(value)
                }
                self.count += 1;
            }
        }

        impl<Element> Deref for tiny_vec_ty![Element; $capacity] {
            type Target = [Element];

            fn deref(&self) -> &Self::Target {
                let slice = &self.storage[..self.count];
                unsafe {
                    &*(slice as *const [MaybeUninit<Element>] as *const [Element])
                }
            }
        }

        impl<Element> DerefMut for tiny_vec_ty![Element; $capacity] {
            fn deref_mut(&mut self) -> &mut Self::Target {
                let slice = &mut self.storage[..self.count];
                unsafe {
                    &mut *(slice as *mut [MaybeUninit<Element>] as *mut [Element])
                }
            }
        }

        impl<Element, Index: core::slice::SliceIndex<[Element]>> core::ops::Index<Index> for tiny_vec_ty![Element; $capacity] {
            type Output = <Index as core::slice::SliceIndex<[Element]>>::Output;

            fn index(&self, index: Index) -> &Self::Output {
                core::ops::Index::index(&**self, index)
            }
        }

        impl<Element: Clone> tiny_vec_ty![Element; $capacity] {

            #[allow(dead_code)] // False positive here
            pub(crate) fn extend_from_slice(&mut self, other: &[Element]) {
                for element in other {
                    self.push(element.clone())
                }
            }
        }

        impl<Element: Clone> Clone for tiny_vec_ty![Element; $capacity] {
            fn clone(&self) -> Self {
                let mut result = Self::new();
                for element in &**self {
                    result.push(element.clone());
                }
                result
            }
        }

        impl<Element: Debug> Debug for tiny_vec_ty![Element; $capacity] {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                Debug::fmt(&**self, f)
            }
        }

        impl<Element: PartialEq> PartialEq for tiny_vec_ty![Element; $capacity] {
            fn eq(&self, other: &Self) -> bool {
                PartialEq::eq(&**self, &**other)
            }
        }

        impl<Element: Eq> Eq for tiny_vec_ty![Element; $capacity] {}

        impl<Element: PartialOrd> PartialOrd for tiny_vec_ty![Element; $capacity] {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                PartialOrd::partial_cmp(&**self, &**other)
            }
        }

        impl<Element: Ord> Ord for tiny_vec_ty![Element; $capacity] {
            fn cmp(&self, other: &Self) -> Ordering {
                Ord::cmp(&**self, &**other)
            }
        }

        impl<Element> FromIterator<Element> for tiny_vec_ty![Element; $capacity] {
            fn from_iter<I: IntoIterator<Item = Element>>(iter: I) -> Self {
                let mut v = Self::new();
                for element in iter {
                    v.push(element);
                }
                v
            }
        }
    };
}

__tiny_vec_implementation!(16);
__tiny_vec_implementation!(128);
__tiny_vec_implementation!(4096);

impl<Storage: TinyVecStorage> TinyVec<Storage> {
    pub(crate) fn clear(&mut self) {
        for element in self.storage.as_mut_slice().iter_mut().take(self.count) {
            unsafe { core::ptr::drop_in_place(element.as_mut_ptr()) }
        }
        self.count = 0
    }
}

impl<Storage: TinyVecStorage> Drop for TinyVec<Storage> {
    fn drop(&mut self) {
        self.clear()
    }
}

macro_rules! __tiny_vec_push_elements {
    ($v:ident, ) => ();
    ($v:ident, $head:expr, $($tail:expr,)*) => {
        $v.push($head);
        __tiny_vec_push_elements!($v, $($tail,)*);
    };
}

macro_rules! tiny_vec {
    (capacity: $capacity:literal) => (
        <tiny_vec_ty![_; $capacity]>::new()
    );
    (capacity: $capacity:literal, []) => (
        tiny_vec!(capacity: $capacity)
    );
    (capacity: $capacity:literal, [$($elements:expr),*]) => {{
        let mut v = tiny_vec!(capacity: $capacity);
        __tiny_vec_push_elements!(v, $($elements,)*);
        v
    }};
    (capacity: $capacity:literal, [$($elements:expr,)*]) => (
        tiny_vec![capacity: $capacity, [$($elements),*]]
    )
}

pub struct TinyString<Storage: TinyVecStorage> {
    v: TinyVec<Storage>,
}

macro_rules! tiny_string_ty {
    ($capacity:literal) => {
        $crate::util::TinyString::<[core::mem::MaybeUninit<u8>; $capacity]>
    };
}

macro_rules! __tiny_string_implementation {
    ($capacity:literal) => {
        impl tiny_string_ty![$capacity] {
            pub(crate) fn new() -> Self {
                Self {
                    v: <tiny_vec_ty![u8; $capacity]>::new()
                }
            }

            pub(crate) fn push(&mut self, ch: char) {
                match ch.len_utf8() {
                    1 => self.v.push(ch as u8),
                    _ => self.v.extend_from_slice(ch.encode_utf8(&mut [0; 4]).as_bytes()),
                }
            }
        }
        impl From<&str> for tiny_string_ty![$capacity] {
            fn from(s: &str) -> Self {
                Self {
                    v: s.bytes().collect()
                }
            }
        }

        impl Deref for tiny_string_ty![$capacity] {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                unsafe {
                    core::str::from_utf8_unchecked(&*self.v)
                }
            }
        }

        impl FromIterator<char> for tiny_string_ty![$capacity] {
            fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
                let mut v = Self::new();
                for element in iter {
                    v.push(element);
                }
                v
            }
        }

        impl PartialEq for tiny_string_ty![$capacity] {
            fn eq(&self, other: &tiny_string_ty![$capacity]) -> bool {
                PartialEq::eq(&self[..], &other[..])
            }
            fn ne(&self, other: &tiny_string_ty![$capacity]) -> bool {
                PartialEq::ne(&self[..], &other[..])
            }
        }

        impl PartialEq<str> for tiny_string_ty![$capacity] {
            fn eq(&self, other: &str) -> bool {
                PartialEq::eq(&self[..], &other[..])
            }
            fn ne(&self, other: &str) -> bool {
                PartialEq::ne(&self[..], &other[..])
            }
        }

        impl<'a> PartialEq<&'a str> for tiny_string_ty![$capacity] {
            fn eq(&self, other: & &'a str) -> bool {
                PartialEq::eq(&self[..], &other[..])
            }
            fn ne(&self, other: &&'a str) -> bool {
                PartialEq::ne(&self[..], &other[..])
            }
        }

        impl Debug for tiny_string_ty![$capacity] {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                Debug::fmt(&self[..], f)
            }
        }

        impl core::fmt::Display for tiny_string_ty![$capacity] {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                core::fmt::Display::fmt(&self[..], f)
            }
        }
    };
}

__tiny_string_implementation!(128);

/// В переданной шкале пар `scale` находит первую пару, первый элемент которой строго
/// больше чем `value`, и возвращает второй элемент этой пары. Если такая пара не найдена,
/// возвращает `default`.
pub(crate) fn assess<'a, 'b: 'a, T: PartialOrd, U>(
    scale: &'a [(T, U)],
    value: &T,
    default: &'b U,
) -> &'a U {
    scale
        .iter()
        .find(|&(bound, _)| bound > value)
        .map_or(default, |(_, assessment)| assessment)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_full_capacity() {
        let mut v = tiny_vec!(capacity: 16);
        for i in 0..16 {
            v.push(i + 1);
        }
        assert_eq!(&*v, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    }

    #[test]
    #[should_panic]
    fn test_exceeding_capacity() {
        let mut v = tiny_vec!(capacity: 16);
        for i in 0..=16 {
            v.push(i + 1);
        }
    }

    #[test]
    fn test_clone() {
        let v = tiny_vec!(capacity: 16, [1, 3, 5, 7, 9]);
        let cloned = v.clone();
        assert_eq!(&*cloned, [1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_clear() {
        use core::cell::RefCell;

        let drop_counter = RefCell::new(0usize);

        struct DropCounter<'a> {
            counter: &'a RefCell<usize>,
        }

        impl Drop for DropCounter<'_> {
            fn drop(&mut self) {
                *self.counter.borrow_mut() += 1;
            }
        }

        let vec = tiny_vec!(capacity: 16, [
            DropCounter { counter: &drop_counter },
            DropCounter { counter: &drop_counter },
            DropCounter { counter: &drop_counter },
            DropCounter { counter: &drop_counter },
            DropCounter { counter: &drop_counter },
        ]);

        assert_eq!(*drop_counter.borrow(), 0);

        core::mem::drop(vec);

        assert_eq!(*drop_counter.borrow(), 5);
    }
}
