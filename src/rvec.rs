#![allow(dead_code)]

#[flux::opaque]
#[flux::refined_by(len: int)]
pub struct RVec<T> {
    inner: Vec<T>,
}

impl<T> RVec<T> {
    #[flux::assume]
    #[flux::sig(fn() -> RVec<T>[0])]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    #[flux::assume]
    #[flux::sig(
    fn(self: &strg RVec<T>[@n], T) -> () ensures self: RVec<T>[n+1])]
    pub fn push(&mut self, item: T) {
        self.inner.push(item);
    }

    #[flux::assume]
    #[flux::sig(fn(&RVec<T>[@n]) -> usize[n])]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[flux::assume]
    #[flux::sig(fn(&RVec<T>[@n]) -> bool[n == 0])]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[flux::assume]
    #[flux::sig(fn(&RVec<T>[@n], i: usize{0 <= i && i < n}) -> &T)]
    pub fn get(&self, i: usize) -> &T {
        &self.inner[i]
    }

    #[flux::assume]
    #[flux::sig(fn(&mut RVec<T>[@n], i: usize{ 0 <= i && i < n}) -> &mut T)]
    pub fn get_mut(&mut self, i: usize) -> &mut T {
        &mut self.inner[i]
    }

    #[flux::assume]
    #[flux::sig(fn(self: &strg RVec<T>[@n]) -> T requires n > 0 ensures self: RVec<T>[n-1])]
    pub fn pop(&mut self) -> T {
        self.inner.pop().unwrap()
    }

    #[flux::assume]
    #[flux::sig(fn(&mut RVec<T>[@n], a: usize{0 <= a && a < n}, b: usize{0 <= b && b < n}) -> ())]
    pub fn swap(&mut self, a: usize, b: usize) {
        self.inner.swap(a, b);
    }

    #[flux::assume]
    #[flux::sig(fn(T, n: usize) -> RVec<T>[n])]
    pub fn from_elem_n(elem: T, n: usize) -> Self
    where
        T: Copy,
    {
        let mut vec = Self::new();
        let mut i = 0;
        while i < n {
            vec.push(elem);
            i += 1;
        }
        vec
    }
}
