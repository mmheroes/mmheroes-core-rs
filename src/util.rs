#![macro_use]

pub(crate) struct StackAllocatedVector<T> {
    storage: T,
    count: usize,
}

macro_rules! __stack_allocated_vector_implementation {
    ($capacity:literal) => {
        impl<Element: Default + Copy> StackAllocatedVector<[Element; $capacity]> {
            pub(crate) fn new() -> Self {
                Self {
                    storage: [Element::default(); $capacity],
                    count: 0,
                }
            }

            pub(crate) fn push(&mut self, value: Element) {
                assert!(self.count < $capacity, "Capacity is exceeded");
                self.storage[self.count] = value;
                self.count += 1;
            }
        }

        impl<Element> core::ops::Deref for StackAllocatedVector<[Element; $capacity]> {
            type Target = [Element];

            fn deref(&self) -> &Self::Target {
                &self.storage[..self.count]
            }
        }
    };
}

__stack_allocated_vector_implementation!(12);

macro_rules! stack_allocated_vec {
    ($ty:ty; $count:literal) => {
        StackAllocatedVector::<[$ty; $count]>::new()
    };
}

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
