use std::{slice::Iter as SliceIter, iter::Chain};

pub type Iter<'a, T> = Chain<SliceIter<'a, T>, SliceIter<'a, T>>;

pub struct CircularQueue<T> {
    data: Vec<T>,
    index: usize,
}

impl<T> CircularQueue<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {data: Vec::with_capacity(capacity), index: 0}
    }

    pub fn push(&mut self, item: T) {
        if self.data.len() == self.data.capacity() {
            self.data[self.index] = item;
        } else {
            self.data.push(item)
        }
        self.index = if self.index == self.data.capacity() - 1 {
            0
        } else {
            self.index + 1
        };
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_full(&self) -> bool {
        self.data.len() == self.data.capacity()
    }

    pub fn iter(&self) -> Iter<T> {
        let (tail, head) = self.data.split_at(self.index);
        head.iter().chain(tail.iter())
    }
}

impl<T: Clone> CircularQueue<T> {
    fn owned_vec(&self) -> Vec<T> {
        let mut ret = vec![];
        for el in self.iter() {
            ret.push(el.clone());
        }
        ret
    }
}


#[test]
fn insert_test() {
    let mut q = CircularQueue::with_capacity(3);
    q.push(1);
    assert!(!q.is_full());
    q.push(2);
    assert!(!q.is_full());
    q.push(3);
    assert!(q.is_full());
    q.push(4);
    assert!(q.is_full());

    assert_eq!(q.len(), 3);
    assert_eq!(&q.owned_vec(), &[2, 3, 4]);
    q.push(5);
    assert!(q.is_full());
    assert_eq!(&q.owned_vec(), &[3, 4, 5]);
}
