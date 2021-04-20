//! A contiguous growable array written `Vector<T>`.
//!                                                                                                       
//! Vectors ensure they never allocate zero sized element and can grow up to `isize::MAX` bytes.                                                                                             
//! The elements of a vector are stored contiguously and can be accessed using offsets. The storage       
//! of the vector is handled automatically by expanding the memory as needed and hence a vector           
//! data structure takes more memmory as compaired to `std::array`                                        
//!          
//! # Examples
//! ```rust
//! use dsa_sport::datastruct::vec_struct::Vector;
//! let mut v: Vector<char> = Vector::new();
//! assert_eq!(format!("{}", v), "⎩∅⎭");
//! v.push_back('A');
//! v.push_back('B');
//! v.push_back('C');
//! assert_eq!(format!("{}", v), "⎩A⎭⎩B⎭⎩C⎭");
//! ```
use core::mem;
use core::ptr;
use std::alloc;
use core::ops::Index;

pub struct Vector<T> {
    pointer: *mut T,
    length: usize,
    capacity: usize,
}

// public associated functions
impl<T> Vector<T> {
    /// # Examples
    /// ```rust
    /// use dsa_sport::datastruct::vec_struct::Vector;
    /// let v: Vector<char> = Vector::new();
    /// ```
    pub fn new() -> Self {
        Self {
            pointer: ptr::null_mut(),
            length: 0,
            capacity: 0,
        }
    }

    /// # Examples
    /// ```rust
    /// use dsa_sport::datastruct::vec_struct::Vector;
    /// let mut v: Vector<char> = Vector::new();
    /// assert_eq!(v.len(), 0);
    /// v.push_back('A');
    /// assert_eq!(v.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        return self.length;
    }

    /// # Examples
    /// ```rust
    /// use dsa_sport::datastruct::vec_struct::Vector;
    /// let mut v: Vector<char> = Vector::new();
    /// assert_eq!(v.capacity(), 0);
    /// v.push_back('A');
    /// assert_eq!(v.capacity(), 1);
    /// v.push_back('B');
    /// assert_eq!(v.capacity(), 2);
    /// v.push_back('C');
    /// assert_eq!(v.capacity(), 4);
    /// ```
    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    /// # Examples
    /// ```rust
    /// use dsa_sport::datastruct::vec_struct::Vector;
    /// let mut v: Vector<char> = Vector::new();
    /// assert!(v.is_empty());
    /// v.push_back('A');
    /// assert!(!v.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        return self.length == 0;
    }

    /// write a non zero sized element from the back of the vector,
    /// # Panics
    /// ```should_panic
    ///# use dsa_sport::datastruct::vec_struct::Vector;
    ///struct Dummy;
    ///let mut v: Vector<Dummy> = Vector::new();
    ///v.push_back(Dummy);
    /// ```
    /// # Safety
    /// Declering a Vector doesnot allocate memory in the heap, While initializing the vector it is assume that the capacity of Vector is 4 initially
    /// so to avoid zero sized allocation of memory.
    /// more to see from [`core::alloc::GlobalAlloc::alloc`].
    /// # Examples
    /// ```rust
    /// use dsa_sport::datastruct::vec_struct::Vector;
    /// let mut v: Vector<char> = Vector::new();
    /// v.push_back('A');
    /// v.push_back('B');
    /// v.push_back('C');
    /// v.push_back('D');
    /// ```
    pub fn push_back(&mut self, element: T) {
        let size = mem::size_of::<T>();
        if size == 0 {
            panic!("Size of element must be non zero");
        }

        if self.pointer.is_null() {
            let align = mem::align_of::<T>();
            let vec_ptr = unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let ptr = alloc::alloc(layout) as *mut T;
                ptr.write(element);
                ptr
            };
            self.pointer = vec_ptr;
            self.length += 1;
            self.capacity += 1;
        } else if self.length < self.capacity {
            unsafe {
                self.pointer.add(self.length).write(element);
            }
            self.length += 1;
        } else {
            if let Some(new_capacity) = self.capacity.checked_mul(2) {
                let old_vec_size = size * self.capacity;
                let new_vec_size = size * new_capacity;
                let align = mem::align_of::<T>();
                let vec_ptr = unsafe {
                    let layout = alloc::Layout::from_size_align_unchecked(old_vec_size, align);
                    let ptr =
                        alloc::realloc(self.pointer as *mut u8, layout, new_vec_size) as *mut T;
                    ptr.add(self.length).write(element);
                    ptr
                };
                self.pointer = vec_ptr;
                self.length += 1;
                self.capacity = new_capacity;
            } else {
                panic!("usize vector capacity reached its limit");
            }
        }
    }

    /// # Examples
    /// ```rust
    /// use dsa_sport::datastruct::vec_struct::Vector;
    /// let mut v: Vector<char> = Vector::new();
    /// v.push_back('B');
    /// v.push_back('C');
    /// v.push_back('D');
    /// assert_eq!(v.pop_back(), Some('D'));
    /// assert_eq!(v.pop_back(), Some('C'));
    /// assert_eq!(v.pop_back(), Some('B'));
    /// assert_eq!(v.pop_back(), None);
    /// assert_eq!(v.len(), 0);
    /// assert_eq!(v.capacity(), 4);
    /// v.push_back('A');
    /// assert_eq!(v.pop_back(), Some('A'));
    /// assert_eq!(v.len(), 0);
    /// assert_eq!(v.capacity(), 4);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        if !self.pointer.is_null() && self.length > 0 {
            let item = unsafe { self.pointer.add(self.length - 1).read() };
            self.length -= 1;
            return Some(item);
        } else {
            return None;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        todo!()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if !self.pointer.is_null() && self.length > 0 {
            return unsafe {self.pointer.add(index).as_ref()};
        } else {
            return None;
        }
    }
}

// private associated functions
impl<T> Vector<T> {
    fn deallocate_memory(&mut self) {
        let vec_size = mem::size_of::<T>() * self.capacity;
        let align = mem::align_of::<T>();
        unsafe {
            let layout = alloc::Layout::from_size_align_unchecked(vec_size, align);
            alloc::dealloc(self.pointer as *mut u8, layout)
        }
    }

    fn prety_print(&self, out: &mut String)
    where
        T: std::fmt::Display,
    {
        for i in 0..self.length {
            unsafe {
                out.push_str(&format!("⎩{}⎭", self.pointer.add(i).read()));
            }
        }
    }

    fn vec_debug(&self, out: &mut String)
    where
        T: std::fmt::Debug,
    {
        out.push_str(&format!(
            "Length = {} Capacity = {} -> ",
            self.length, self.capacity
        ));
        for i in 0..self.length {
            unsafe {
                out.push_str(&format!("⎩{:?}⎭", self.pointer.add(i).read()));
            }
        }
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        Vector::deallocate_memory(self);
    }
}

impl<T> std::fmt::Display for Vector<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        if !self.pointer.is_null() {
            self.prety_print(&mut out);
        } else {
            out.push_str("⎩∅⎭");
        }
        write!(f, "{}", out)
    }
}

impl<T> std::fmt::Debug for Vector<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        if !self.pointer.is_null() {
            self.vec_debug(&mut out);
        } else {
            out.push_str("⎩∅⎭");
        }
        write!(f, "{}", out)
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).expect("Out of bounds access")
    }
}
