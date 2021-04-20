//! supports fast insertion and removal of elements from anywhere in the container `LinkedList<T>`
//!
//!It is implemented as a singly-linked list, unlike doublelinked this struct provide more space
//!efficient storage.
//!In linkedlist the elements are not required to stored at contagius memory location and each
//!member of the list stores the address of the next member.
//!
//!Most of the method of this struct has both recursive as well as iterative approach, the time
//!complexity may varies dependeng on the approach However sometime both example may be having same
//!time complexity in that case iterative approach is practically faster than recursive approach.
//!
//! # Examples
//! ```
//! # use dsa_sport::datastruct::list_struct::LinkedList;
//! let mut list = LinkedList::new();
//! list.add_node(1);
//! list.add_node(2);
//! list.add_node(3);
//! assert_eq!(format!("{:?}",list), format!("1 -> 2 -> 3 -> x"));
//! assert_eq!(list.len(), 3);
//! ```
//!

use core::mem;
use core::ptr;
use std::alloc;

pub struct LinkedList<T> {
    head: *mut Member<T>,
    tail: *mut Member<T>,
}

impl<T> LinkedList<T>
where
    T: PartialOrd + std::fmt::Debug,
{
    pub fn new() -> Self {
        return Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        };
    }

    /// add node from the tail position
    /// # Examples
    /// ```
    /// # use dsa_sport::datastruct::list_struct::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.add_node('a');
    /// list.add_node('b');
    /// list.add_node('c');
    /// assert_eq!(list.len(), 3);
    /// ```
    pub fn add_node(&mut self, element: T) {
        let node = Member::new(element);
        let node_ptr = Member::allocate_memory(node);
        self.push_back_node(node_ptr);
    }

    /// find the lenth of linkedlist using iterative approach O(n) and eliminate the overhead of
    /// recursive calling which make it faster than recursive approach
    pub fn len(&self) -> usize {
        let mut length = 0usize;
        if self.head.is_null() {
            length = 0;
        } else {
            let mut current_node = &self.head;
            length += 1;
            loop {
                unsafe {
                    if (*(*current_node)).next.is_null() {
                        break;
                    } else {
                        length += 1;
                        current_node = &(*(*current_node)).next;
                    }
                }
            }
        }
        return length;
    }

    /// find the length of linked list using a recursive approach O(n) but slower than
    /// [`LinkedList::len`]
    pub fn rec_len(&self) -> usize {
        if self.head.is_null() {
            return 0;
        } else {
            unsafe {
                return LinkedList::rec_len_util(self.head);
            }
        }
    }

    /// get the borrow of the element at index
    /// # Examples
    /// ```
    /// # use dsa_sport::datastruct::list_struct::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.add_node('a');
    /// list.add_node('b');
    /// list.add_node('c');
    /// assert_eq!(list.get_at(1), Some(&'b'));
    /// ```
    pub fn get_at(&self, index: usize) -> Option<&T> {
        let mut length = 0usize;
        if self.head.is_null() {
            return None;
        } else {
            let mut current_node = &self.head;
            length += 1;
            loop {
                unsafe {
                    if length > index {
                        break;
                    } else if (*(*current_node)).next.is_null() {
                        break;
                    } else {
                        length += 1;
                        current_node = &(*(*current_node)).next;
                    }
                }
            }
            return unsafe { Some(&(*(*current_node)).data) };
        }
    }

    /// return an option of mid element
    /// ```
    /// # use dsa_sport::datastruct::list_struct::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.add_node('a');
    /// list.add_node('b');
    /// list.add_node('c');
    /// assert_eq!(list.take_mid(), Some(&'b'));
    /// ```
    pub fn take_mid(&self) -> Option<&T> {
        if self.head.is_null() {
            return None;
        } else {
            unsafe {
                if (*self.head).next.is_null() {
                    return Some(&(*self.head).data);
                }
            }
            let mut slow = self.head;
            let mut fast = self.head;
            unsafe {
                while !(*fast).next.is_null() && !(*(*fast).next).next.is_null() {
                    slow = (*slow).next;
                    fast = (*(*fast).next).next;
                }
                return Some(&(*slow).data);
            }
        }
    }

    /// Find a node in linked list and return its index position if found else returns None
    /// ```
    /// # use dsa_sport::datastruct::list_struct::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.add_node('a');
    /// list.add_node('b');
    /// list.add_node('c');
    /// assert_eq!(list.find_node('b'), Some(1));
    /// assert_eq!(list.find_node('d'), None);
    /// ```
    pub fn find_node(&mut self, element: T) -> Option<usize>
    where
        T: PartialEq,
    {
        let mut current_node = self.head;
        let mut index: usize = 0;
        while !current_node.is_null() {
            unsafe {
                if (*current_node).data == element {
                    return Some(index);
                }
                current_node = (*current_node).next;
            }
            index += 1;
        }
        return None;
    }

    /// recursive approach for [`LinkedList::find_node`]
    pub fn recursive_find(&self, element: T) -> Option<usize>
    where
        T: PartialEq,
    {
        return LinkedList::recursive_find_helper(self.head, element);
    }

    /// insert node at given position
    /// ```
    /// # use dsa_sport::datastruct::list_struct::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.add_node(1);
    /// list.add_node(2);
    /// list.add_node(3);
    /// assert_eq!(format!("{:?}",list), format!("1 -> 2 -> 3 -> x"));
    /// list.insert_node_at(1, 4);
    /// assert_eq!(format!("{:?}",list), format!("1 -> 4 -> 2 -> 3 -> x"));
    /// ```
    pub fn insert_node_at(&mut self, index: usize, element: T) {
        let mut node = Member::new(element);

        if index == 0 {
            node.next = self.head;
            let node_ptr = Member::allocate_memory(node);
            self.head = node_ptr;
        } else {
            let mut count = 0;
            let mut current_node = self.head;
            while !current_node.is_null() && count < index - 1 {
                unsafe {
                    current_node = (*current_node).next;
                }
                count += 1;
            }
            if !current_node.is_null() {
                unsafe {
                    node.next = (*current_node).next;
                    let node_ptr = Member::allocate_memory(node);
                    (*current_node).next = node_ptr;
                }
            }
        }
    }

    /// recursive approach for [`LinkedList::insert_node_at`]
    pub fn insert_node_at_rec(&mut self, index: usize, element: T) {
        let node = Member::new(element);
        let node_ptr = Member::allocate_memory(node);
        let new_head = LinkedList::insert_node_at_rec_help(index, self.head, node_ptr);
        self.head = new_head;
    }

    /// delete node at given position
    /// ```
    /// # use dsa_sport::datastruct::list_struct::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.add_node(1);
    /// list.add_node(2);
    /// list.add_node(3);
    /// assert_eq!(format!("{:?}",list), format!("1 -> 2 -> 3 -> x"));
    /// list.delete_node_at(1);
    /// assert_eq!(format!("{:?}",list), format!("1 -> 3 -> x"));
    /// ```
    pub fn delete_node_at(&mut self, index: usize) {
        let align = mem::align_of::<Member<T>>();
        let size = mem::size_of::<Member<T>>();
        if index == 0 {
            let old_head = self.head;
            unsafe {
                self.head = (*old_head).next;
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                alloc::dealloc(old_head as *mut u8, layout)
            }
        } else {
            let mut count = 0;
            let mut current_node = self.head;
            while !current_node.is_null() && count < index - 1 {
                unsafe {
                    current_node = (*current_node).next;
                }
                count += 1;
            }
            unsafe {
                if !current_node.is_null() && !(*current_node).next.is_null() {
                    let a = (*current_node).next;
                    let b = (*a).next;
                    (*current_node).next = b;
                    let layout = alloc::Layout::from_size_align_unchecked(size, align);
                    alloc::dealloc(a as *mut u8, layout)
                }
            }
        }
    }

    /// recursive approach for [`LinkedList::delete_node_at`]
    pub fn delete_node_at_rec(&mut self, index: usize) {
        let new_head = LinkedList::delete_node_at_rec_help(index, self.head);
        self.head = new_head;
    }

    /// swap node of the linkedlist by its index value
    /// ```
    /// # use dsa_sport::datastruct::list_struct::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.add_node(1);
    /// list.add_node(2);
    /// list.add_node(3);
    /// assert_eq!(format!("{:?}",list), format!("1 -> 2 -> 3 -> x"));
    /// list.swape_nodes(0, 2);
    /// assert_eq!(format!("{:?}",list), format!("3 -> 2 -> 1 -> x"));
    /// ```
    pub fn swape_nodes(&mut self, i: usize, j: usize) {
        if i == j {
            return;
        }

        let mut current = self.head;
        let mut previous = ptr::null_mut() as *mut Member<T>;
        let mut current_one = ptr::null_mut() as *mut Member<T>;
        let mut previous_one = ptr::null_mut() as *mut Member<T>;
        let mut current_two = ptr::null_mut() as *mut Member<T>;
        let mut previous_two = ptr::null_mut() as *mut Member<T>;

        let mut pos = 0;

        while !current.is_null() {
            if pos == i {
                previous_one = previous;
                current_one = current;
            } else if pos == j {
                previous_two = previous;
                current_two = current;
            }
            previous = current;
            current = unsafe { (*current).next };
            pos += 1;
        }
        if !previous_one.is_null() {
            unsafe {
                (*previous_one).next = current_two;
            }
        } else {
            self.head = current_two;
        }

        if !previous_two.is_null() {
            unsafe {
                (*previous_two).next = current_one;
            }
        } else {
            self.head = current_one;
        }

        unsafe {
            let current_one_temp = (*current_two).next;
            (*current_two).next = (*current_one).next;
            (*current_one).next = current_one_temp;
        }
    }

    /// remove duplicate from the linkedlistt
    /// ```
    /// # use dsa_sport::datastruct::list_struct::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.add_node(1);
    /// list.add_node(2);
    /// list.add_node(2);
    /// list.add_node(3);
    /// list.add_node(4);
    /// list.add_node(4);
    /// assert_eq!(format!("{:?}",list), format!("1 -> 2 -> 2 -> 3 -> 4 -> 4 -> x"));
    /// list.eliminate_dup();
    /// assert_eq!(format!("{:?}",list), format!("1 -> 2 -> 3 -> 4 -> x"));
    /// ```
    pub fn eliminate_dup(&mut self)
    where
        T: PartialEq,
    {
        if self.head.is_null() {
            return;
        }
        let len = self.rec_len();
        if len == 1 {
            return;
        }
        let align = mem::align_of::<Member<T>>();
        let size = mem::size_of::<Member<T>>();
        let mut t1 = self.head;
        let mut t2 = unsafe { (*self.head).next };
        let mut __ptr = ptr::null_mut();
        while !t2.is_null() {
            unsafe {
                if (*t1).data == (*t2).data {
                    __ptr = t2;
                    t2 = (*t2).next;
                    let layout = alloc::Layout::from_size_align_unchecked(size, align);
                    alloc::dealloc(__ptr as *mut u8, layout);
                } else {
                    (*t1).next = t2;
                    t1 = t2;
                    t2 = (*t2).next;
                }
            }
        }
        unsafe {
            (*t1).next = t2;
        }
    }

    ///
    /// returns true if the linkedlist id palindrome
    /// ```
    /// # use dsa_sport::datastruct::list_struct::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.add_node('w');
    /// list.add_node('o');
    /// list.add_node('w');
    /// assert_eq!(list.is_palindrome(), true);
    /// let mut list = LinkedList::new();
    /// list.add_node('a');
    /// list.add_node('w');
    /// list.add_node('w');
    /// assert_eq!(list.is_palindrome(), false);
    /// ```
    pub fn is_palindrome(&self) -> bool
    where
        T: PartialEq,
    {
        unsafe {
            if self.head.is_null() || (*self.head).next.is_null() {
                return true;
            }
        }
        let mut fast = self.head;
        let mut slow = self.head;
        unsafe {
            while !(*fast).next.is_null() && !(*(*fast).next).next.is_null() {
                fast = (*(*fast).next).next;
                slow = (*slow).next;
            }
        }
        let mut head2 = unsafe { (*slow).next };
        unsafe {
            (*slow).next = ptr::null_mut();
            head2 = LinkedList::reverse_list(head2);
        }
        let mut sublist1 = self.head;
        let mut sublist2 = head2;
        let mut ans = true;

        unsafe {
            while !sublist2.is_null() {
                if (*sublist1).data != (*sublist2).data {
                    ans = false;
                    break;
                }
                sublist1 = (*sublist1).next;
                sublist2 = (*sublist2).next;
            }
        }

        sublist1 = self.head;
        sublist2 = LinkedList::reverse_list(head2);

        unsafe {
            while !(*sublist1).next.is_null() {
                sublist1 = (*sublist1).next;
            }
            (*sublist1).next = sublist2;
        }
        return ans;
    }

    ///
    /// reverse the linkedlist
    /// ```
    /// # use dsa_sport::datastruct::list_struct::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.add_node(1);
    /// list.add_node(2);
    /// list.add_node(3);
    /// list.add_node(4);
    /// list.add_node(5);
    /// assert_eq!(format!("{:?}",list), format!("1 -> 2 -> 3 -> 4 -> 5 -> x"));
    /// list.reverse_iterative();
    /// assert_eq!(format!("{:?}",list), format!("5 -> 4 -> 3 -> 2 -> 1 -> x"));
    /// ```
    pub fn reverse_iterative(&mut self) {
        let mut prev = ptr::null_mut() as *mut Member<T>;
        let mut curr = self.head;
        let mut _next = ptr::null_mut() as *mut Member<T>;
        unsafe {
            while !curr.is_null() {
                _next = (*curr).next;
                (*curr).next = prev;
                prev = curr;
                curr = _next;
            }
        }
        self.head = prev;
    }

    /// recursive approach for [`LinkedList::reverse_iterative`]
    pub fn reverse_recursive(&mut self) {
        self.head = LinkedList::reverse_recursive_helper(self.head);
    }

    /// sort the linkedlist into two category where formal elements are odd number followed by all
    /// even number elements.
    /// ```
    /// # use dsa_sport::datastruct::list_struct::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.add_node(1);
    /// list.add_node(2);
    /// list.add_node(3);
    /// list.add_node(4);
    /// list.add_node(5);
    /// assert_eq!(format!("{:?}",list), format!("1 -> 2 -> 3 -> 4 -> 5 -> x"));
    /// list.activate_kejriwal();
    /// assert_eq!(format!("{:?}",list), format!("1 -> 3 -> 5 -> 2 -> 4 -> x"));
    /// ```
    /// # Errors
    /// this function needs the type to be `LinkedList<i32>` only
    /// ```compile_fail
    /// # use dsa_sport::datastruct::list_struct::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.add_node('a');
    /// list.add_node('b');
    /// list.add_node('c');
    /// list.add_node('d');
    /// list.add_node('e');
    /// list.activate_kejriwal();
    /// ```
    pub fn activate_kejriwal(&mut self)
    where
        T: Copy + std::ops::Rem<Output = T> + PartialEq + From<i32>,
    {
        if self.head.is_null() {
            return;
        }
        let two: T = 2i32.into();
        let zero: T = 0i32.into();
        let mut odd_head = ptr::null_mut() as *mut Member<T>;
        let mut odd_tail = ptr::null_mut() as *mut Member<T>;
        let mut even_head = ptr::null_mut() as *mut Member<T>;
        let mut even_tail = ptr::null_mut() as *mut Member<T>;
        let mut c_node = self.head;
        loop {
            if c_node.is_null() {
                break;
            }
            unsafe {
                if (*c_node).data % two == zero {
                    if even_head.is_null() {
                        even_head = c_node;
                        even_tail = c_node;
                    } else {
                        (*even_tail).next = c_node;
                        even_tail = c_node;
                    }
                } else {
                    if odd_head.is_null() {
                        odd_head = c_node;
                        odd_tail = c_node;
                    } else {
                        (*odd_tail).next = c_node;
                        odd_tail = c_node;
                    }
                }
                c_node = (*c_node).next;
            }
        }
        unsafe {
            if odd_head.is_null() {
                self.head = even_head;
                self.tail = even_tail;
            } else {
                if even_head.is_null() {
                    self.head = odd_head;
                    self.tail = odd_tail;
                } else {
                    self.head = odd_head;
                    (*odd_tail).next = even_head;
                    self.tail = even_tail;
                    (*even_tail).next = ptr::null_mut();
                }
            }
        }
    }

    /// take last k element and append it to the front of the list
    /// ```
    /// # use dsa_sport::datastruct::list_struct::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.add_node(1);
    /// list.add_node(2);
    /// list.add_node(3);
    /// list.add_node(4);
    /// list.add_node(5);
    /// assert_eq!(format!("{:?}",list), format!("1 -> 2 -> 3 -> 4 -> 5 -> x"));
    /// list.rotate_from_k(2);
    /// assert_eq!(format!("{:?}",list), format!("4 -> 5 -> 1 -> 2 -> 3 -> x"));
    /// ```
    pub fn rotate_from_k(&mut self, last_n: usize) {
        let size = self.rec_len();
        if size < last_n {
            return;
        }
        let count = size - last_n;
        if count == size {
            return;
        }
        if last_n == 0 || self.head.is_null() {
            return;
        } else {
            let mut i = 1;
            let head_1 = self.head;
            let mut current_head = self.head;
            while i < count {
                let h = unsafe { (*current_head).next };
                if h.is_null() {
                    break;
                } else {
                    current_head = h;
                    i += 1;
                }
            }
            let head_2 = unsafe { (*current_head).next };
            unsafe {
                (*current_head).next = ptr::null_mut();
            }
            let mut temp_head = head_2;
            unsafe {
                loop {
                    let temp_next = (*temp_head).next;
                    if temp_next.is_null() {
                        break;
                    } else {
                        temp_head = temp_next;
                    }
                }
                (*temp_head).next = head_1;
            }
            self.head = head_2;
        }
    }

    /// O(n^2) sorting algorithm
    /// ```
    /// # use dsa_sport::datastruct::list_struct::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.add_node("abc");
    /// list.add_node("abd");
    /// list.add_node("aad");
    /// list.add_node("bcd");
    /// list.add_node("bac");
    /// list.bubble_sort();
    /// assert_eq!(format!("{:?}",list), format!("\"aad\" -> \"abc\" -> \"abd\" -> \"bac\" -> \"bcd\" -> x"));
    /// ```
    pub fn bubble_sort(&mut self) {
        for _ in 0..self.len() {
            let mut current = self.head;
            let mut previous = ptr::null_mut() as *mut Member<T>;
            unsafe {
                while !(*current).next.is_null() {
                    if (*current).data > (*(*current).next).data {
                        if !previous.is_null() {
                            let temp = (*(*current).next).next;
                            (*(*current).next).next = current;
                            (*previous).next = (*current).next;
                            (*current).next = temp;
                            previous = (*previous).next;
                        } else {
                            self.head = (*current).next;
                            (*current).next = (*self.head).next;
                            (*self.head).next = current;
                            previous = self.head;
                        }
                    } else {
                        previous = current;
                        current = (*current).next;
                    }
                }
            }
        }
    }

    /// O(n log n) sorting algorithm
    /// ```
    /// # use dsa_sport::datastruct::list_struct::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.add_node("abc");
    /// list.add_node("abd");
    /// list.add_node("aad");
    /// list.add_node("bcd");
    /// list.add_node("bac");
    /// list.bubble_sort();
    /// assert_eq!(format!("{:?}",list), format!("\"aad\" -> \"abc\" -> \"abd\" -> \"bac\" -> \"bcd\" -> x"));
    /// ```
    pub fn merge_sort(&mut self)
    where
        T: PartialOrd,
    {
        self.head = LinkedList::merge_sort_helper(self.head);
    }

    fn delete_node_at_rec_help(index: usize, mut head: *mut Member<T>) -> *mut Member<T> {
        if head.is_null() {
            return head;
        }
        if index == 0 {
            let align = mem::align_of::<Member<T>>();
            let size = mem::size_of::<Member<T>>();
            let old_head = head;
            unsafe {
                head = (*old_head).next;
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                alloc::dealloc(old_head as *mut u8, layout);
            }
        } else {
            unsafe {
                let node_ptr = LinkedList::delete_node_at_rec_help(index - 1, (*head).next);
                (*head).next = node_ptr;
            }
        }
        return head;
    }

    fn insert_node_at_rec_help(
        index: usize,
        mut head: *mut Member<T>,
        node: *mut Member<T>,
    ) -> *mut Member<T> {
        if index == 0 {
            unsafe {
                (*node).next = head;
                head = node;
            }
        } else {
            unsafe {
                let node_ptr = LinkedList::insert_node_at_rec_help(index - 1, (*head).next, node);
                (*head).next = node_ptr;
            }
        }
        return head;
    }

    fn push_back_node(&mut self, new_node: *mut Member<T>) {
        if self.head.is_null() {
            self.head = new_node;
            self.tail = new_node;
        } else {
            unsafe {
                (*self.tail).next = new_node;
                self.tail = (*self.tail).next;
                //self.tail = new_node;
            }
        }
    }

    unsafe fn rec_len_util(node: *mut Member<T>) -> usize {
        if node.is_null() {
            return 0;
        } else {
            return 1 + LinkedList::rec_len_util((*node).next);
        }
    }

    fn merge_sort_helper(head: *mut Member<T>) -> *mut Member<T>
    where
        T: PartialOrd,
    {
        unsafe {
            if head.is_null() || (*head).next.is_null() {
                return head;
            }
        }
        let mid = LinkedList::get_mid(head);
        let mut half1 = head;
        let mut half2 = ptr::null_mut();
        if !mid.is_null() {
            unsafe {
                half2 = (*mid).next;
                (*mid).next = ptr::null_mut();
            }
        }
        half1 = LinkedList::merge_sort_helper(half1);
        half2 = LinkedList::merge_sort_helper(half2);

        let final_head = unsafe { (*half1).merge_member(half2) };
        return final_head;
    }

    fn recursive_find_helper(head: *mut Member<T>, element: T) -> Option<usize>
    where
        T: PartialEq,
    {
        if head.is_null() {
            return None;
        }
        unsafe {
            if (*head).data == element {
                return Some(0);
            }
        }
        let ans = unsafe { LinkedList::recursive_find_helper((*head).next, element) };
        match ans {
            None => return None,
            Some(x) => return Some(x + 1),
        }
    }

    fn reverse_recursive_helper(head: *mut Member<T>) -> *mut Member<T> {
        unsafe {
            if head.is_null() || (*head).next.is_null() {
                return head;
            } else {
                let res = LinkedList::reverse_recursive_helper((*head).next);
                let tail = (*head).next;
                (*tail).next = head;
                (*head).next = ptr::null_mut();
                return res;
            }
        }
    }

    fn get_mid(head: *mut Member<T>) -> *mut Member<T> {
        if head.is_null() {
            return head;
        }
        let mut slow = head;
        let mut fast = head;
        unsafe {
            while !fast.is_null() && !(*fast).next.is_null() && !(*(*fast).next).next.is_null() {
                slow = (*slow).next;
                fast = (*(*fast).next).next;
            }
        }
        return slow;
    }

    fn reverse_list(head: *mut Member<T>) -> *mut Member<T> {
        let mut cur = head;
        let mut prv = ptr::null_mut();
        let mut _fwd = ptr::null_mut();
        while !cur.is_null() {
            unsafe {
                _fwd = (*cur).next;
                (*cur).next = prv;
                prv = cur;
                cur = _fwd;
            }
        }
        return prv;
    }
}

struct Member<T> {
    data: T,
    next: *mut Member<T>,
}

impl<T> std::fmt::Debug for LinkedList<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        if self.head.is_null() {
            out += "x";
        } else {
            let mut current_node = &self.head;
            unsafe {
                out += &format!("{:?} -> ", (*(*current_node)).data);
            }
            loop {
                unsafe {
                    if (*(*current_node)).next.is_null() {
                        out += "x";
                        break;
                    }
                    current_node = &(*(*current_node)).next;
                    out += &format!("{:?} -> ", (*(*current_node)).data);
                }
            }
        }
        write!(f, "{}", out)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        Member::deallocate_memory(self.head);
    }
}

impl<T> Member<T> {
    fn new(element: T) -> Self {
        return Self {
            data: element,
            next: ptr::null_mut(),
        };
    }

    fn merge_member(&mut self, mut node2: *mut Member<T>) -> *mut Member<T>
    where
        T: PartialOrd,
    {
        let mut node1 = self as *mut Member<T>;
        if node1.is_null() {
            return node2;
        }
        if node2.is_null() {
            return node1;
        }
        let mut _head = ptr::null_mut();
        let mut _tail = ptr::null_mut();

        // set head
        unsafe {
            if (*node1).data <= (*node2).data {
                _head = node1;
                _tail = node1;
                node1 = (*node1).next;
            } else {
                _head = node2;
                _tail = node2;
                node2 = (*node2).next;
            }
        }

        // set tail
        unsafe {
            while !node1.is_null() && !node2.is_null() {
                if (*node1).data <= (*node2).data {
                    (*_tail).next = node1;
                    _tail = node1;
                    node1 = (*node1).next;
                } else {
                    (*_tail).next = node2;
                    _tail = node2;
                    node2 = (*node2).next;
                }
            }

            if !node1.is_null() {
                (*_tail).next = node1;
            }
            if !node2.is_null() {
                (*_tail).next = node2;
            }
        }

        return _head;
    }

    fn allocate_memory(candidate: T) -> *mut T {
        let align = mem::align_of::<T>();
        let size = mem::size_of::<T>();
        let node_ptr = unsafe {
            let layout = alloc::Layout::from_size_align_unchecked(size, align);
            let ptr = alloc::alloc(layout) as *mut T;
            ptr.write(candidate);
            ptr
        };
        return node_ptr;
    }

    fn deallocate_memory(mut head: *mut Member<T>) {
        let mut count = 0;
        let align = mem::align_of::<Member<T>>();
        let size = mem::size_of::<Member<T>>();
        unsafe {
            let layout = alloc::Layout::from_size_align_unchecked(size, align);
            while !head.is_null() {
                let old_head = head;
                head = (*old_head).next;
                alloc::dealloc(old_head as *mut u8, layout);
                count += 1;
            }
        }
        println!("delocate : {}", count);
    }
}
