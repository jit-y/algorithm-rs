use super::singly_linked_list::SinglyLinkedList;
use std::fmt::Display;

pub struct Queue<T: Display> {
    pub linked_list: SinglyLinkedList<T>,
}

impl<T> Queue<T>
where
    T: Display,
{
    fn new() -> Self {
        Queue {
            linked_list: SinglyLinkedList::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        if let None = self.linked_list.head {
            return true;
        }

        false
    }

    pub fn enqueue(&mut self, value: T) -> Option<&T> {
        self.linked_list.append(value)
    }

    pub fn dequeue(&mut self) -> Option<&T> {
        self.linked_list.delete_head()
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::Queue;

    #[test]
    fn test_is_empty() {
        let mut q = Queue::new();
        assert_eq!(q.is_empty(), true);
        q.enqueue("foo");
        assert_eq!(q.is_empty(), false);
    }

    #[test]
    fn test_enqueue() {
        let mut q = Queue::new();
        assert_eq!(q.enqueue("foo"), Some(&"foo"));
        assert_eq!(q.enqueue("bar"), Some(&"bar"));
    }

    #[test]
    fn test_dequeue() {
        let mut q = Queue::new();
        q.enqueue("foo");
        q.enqueue("bar");
        assert_eq!(q.dequeue(), Some(&"foo"));
        assert_eq!(q.dequeue(), Some(&"bar"));
        assert_eq!(q.dequeue(), None);
    }
}
