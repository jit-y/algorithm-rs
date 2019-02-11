use std::fmt::{Display, Formatter};
use std::ptr::NonNull;

#[derive(PartialEq)]
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

    pub fn prepend(&mut self, val: T) {
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

    pub fn delete_head(&mut self) -> Option<&T> {
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

    pub fn append(&mut self, val: T) -> bool {
        let new_node = Box::into_raw_non_null(Box::new(SinglyLinkedListNode::new(val)));

        unsafe {
            match self.head {
                Some(_head) => {
                    match self.tail {
                        Some(tail) => {
                            (&mut *tail.as_ptr()).next = Some(new_node);
                        }
                        None => return false,
                    };
                }
                None => {
                    self.head = Some(new_node);
                }
            }
        }

        self.tail = Some(new_node);

        true
    }

    pub fn delete_tail(&mut self) -> Option<&T> {
        unsafe {
            let tail = self.tail?;
            let mut head = self.head?;

            if tail == head {
                self.head = None;
                self.tail = None;

                return Some(&(&*tail.as_ptr()).value);
            }

            loop {
                let current = (&*head.as_ptr()).next;
                match current {
                    None => {
                        break;
                    }
                    Some(current) => match (&*current.as_ptr()).next {
                        Some(_) => {
                            head = current;
                        }
                        None => {
                            (&mut *head.as_ptr()).next = None;
                        }
                    },
                }
            }

            self.tail = Some(head);

            Some(&(&*tail.as_ptr()).value)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::singly_linked_list::SinglyLinkedList;

    #[test]
    fn test_prepend() {
        let mut sll = SinglyLinkedList::new();
        sll.prepend("foo");

        unsafe {
            // dereference & move out
            let node = &*sll.head.expect("error").as_ptr();
            assert_eq!(node.value, "foo");
        }
    }

    #[test]
    fn test_delete_head() {
        let mut sll = SinglyLinkedList::new();
        sll.prepend("foo");

        let val = sll.delete_head();

        assert_eq!(val, Some(&"foo"));
        assert_eq!(sll.delete_head(), None);
    }

    #[test]
    fn test_append() {
        let mut sll = SinglyLinkedList::new();
        assert_eq!(sll.append("foo"), true);
        assert_eq!(sll.append("bar"), true);

        unsafe {
            let node = &*sll.tail.expect("error").as_ptr();
            assert_eq!(node.value, "bar");
        }
    }

    #[test]
    fn test_delete_tail() {
        let mut sll = SinglyLinkedList::new();
        sll.append("foo");
        sll.append("bar");
        sll.append("baz");

        assert_eq!(sll.delete_tail(), Some(&"baz"));
        unsafe {
            assert_eq!((&*sll.tail.expect("error").as_ptr()).value, "bar");
        }
    }
}
