use std::{
    array::IntoIter as ArrayIntoIter,
    iter::{
        Copied,
        Once,
        once,
    },
    slice::Iter as SliceIter,
    vec::IntoIter as VecIntoIter,
};

pub trait SubtreeIndex {
    type Iter: Iterator<Item = usize>;

    fn get_index(self) -> Self::Iter;
}

impl SubtreeIndex for usize {
    type Iter = Once<Self>;

    fn get_index(self) -> Self::Iter {
        once(self)
    }
}

impl<const N: usize> SubtreeIndex for [usize; N] {
    type Iter = ArrayIntoIter<usize, N>;

    fn get_index(self) -> Self::Iter {
        self.into_iter()
    }
}

impl<'slice> SubtreeIndex for &'slice [usize] {
    type Iter = Copied<SliceIter<'slice, usize>>;

    fn get_index(self) -> Self::Iter {
        self.iter().copied()
    }
}

impl SubtreeIndex for Box<[usize]> {
    type Iter = VecIntoIter<usize>;

    fn get_index(self) -> Self::Iter {
        self.into_iter()
    }
}

impl SubtreeIndex for Vec<usize> {
    type Iter = VecIntoIter<usize>;

    fn get_index(self) -> Self::Iter {
        self.into_iter()
    }
}
