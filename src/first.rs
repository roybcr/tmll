use std::mem;

pub struct List {
        head: Link,
}

enum Link {
        Empty,
        More(Box<Node>),
}

struct Node {
        elem: i32,
        next: Link,
}

impl List {
        pub fn new() -> Self {
                List { head: Link::Empty }
        }

        // Implement a push method for the List struct, which takes a mutable reference
        // to self and an i32 element.
        pub fn push(&mut self, elem: i32) {
                // Use the mem::replace function to replace the current head with an empty
                // node and store the old head as the next node in the newly created node.
                let node = Box::new(Node {
                        elem,
                        next: mem::replace(&mut self.head, Link::Empty),
                });

                self.head = Link::More(node);
        }

        pub fn pop(&mut self) -> Option<i32> {
                // temporarily placing Link::Empty as our head, while matching the repalced
                // head's value
                match mem::replace(&mut self.head, Link::Empty) {
                        Link::Empty => None,
                        Link::More(node) => {
                                self.head = node.next;
                                Some(node.elem)
                        }
                }
        }
}

impl Drop for List {
        fn drop(&mut self) {
                let mut cur_link = mem::replace(&mut self.head, Link::Empty);
                while let Link::More(mut boxed_node) = cur_link {
                        cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
                }
        }
}

#[cfg(test)]
mod tests {
        use super::List;

        #[test]
        fn basics() {
                // creating a new List
                let mut list = List::new();

                // check empty list behaves right
                assert_eq!(list.pop(), None);

                // populate list
                list.push(1);
                list.push(2);
                list.push(3);

                // check normal removal
                assert_eq!(list.pop(), Some(3));
                assert_eq!(list.pop(), Some(2));

                // push some more just to make sure
                list.push(4);
                list.push(5);

                // check exhaustion
                assert_eq!(list.pop(), Some(5));
                assert_eq!(list.pop(), Some(4));
                assert_eq!(list.pop(), Some(1));
                assert_eq!(list.pop(), None);
        }
}
