#[derive(Debug)]
pub struct FixedVecDeque<T, const N: usize> {
    buffer: [Option<T>; N],
    front: usize,
    rear: usize,
    len: usize,
}

impl<T, const N: usize> FixedVecDeque<T, N>
where
    T: Copy,
{
    pub fn new() -> Self {
        FixedVecDeque {
            buffer: [None; N],
            front: 0,
            rear: 0,
            len: 0,
        }
    }

    pub fn push_front(&mut self, item: T) {
        if self.len == N {
            // The deque is full
            // You can choose to handle this case in any way you want, e.g., overwrite the oldest element
            self.pop_back();
        }

        self.front = (self.front + N - 1) % N;
        self.buffer[self.front] = Some(item);
        self.len += 1;
    }

    pub fn push_back(&mut self, item: T) {
        if self.len == N {
            // The deque is full
            // You can choose to handle this case in any way you want, e.g., overwrite the oldest element
            self.pop_front();
        }

        self.buffer[self.rear] = Some(item);
        self.rear = (self.rear + 1) % N;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let front_item = self.buffer[self.front].take();
            self.front = (self.front + 1) % N;
            self.len -= 1;
            front_item
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let rear = (self.rear + N - 1) % N;
            let rear_item = self.buffer[rear].take();
            self.rear = rear;
            self.len -= 1;
            rear_item
        }
    }

    pub fn contains(&self, item: &T) -> bool
    where
        T: PartialEq,
    {
        self.buffer.iter().any(|x| x.as_ref() == Some(item))
    }

    pub fn front(&self) -> Option<&T> {
        self.buffer[self.front].as_ref()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> FixedVecDequeIter<'_, T, N> {
        FixedVecDequeIter {
            deque: self,
            index: 0,
        }
    }
}

impl<T, const N: usize> Default for FixedVecDeque<T, N>
where
    T: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct FixedVecDequeIter<'a, T, const N: usize> {
    deque: &'a FixedVecDeque<T, N>,
    index: usize,
}

impl<'a, T, const N: usize> Iterator for FixedVecDequeIter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.deque.len {
            let item = &self.deque.buffer[(self.deque.front + self.index) % N];
            self.index += 1;
            Some(item.as_ref().unwrap())
        } else {
            None
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_push_front() {
        let mut deque = FixedVecDeque::<i32, 3>::new();
        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);
        assert_eq!(deque.front(), Some(&3));
        assert_eq!(deque.len(), 3);
    }

    #[test]
    fn test_push_back() {
        let mut deque = FixedVecDeque::<i32, 3>::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.len(), 3);
    }

    #[test]
    fn test_pop_front() {
        let mut deque = FixedVecDeque::<i32, 3>::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
    }

    #[test]
    fn test_pop_back() {
        let mut deque = FixedVecDeque::<i32, 3>::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_back(), Some(2));
    }

    #[test]
    fn test_contains() {
        let mut deque = FixedVecDeque::<i32, 3>::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        assert!(deque.contains(&1));
        assert!(deque.contains(&2));
        assert!(deque.contains(&3));
        assert!(!deque.contains(&4));
    }

    #[test]
    fn test_iter() {
        let mut deque = FixedVecDeque::<i32, 3>::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        let mut iter = deque.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
    }
}
