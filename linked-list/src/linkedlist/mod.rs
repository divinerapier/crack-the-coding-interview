pub struct LinkedList<T> {
    pub head: Option<std::ptr::NonNull<Node<T>>>,
    pub tail: Option<std::ptr::NonNull<Node<T>>>,
    pub length: usize,
}

pub struct Node<T> {
    pub prev: Option<std::ptr::NonNull<Node<T>>>,
    pub next: Option<std::ptr::NonNull<Node<T>>>,
    pub data: T,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node {
            prev: None,
            next: None,
            data,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn push_front(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = self.head;
            node.prev = None;

            let node = Some(Box::into_raw_non_null(node));

            match self.head {
                None => self.tail = node,
                Some(mut head) => head.as_mut().prev = node,
            }
            self.head = node;
            self.length += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<Box<Node<T>>> {
        unsafe {
            match self.head {
                None => None,
                Some(head) => {
                    let node = Box::from_raw(head.as_ptr());
                    self.head = node.next;
                    match self.head {
                        None => self.tail = None,
                        Some(mut new_head) => {
                            new_head.as_mut().prev = None;
                        }
                    }
                    self.length -= 1;
                    Some(node)
                }
            }
        }
    }

    pub fn push_back(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.prev = self.tail;
            node.next = None;

            // #![feature(box_into_raw_non_null)]
            let node = Some(Box::into_raw_non_null(node));

            match self.tail {
                None => self.head = node,
                Some(mut tail) => tail.as_mut().next = node,
            }

            self.tail = node;
            self.length += 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<Box<Node<T>>> {
        unsafe {
            match self.tail {
                None => None,
                Some(tail) => {
                    let node: Box<Node<T>> = Box::from_raw(tail.as_ptr());
                    self.tail = node.prev;

                    match self.tail {
                        None => self.head = None,
                        Some(mut tail) => {
                            tail.as_mut().next = None;
                        }
                    }

                    self.length -= 1;
                    Some(node)
                }
            }
        }
    }
}
