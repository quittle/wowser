use std::collections::VecDeque;

pub trait VecDequeExt<T: PartialEq> {
    fn get_index(&self, val: &T) -> Option<usize>;
}

impl<T: PartialEq> VecDequeExt<T> for VecDeque<T> {
    fn get_index(&self, val: &T) -> Option<usize> {
        let (a, b) = self.as_slices();

        let mut i = 0;
        for v in a {
            if v == val {
                return Some(i);
            }
            i += 1;
        }

        for v in b {
            if v == val {
                return Some(i);
            }
            i += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_index {
        use super::*;
        use std::collections::VecDeque;

        #[test]
        fn test_empty() {
            assert_eq!(None, VecDeque::new().get_index(&'a'));
        }

        #[test]
        fn test_small() {
            let mut queue = VecDeque::new();
            queue.push_front('c');
            queue.push_front('b');
            queue.push_front('a');
            assert_eq!(Some(0), queue.get_index(&'a'));
            assert_eq!(Some(1), queue.get_index(&'b'));
            assert_eq!(Some(2), queue.get_index(&'c'));
            assert_eq!(None, queue.get_index(&'d'));
        }

        #[test]
        fn test_complex() {
            let mut queue = VecDeque::new();
            queue.push_front('b');
            queue.push_front('x');
            queue.push_front('a');
            queue.remove(1);
            queue.push_back('c');
            let (a, b) = queue.as_slices();
            assert!(!a.is_empty());
            assert!(!b.is_empty());
            assert_eq!(Some(0), queue.get_index(&'a'));
            assert_eq!(Some(1), queue.get_index(&'b'));
            assert_eq!(Some(2), queue.get_index(&'c'));
        }
    }
}
