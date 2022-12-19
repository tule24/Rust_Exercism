use std::{
    fmt::{self, Display, Formatter},
    ptr::NonNull,
};

pub struct List<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}
struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    val: T,
}

impl<T> Node<T> {
    fn new(t: T) -> Self {
        Node {
            next: None,
            prev: None,
            val: t,
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_head(&mut self, ele: T) {
        // update for Node
        let mut node = Box::new(Node::new(ele)); // tạo con trỏ ở vùng nhớ heap và đặt node tại chỗ đó
        node.next = self.head;
        node.prev = None;

        // Update for list
        // Tạo con trỏ NonNull từ node ở trên
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });

        match self.head {
            // check head hiện tại của list
            None => self.tail = node_ptr, // nếu none nghĩa là list này mới được khởi tạo nên node đó làm cả head và tail
            Some(head_ptr) => unsafe {
                // nếu head đã có thì update dữ liệu của node tại head là prev của nó sẽ nối với node mới tạo
                (*head_ptr.as_ptr()).prev = node_ptr
            },
        }

        // gán cái đầu list hiện tại là con trỏ NonNull được tạo ở trên và tăng len list
        self.head = node_ptr;
        self.len += 1;
    }

    pub fn push_tail(&mut self, ele: T) {
        let mut node = Box::new(Node::new(ele));
        node.next = None;
        node.prev = self.tail;

        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });

        match self.tail {
            None => self.head = node_ptr,
            Some(tail_prt) => unsafe { (*tail_prt.as_ptr()).next = node_ptr },
        }

        self.tail = node_ptr;
        self.len += 1;
    }

    pub fn insert_at_index(&mut self, index: usize, ele: T) {
        if self.len < index {
            panic!("Index out of bounds");
        }

        if index == 0 || self.head.is_none() {
            // nếu index = 0 hoặc head là none(chưa khởi tạo list) thì insert này sẽ như push_head
            self.push_head(ele);
            return;
        }

        if self.len == index {
            // nếu index = len nghĩa là nó gắn vào đuôi sử dụng push tail
            self.push_tail(ele);
            return;
        }

        /* Gắn vào giữa thì mình sẽ làm các bước: giả sử list 0,1,2,3,4 và mình muốn chèn index = 2
        - Tạo node mới gọi là node(new)
        - Lấy node tại vị trí cần gắn => 2
        - Gán prev của node(new) là 1, next của node(new) là 2
        - Thay đổi next của node(1) là new và prev của node(2) là new
         => list sau thay đổi 0,1,new,2,3,4
        */
        if let Some(mut node_i) = self.head {
            for _ in 0..index {
                // lấy giá trị node tại vị trí cần gắn, do linked_list ko truy xuất qua index được nên phải tìm kiếm tuần tự bằng for
                unsafe {
                    match (*node_i.as_ptr()).next {
                        // lặp từ 0 tới vị trí trước index
                        None => panic!("Index out of bounds"),
                        Some(next_prt) => node_i = next_prt, // gán node_i là node_next => hết vòng lặp node_i sẽ là node tại index => node(index)
                    }
                }
            }

            let mut node = Box::new(Node::new(ele)); // tạo node mới
            unsafe {
                node.prev = (*node_i.as_ptr()).prev; // gán prev của node mới là prev tại node(index)
                node.next = Some(node_i); // gán next của node mới là node(index)

                // Thay đổi prev và next của 2 node 2 bên node mới
                if let Some(p) = (*node_i.as_ptr()).prev {
                    // lấy giá trị prev của node(index)
                    let node_prt = Some(NonNull::new_unchecked(Box::into_raw(node))); // tạo NonNull từ node mới
                    println!("{:?}", (*p.as_ptr()).next);
                    (*p.as_ptr()).next = node_prt; // update lại next nó bây giờ là node mới không phải node(index) cũ
                    (*node_i.as_ptr()).prev = node_prt; // update lại prev của node(index) là node mới
                    self.len += 1; // tăng len lên 1
                }
            }
        }
    }

    pub fn delete_head(&mut self) -> Option<T> {
        // Safety: head_ptr points to a leaked boxed node managed by this list
        // We reassign pointers that pointed to the head node
        // Lấy head ra update cái next là head, prev của cái next là None
        self.head.map(|head_ptr| unsafe {
            let old_head = Box::from_raw(head_ptr.as_ptr());
            match old_head.next {
                Some(mut next_ptr) => next_ptr.as_mut().prev = None,
                None => self.tail = None,
            }
            self.head = old_head.next;
            self.len -= 1;
            old_head.val
        })
    }

    pub fn delete_tail(&mut self) -> Option<T> {
        self.tail.map(|tail_ptr| unsafe {
            let old_tail = Box::from_raw(tail_ptr.as_ptr());
            match old_tail.prev {
                Some(mut pre_ptr) => pre_ptr.as_mut().next = None,
                None => self.head = None,
            }
            self.tail = old_tail.next;
            self.len -= 1;
            old_tail.val
        })
    }

    pub fn delete_at_index(&mut self, index: usize) -> Option<T> {
        if self.len - 1 < index {
            panic!("Index out of bounds");
        }

        if index == 0 || self.head.is_none() {
            return self.delete_head();
        }

        if self.len == index + 1 {
            return self.delete_tail();
        }

        if let Some(mut node_i) = self.head {
            for _ in 0..index {
                unsafe {
                    match (*node_i.as_ptr()).next {
                        None => panic!("Index out of bounds"),
                        Some(next_ptr) => node_i = next_ptr,
                    }
                }
            }
            // lấy node ra chuyển thằng next của node trước liên kết với thằng pre của node sau
            unsafe {
                let old_ith = Box::from_raw(node_i.as_ptr());
                if let Some(mut prev) = old_ith.prev {
                    prev.as_mut().next = old_ith.next;
                }
                if let Some(mut next) = old_ith.next {
                    next.as_mut().prev = old_ith.prev;
                }

                self.len -= 1;
                Some(old_ith.val)
            }
        } else {
            None
        }
    }

    pub fn get(&mut self, index: i32) -> Option<&'static T> {
        Self::get_ith_node(self.head, index)
    }

    fn get_ith_node(node: Option<NonNull<Node<T>>>, index: i32) -> Option<&'static T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => Self::get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // Pop items until there are none left
        while self.delete_head().is_some() {}
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.head {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

pub fn linked_list() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_list() {
        let l = List::<u32>::new();
        assert_eq!(l.len, 0);
        assert_eq!(l.head, None);
        assert_eq!(l.tail, None);
    }

    #[test]
    fn test_empty_node() {
        let n = Node::new(32);
        assert_eq!(n.prev, None);
        assert_eq!(n.next, None);
        assert_eq!(n.val, 32);
    }

    #[test]
    fn push_tail_works() {
        let mut list = List::<i32>::new();
        let second_value = 2;
        list.push_tail(1);
        list.push_tail(second_value);
        println!("Linked List is {}", list);
        match list.get(1) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {} at index 1", second_value),
        }
    }
    #[test]
    fn push_head_works() {
        let mut list = List::<i32>::new();
        let second_value = 2;
        list.push_head(1);
        list.push_head(second_value);
        println!("Linked List is {}", list);
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {} at index 0", second_value),
        }
    }

    #[test]
    fn insert_at_index_can_add_to_tail() {
        let mut list = List::<i32>::new();
        let second_value = 2;
        list.insert_at_index(0, 0);
        list.insert_at_index(1, second_value);
        println!("Linked List is {}", list);
        match list.get(1) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {} at index 1", second_value),
        }
    }

    #[test]
    fn insert_at_index_can_add_to_head() {
        let mut list = List::<i32>::new();
        let second_value = 2;
        list.insert_at_index(0, 1);
        list.insert_at_index(0, second_value);
        println!("Linked List is {}", list);
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {} at index 0", second_value),
        }
    }

    #[test]
    fn insert_at_index_can_add_to_middle() {
        let mut list = List::<i32>::new();
        let second_value = 2;
        let third_value = 3;
        list.insert_at_index(0, 1);
        list.insert_at_index(1, second_value);
        list.insert_at_index(1, third_value);
        println!("Linked List is {}", list);
        match list.get(1) {
            Some(val) => assert_eq!(*val, third_value),
            None => panic!("Expected to find {} at index 1", third_value),
        }

        match list.get(2) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {} at index 1", second_value),
        }
    }

    #[test]
    fn insert_at_index_and_delete_at_ith_in_the_middle() {
        // Insert and delete in the middle of the list to ensure pointers are updated correctly
        let mut list = List::<i32>::new();
        let first_value = 0;
        let second_value = 1;
        let third_value = 2;
        let fourth_value = 3;

        list.insert_at_index(0, first_value);
        list.insert_at_index(1, fourth_value);
        list.insert_at_index(1, third_value);
        list.insert_at_index(1, second_value);

        list.delete_at_index(2);
        list.insert_at_index(2, third_value);

        for (i, expected) in [
            (0, first_value),
            (1, second_value),
            (2, third_value),
            (3, fourth_value),
        ] {
            match list.get(i) {
                Some(val) => assert_eq!(*val, expected),
                None => panic!("Expected to find {} at index {}", expected, i),
            }
        }
    }

    #[test]
    fn insert_at_index_and_delete_at_index_work_over_many_iterations() {
        let mut list = List::<i32>::new();
        for i in 0..100 {
            list.insert_at_index(i, i.try_into().unwrap());
        }

        // Pop even numbers to 50
        for i in 0..50 {
            println!("list.len {}", list.len);
            if i % 2 == 0 {
                list.delete_at_index(i);
            }
        }

        assert_eq!(list.len, 75);

        // Insert even numbers back
        for i in 0..50 {
            if i % 2 == 0 {
                list.insert_at_index(i, i.try_into().unwrap());
            }
        }

        assert_eq!(list.len, 100);

        // Ensure numbers were adderd back and we're able to traverse nodes
        if let Some(val) = list.get(78) {
            assert_eq!(*val, 78);
        } else {
            panic!("Expected to find 78 at index 78");
        }
    }

    #[test]
    fn delete_tail_works() {
        let mut list = List::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.push_tail(first_value);
        list.push_tail(second_value);
        match list.delete_tail() {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to remove {} at tail", second_value),
        }

        println!("Linked List is {}", list);
        match list.get(0) {
            Some(val) => assert_eq!(*val, first_value),
            None => panic!("Expected to find {} at index 0", first_value),
        }
    }

    #[test]
    fn delete_head_works() {
        let mut list = List::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.push_tail(first_value);
        list.push_tail(second_value);
        match list.delete_head() {
            Some(val) => assert_eq!(val, 1),
            None => panic!("Expected to remove {} at head", first_value),
        }

        println!("Linked List is {}", list);
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {} at index 0", second_value),
        }
    }

    #[test]
    fn delete_at_index_can_delete_at_tail() {
        let mut list = List::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.push_tail(first_value);
        list.push_tail(second_value);
        match list.delete_at_index(1) {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to remove {} at tail", second_value),
        }

        assert_eq!(list.len, 1);
    }

    #[test]
    fn delete_at_index_can_delete_at_head() {
        let mut list = List::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.push_tail(first_value);
        list.push_tail(second_value);
        match list.delete_at_index(0) {
            Some(val) => assert_eq!(val, 1),
            None => panic!("Expected to remove {} at tail", first_value),
        }

        assert_eq!(list.len, 1);
    }

    #[test]
    fn delete_at_index_can_delete_in_middle() {
        let mut list = List::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        let third_value = 3;
        list.push_tail(first_value);
        list.push_tail(second_value);
        list.push_tail(third_value);
        match list.delete_at_index(1) {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to remove {} at tail", second_value),
        }

        match list.get(1) {
            Some(val) => assert_eq!(*val, third_value),
            None => panic!("Expected to find {} at index 1", third_value),
        }
    }

    #[test]
    fn create_numeric_list() {
        let mut list = List::<i32>::new();
        list.push_tail(1);
        list.push_tail(2);
        list.push_tail(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.len);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = List::<String>::new();
        list_str.push_tail("A".to_string());
        list_str.push_tail("B".to_string());
        list_str.push_tail("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.len);
    }

    #[test]
    fn get_by_index_in_numeric_list() {
        let mut list = List::<i32>::new();
        list.push_tail(1);
        list.push_tail(2);
        println!("Linked List is {}", list);
        let retrived_item = list.get(1);
        assert!(retrived_item.is_some());
        assert_eq!(2 as i32, *retrived_item.unwrap());
    }

    #[test]
    fn get_by_index_in_string_list() {
        let mut list_str = List::<String>::new();
        list_str.push_tail("A".to_string());
        list_str.push_tail("B".to_string());
        println!("Linked List is {}", list_str);
        let retrived_item = list_str.get(1);
        assert!(retrived_item.is_some());
        assert_eq!("B", *retrived_item.unwrap());
    }
}
