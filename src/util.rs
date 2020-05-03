#![macro_use]

pub(crate) struct StackAllocatedVector<T> {
    pub(crate) storage: T,
    pub(crate) count: usize,
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
