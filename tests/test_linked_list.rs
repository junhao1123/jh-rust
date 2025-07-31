use std::cell::RefCell;
use std::fmt;

#[derive(Debug, PartialEq)]
struct TestStruct {
    elem: u32,
    name: String,
}

impl fmt::Display for TestStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.elem, self.name)
    }
}


#[cfg(test)]
mod test_singly_linked_list {
    use jh_rust::singly_linked_list::List;
    use crate::TestStruct;

    #[test]
    fn test_list() {
        let mut list: List<TestStruct> = List::new();
        assert_eq!(list.len(), 0);

        list.push(TestStruct {elem:123, name:"name".to_string()});
        assert_eq!(list.len(), 1);
        assert_eq!(list.peek(), Some(&TestStruct {elem:123, name:"name".to_string()}));

        list.push(TestStruct {elem:456, name:"name2".to_string()});
        assert_eq!(list.len(), 2);
        assert_eq!(list.peek(), Some(&TestStruct {elem:456, name:"name2".to_string()}));

        list.peek_mut().map(|x| {*x = TestStruct {elem:789, name:"name3".to_string()}});
        assert_eq!(list.peek(), Some(&TestStruct {elem:789, name:"name3".to_string()}));

        assert_eq!(list.pop(), Some(TestStruct {elem:789, name:"name3".to_string()}));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop(), Some(TestStruct {elem:123, name:"name".to_string()}));
        assert_eq!(list.len(), 0);
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn test_into_iter() {
        let mut list: List<TestStruct> = List::new();
        list.push(TestStruct {elem:123, name:"name".to_string()});
        list.push(TestStruct {elem:456, name:"name2".to_string()});
        list.push(TestStruct {elem:789, name:"name3".to_string()});

        let mut iterator = list.into_iter();
        assert_eq!(iterator.next(), Some(TestStruct {elem:789, name:"name3".to_string()}));
        assert_eq!(iterator.next(), Some(TestStruct {elem:456, name:"name2".to_string()}));
        assert_eq!(iterator.next(), Some(TestStruct {elem:123, name:"name".to_string()}));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_reverse() {
        let mut list: List<TestStruct> = List::new();
        list.push(TestStruct {elem:123, name:"name".to_string()});
        list.push(TestStruct {elem:456, name:"name2".to_string()});
        list.push(TestStruct {elem:789, name:"name3".to_string()});

        list.reverse();
        assert_eq!(list.pop(), Some(TestStruct {elem:123, name:"name".to_string()}));
        assert_eq!(list.pop(), Some(TestStruct {elem:456, name:"name2".to_string()}));
        assert_eq!(list.pop(), Some(TestStruct {elem:789, name:"name3".to_string()}));
    }

    #[test]
    fn test_apply() {
        let mut list: List<TestStruct> = List::new();
        list.push(TestStruct {elem:123, name:"name".to_string()});
        list.push(TestStruct {elem:456, name:"name2".to_string()});
        list.push(TestStruct {elem:789, name:"name3".to_string()});

        let mut new_list = list.apply(|x| x.elem);
        assert_eq!(new_list.pop(), Some(789));
        assert_eq!(new_list.pop(), Some(456));
        assert_eq!(new_list.pop(), Some(123));
    }

    #[test]
    fn test_partial_eq() {
        let mut list1: List<TestStruct> = List::new();
        list1.push(TestStruct {elem:123, name:"name".to_string()});
        list1.push(TestStruct {elem:456, name:"name2".to_string()});

        let mut list2: List<TestStruct> = List::new();
        list2.push(TestStruct {elem:123, name:"name".to_string()});
        list2.push(TestStruct {elem:456, name:"name2".to_string()});

        assert_eq!(list1, list2);
        assert_eq!(list1.len(), list2.len());
        assert_eq!(list1.len(), 2);
        assert_eq!(list1.pop(), list2.pop());
    }

    #[test]
    fn test_display() {
        let mut list: List<TestStruct> = List::new();
        list.push(TestStruct {elem:123, name:"name".to_string()});
        list.push(TestStruct {elem:456, name:"name2".to_string()});
        list.push(TestStruct {elem:789, name:"name3".to_string()});

        assert_eq!(format!("{}", list), "(789, name3)->(456, name2)->(123, name)->end");
    }

    #[test]
    fn test_interleave_even() {
        let mut list1: List<TestStruct> = List::new();
        list1.push(TestStruct {elem:123, name:"name".to_string()});
        list1.push(TestStruct {elem:456, name:"name2".to_string()});

        let mut list2: List<TestStruct> = List::new();
        list2.push(TestStruct {elem:100, name:"name4".to_string()});
        list2.push(TestStruct {elem:200, name:"name5".to_string()});

        list1 = list1.interleave(list2);
        assert_eq!(list1.len(), 4);
        assert_eq!(list1.to_string(), "(123, name)->(100, name4)->(456, name2)->(200, name5)->end");
    }
}

#[test]
fn test_refcell_drop() {
    let x = RefCell::new(5);

    let mut mutable_borrow = x.borrow_mut();

    *mutable_borrow = 6;

    assert_eq!(*mutable_borrow, 6);
    // assert_eq!(x, RefCell::new(6)); if uncommented, runs into "already borrowed" error

    drop(mutable_borrow);

    assert_eq!(x, RefCell::new(6));
}
