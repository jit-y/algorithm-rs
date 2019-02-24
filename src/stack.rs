use super::singly_linked_list::SinglyLinkedList;
use std::fmt::Display;

pub struct Stack<T: Display + Copy + Clone + PartialEq> {
    pub linked_list: SinglyLinkedList<T>,
}

impl<T> Stack<T>
where
    T: Display + Copy + Clone + PartialEq,
{
    pub fn new() -> Self {
        Stack {
            linked_list: SinglyLinkedList::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        if let None = self.linked_list.head {
            return true;
        }

        false
    }

    pub fn peek(&self) -> Option<T> {
        match self.linked_list.head {
            Some(node) => unsafe { Some((&*node.as_ptr()).value.clone()) },
            None => None,
        }
    }

    pub fn push(&mut self, value: T) {
        self.linked_list.prepend(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.linked_list.delete_head()
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::Stack;

    #[test]
    fn test_push() {
        let mut s = Stack::new();

        s.push("foo");
        assert_eq!(Some("foo"), s.peek());
        s.push("bar");
        assert_eq!(Some("bar"), s.peek());
    }

    #[test]
    fn test_pop() {
        let mut s = Stack::new();

        s.push("foo");
        s.push("bar");
        assert_eq!(Some("bar"), s.pop());
        assert_eq!(false, s.is_empty());
        assert_eq!(Some("foo"), s.pop());
        assert_eq!(true, s.is_empty());
        assert_eq!(None, s.pop());
    }
}
