pub struct List<T> {
        head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
        elem: T,
        next: Link<T>,
}

impl<T> List<T> {
        pub fn new() -> Self {
                List { head: None }
        }

        // Implement a push method for the List struct, which takes a mutable reference
        // to self and an i32 element.
        pub fn push(&mut self, elem: T) {
                // Use the take() function of Option<T> instead of writing mem::replace(...),
                // to replace the current head with an empty node and store the old head
                // as the next node in the newly created node.
                let node = Box::new(Node {
                        elem,
                        next: self.head.take(),
                });

                self.head = Some(node);
        }

        pub fn push_many(&mut self, elements: Vec<T>) {
                for elem in elements.into_iter() {
                        self.push(elem);
                }
        }

        pub fn pop(&mut self) -> Option<T> {
                // temporarily placing None as our head, while matching the repalced
                // head's value
                self.head.take().map(|node| {
                        // match option { None => None, Some(x) => Some(y) } is such an incredibly
                        // common idiom that it was called map.
                        self.head = node.next;
                        node.elem
                })
        }

        pub fn peek(&self) -> Option<&T> {
                // return a reference to the element in the head of the list
                self.head.as_ref().map(|node| &node.elem)
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
                self.head.as_mut().map(|node| &mut node.elem)
        }

        pub fn iter<'a>(&'a self) -> Iter<'a, T> {
                Iter {
                        next: self.head.as_deref().map(|node| &*node),
                }
        }
        pub fn into_iter(self) -> IntoIter<T> {
                IntoIter(self)
        }

        pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
                IterMut {
                        next: self.head.as_deref_mut(),
                }
        }
}

// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
                self.0.pop()
        }
}

pub struct Iter<'a, T> {
        next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
                self.next.map(|node| {
                        self.next = node.next.as_deref();
                        &node.elem
                })
        }
}

pub struct IterMut<'a, T> {
        next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;
        fn next(&mut self) -> Option<Self::Item> {
                self.next.take().map(|node| {
                        self.next = node.next.as_deref_mut();
                        &mut node.elem
                })
        }
}

impl<T> Drop for List<T> {
        fn drop(&mut self) {
                let mut cur_link = self.head.take();
                while let Some(mut boxed_node) = cur_link {
                        cur_link = boxed_node.next.take();
                }
        }
}

#[cfg(test)]
mod tests {
        use super::List;

        #[test]
        fn iter_mut() {
                let mut list = List::new();
                list.push_many(vec![1, 2, 3, 4]);
                let mut iter = list.iter_mut();

                assert_eq!(iter.next(), Some(&mut 4));
                assert_eq!(iter.next(), Some(&mut 3));
                assert_eq!(iter.next(), Some(&mut 2));
                assert_eq!(iter.next(), Some(&mut 1));
                assert_eq!(iter.next(), None);
        }

        #[test]
        fn iter() {
                let mut list = List::new();
                list.push_many(vec![1, 2, 3, 4]);
                let mut iter = list.iter();

                assert_eq!(iter.next(), Some(&4));
                assert_eq!(iter.next(), Some(&3));
                assert_eq!(iter.next(), Some(&2));
                assert_eq!(iter.next(), Some(&1));
                assert_eq!(iter.next(), None);
        }

        #[test]
        fn into_iter() {
                let mut list = List::new();
                list.push_many(vec![1, 2, 3, 4]);
                let mut iter = list.into_iter();

                assert_eq!(iter.next(), Some(4));
                assert_eq!(iter.next(), Some(3));
                assert_eq!(iter.next(), Some(2));
                assert_eq!(iter.next(), Some(1));
                assert_eq!(iter.next(), None);
        }

        #[test]
        fn peek() {
                let mut list = List::new();

                assert_eq!(list.peek(), None);
                assert_eq!(list.peek_mut(), None);

                list.push_many(vec![1, 2, 3, 4]);

                assert_eq!(list.peek(), Some(&4));
                assert_eq!(list.peek_mut(), Some(&mut 4));

                list.peek_mut().map(|elem| *elem = 38);

                assert_eq!(list.peek(), Some(&38));
                assert_eq!(list.peek_mut(), Some(&mut 38));
        }

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
