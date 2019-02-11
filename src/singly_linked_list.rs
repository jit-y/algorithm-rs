use std::fmt::{Display, Formatter};
use std::ptr::NonNull;

struct SinglyLinkedListNode<T: Display> {
    value: T,
    next: Option<NonNull<SinglyLinkedListNode<T>>>,
}

impl<T> Display for SinglyLinkedListNode<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> SinglyLinkedListNode<T>
where
    T: Display,
{
    pub fn new(value: T) -> Self {
        Self {
            value: value,
            next: None,
        }
    }
}

struct SinglyLinkedList<T: Display> {
    head: Option<NonNull<SinglyLinkedListNode<T>>>,
    tail: Option<NonNull<SinglyLinkedListNode<T>>>,
}
impl<T> SinglyLinkedList<T>
where
    T: Display,
{
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, val: T) {
        let mut node = SinglyLinkedListNode::new(val);
        node.next = self.head;

        let node = Some(Box::into_raw_non_null(Box::new(node)));
        self.head = node;

        match self.tail {
            Some(_) => {}
            None => {
                self.tail = node;
            }
        }
    }

    pub fn pop(&mut self) -> Option<&T> {
        unsafe {
            let head = match self.head {
                Some(node) => &*node.as_ptr(),
                None => return None,
            };

            self.head = head.next;

            match self.head {
                Some(_) => {}
                None => {
                    self.tail = None;
                }
            }

            Some(&head.value)
        }
    }

    pub fn add(&mut self, val: T) -> bool {
        let node = Box::into_raw_non_null(Box::new(SinglyLinkedListNode::new(val)));

        unsafe {
            match self.head {
                Some(_head) => {
                    match self.tail {
                        Some(tail) => {
                            let tail = &mut *tail.as_ptr();
                            tail.next = Some(node);
                        }
                        None => return false,
                    };
                }
                None => {
                    self.head = Some(node);
                }
            }
        }

        self.tail = Some(node);

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::singly_linked_list::SinglyLinkedList;

    #[test]
    fn test_push() {
        let mut sll = SinglyLinkedList::new();
        sll.push("foo");

        unsafe {
            // dereference & move out
            let node = &*sll.head.expect("error").as_ptr();
            assert_eq!(node.value, "foo");
        }
    }

    #[test]
    fn test_pop() {
        let mut sll = SinglyLinkedList::new();
        sll.push("foo");

        let val = sll.pop();

        assert_eq!(val, Some(&"foo"));
        assert_eq!(sll.pop(), None);
    }

    #[test]
    fn test_add() {
        let mut sll = SinglyLinkedList::new();
        assert_eq!(sll.add("foo"), true);

        unsafe {
            let node = &*sll.tail.expect("error").as_ptr();
            assert_eq!(node.value, "foo");
        }
    }
}
