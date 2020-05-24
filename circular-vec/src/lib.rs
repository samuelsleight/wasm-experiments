#[derive(Default)]
pub struct CircularVec<T> {
    vec: Vec<T>,
    offset: usize
}

pub struct Iter<'a, T> {
    vec: &'a CircularVec<T>,
    index: usize
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vec.len() {
            None
        } else {
            self.index += 1;
            Some(&self.vec[self.index - 1])
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.vec.len();
        (len, Some(len))
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

impl<T> CircularVec<T> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            offset: 0
        }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            vec: self,
            index: 0
        }
    }

    pub fn push(&mut self, value: T) {
        if self.offset == 0 {
            self.vec.push(value);
        } else {
            self.vec.insert(self.offset, value);
            self.rotate_left(1);
        }
    }

    pub fn rotate_left(&mut self, offset: usize) {
        self.offset = (self.offset + offset) % self.vec.len();
    }

    pub fn rotate_right(&mut self, offset: usize) {
        let len = self.vec.len();
        self.rotate_left(len - (offset % len));
    }
}

impl<T> std::ops::Index<usize> for CircularVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[(self.offset + index) % self.vec.len()]
    }
}

impl<T> std::ops::IndexMut<usize> for CircularVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index = (self.offset + index) % self.vec.len();
        &mut self.vec[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut vec = CircularVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);
        assert_eq!(vec.vec, vec![1, 2, 3, 4, 5]);

        assert_eq!(vec[0], 1);

        vec.rotate_left(2);
        assert_eq!(vec[0], 3);

        vec.rotate_left(6);
        assert_eq!(vec[0], 4);

        vec.rotate_right(1);
        assert_eq!(vec[0], 3);

        vec.rotate_right(7);
        assert_eq!(vec[0], 1);

        vec.rotate_left(3);
        assert_eq!(vec[0], 4);

        vec.push(6);
        vec.push(7);
        vec.push(8);
        vec.push(9);
        assert_eq!(vec.vec, vec![1, 2, 3, 6, 7, 8, 9, 4, 5]);
        assert_eq!(vec[0], 4);

        assert_eq!(vec.iter().cloned().collect::<Vec<_>>(), vec![4, 5, 1, 2, 3, 6, 7, 8, 9])
    }
}
