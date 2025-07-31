use std::fmt;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn _reverse(init: Option<Box<Node<T>>>, mut head: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        let mut prev = init;
        
        while let Some(mut curr) = head {
            head = curr.next.take();
            curr.next = prev;
            prev = Some(curr);
        }

        prev
    }

    pub fn reverse(head: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        Self::_reverse(None, head)
    }

    fn _interleave(curr: Option<Box<Node<T>>>, first: Option<Box<Node<T>>>, second: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        let r = match (first, second) {
            (None, None) => None,
            (Some(f), None) => Self::_reverse(curr, Some(f)),
            (None, Some(f)) => Self::_reverse(curr, Some(f)),
            (Some(mut f), Some(s)) => {
                let f_next = f.next.take();
                f.next = curr;
                Self::_interleave(Some(f), Some(s), f_next)
            }
        };

        Self::reverse(r)
    }

    pub fn interleave(first: Option<Box<Node<T>>>, second: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        Self::_interleave(None, first, second)
    }
}

fn _fmt<T: std::fmt::Display>(node: &Option<Box<Node<T>>>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match node {
        None => write!(f, "end"),
        Some(node) => {
            write!(f, "{}->", node.elem)?;
            _fmt(&node.next, f)
        }
    }
}

impl<T: std::fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}->", self.elem)?;
        _fmt(&self.next, f)
    }
}

fn _eq<T: PartialEq>(curr: &Option<Box<Node<T>>>, other: &Option<Box<Node<T>>>) -> bool {
    match (curr, other) {
        (None, None) => true,
        (Some(_), None) => false,
        (None, Some(_)) => false,
        (Some(n1), Some(n2)) => {
            if n1.elem == n2.elem {
                return _eq(&n1.next, &n2.next);
            }
            return false;
        }
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.elem == other.elem {
            return _eq(&self.next, &other.next);
        }
        return false;
    }
}

#[derive(Debug)]
pub struct List<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.size += 1;
        self.head = Some(new_node);
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }

    pub fn reverse(&mut self) {
        match self.head.take() {
            None => {},
            Some(node) => {
                self.head = Node::reverse(Some(node));
            }
        }
    }

    pub fn apply<U>(&self, mapper_func: fn(&T) -> U) -> List<U> {
        let mut new_list: List<U> = List::new();
        let mut curr = &self.head;

        while let Some(node) = curr {
            new_list.push(mapper_func(&node.elem));
            curr = &node.next;
        }

        new_list.reverse();
        new_list
    }

    pub fn interleave(mut self, mut other: Self) -> Self {
        let (n, m) = (self.size % 2 == 1, other.size % 2 == 1);
        match (n, m) {
            (true, true) | (false, false) => {
                self.head = Node::interleave(other.head.take(), self.head.take());
            },
            _ => {
                self.head = Node::interleave(self.head.take(), other.head.take());
            }
        }
        self.size += other.size;
        self
    }
}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.head == other.head;
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T: std::fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        _fmt(&self.head, f)
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
