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
//! assert_eq!(format!("{}", v), "▅");
//! v.push_back('A');
//! v.push_back('B');
//! v.push_back('C');
//! assert_eq!(format!("{}", v), "⎩A⎭⎩B⎭⎩C⎭⎩▅⎭");
//! ```
use core::mem;
use core::ops::Index;
use core::ptr;
use std::alloc;

const LEFT: &str = "⎩";
const PHI: &str = "▅";
const RIGHT: &str = "⎭";

pub struct Vector<T> {
    pointer: *mut T,
    front: Option<usize>,
    back: Option<usize>,
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
            front: None,
            back: None,
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
    /// ```
    pub fn push_back(&mut self, element: T) {
        let size = mem::size_of::<T>();
        if size == 0 {
            panic!("Size of element must be non zero");
        }
        let align = mem::align_of::<T>();

        if self.pointer.is_null() {
            let vec_capacity = 4;
            let vec_size = size * vec_capacity;
            let vec_ptr = unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(vec_size, align);
                let raw_ptr = alloc::alloc(layout) as *mut T;
                raw_ptr.write(element);
                raw_ptr
            };
            self.pointer = vec_ptr;
            self.front = Some(0);
            self.back = Some(0);
            self.length += 1;
            self.capacity = vec_capacity;
        } else if self.length < self.capacity {
            let next_index = {
                match self.back {
                    Some(back) => {
                        let next_index = back + 1;
                        if next_index < self.capacity {
                            next_index
                        } else {
                            next_index % self.capacity
                        }
                    }
                    None => 0,
                }
            };
            unsafe {
                self.pointer.add(next_index).write(element);
            }
            if let None = self.front {
                self.front = Some(0);
            }
            self.back = Some(next_index);
            self.length += 1;
        } else {
            if let Some(new_capacity) = self.capacity.checked_mul(2) {
                if let Some(front) = self.front {
                    let old_vec_size = size * self.capacity;
                    let new_vec_size = size * new_capacity;
                    let vec_ptr = unsafe {
                        let old_layout =
                            alloc::Layout::from_size_align_unchecked(old_vec_size, align);
                        let new_layout =
                            alloc::Layout::from_size_align_unchecked(new_vec_size, align);
                        let raw_ptr = alloc::alloc(new_layout) as *mut T;
                        if front == 0 {
                            ptr::copy_nonoverlapping(self.pointer, raw_ptr, self.capacity);
                        } else {
                            let part_size = self.capacity - front;
                            ptr::copy_nonoverlapping(
                                self.pointer.offset(front as isize),
                                raw_ptr,
                                part_size,
                            );
                            ptr::copy_nonoverlapping(
                                self.pointer,
                                raw_ptr.offset(part_size as isize),
                                self.capacity - part_size,
                            );
                        }
                        raw_ptr.add(self.length).write(element);
                        alloc::dealloc(self.pointer as *mut u8, old_layout);
                        raw_ptr
                    };
                    self.pointer = vec_ptr;
                    self.front = Some(0);
                    self.back = Some(self.length);
                    self.length += 1;
                    self.capacity = new_capacity;
                }
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
    /// assert_eq!(v.pop_front(), Some('B'));
    /// assert_eq!(v.pop_front(), Some('C'));
    /// assert_eq!(v.pop_front(), Some('D'));
    /// assert_eq!(v.pop_front(), None);
    /// assert_eq!(v.len(), 0);
    /// assert_eq!(v.capacity(), 4);
    /// v.push_back('A');
    /// assert_eq!(v.pop_front(), Some('A'));
    /// assert_eq!(v.len(), 0);
    /// assert_eq!(v.capacity(), 4);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        if !self.pointer.is_null() && self.length > 0 {
            match self.front {
                Some(front_ixd) => {
                    let item = unsafe { self.pointer.add(front_ixd).read() };
                    self.front = Some((front_ixd + 1) % self.capacity);
                    self.length -= 1;
                    return Some(item);
                }
                None => return None,
            }
        } else {
            return None;
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
            match self.back {
                Some(back_ixd) => {
                    let item = unsafe { self.pointer.add(back_ixd).read() };
                    if back_ixd > 0 {
                        self.back = back_ixd.checked_sub(1);
                    } else {
                        self.back = self.capacity.checked_sub(1);
                    }
                    self.length -= 1;
                    if self.length == 0 {
                        self.front = None;
                        self.back = None;
                    }
                    return Some(item);
                }
                None => return None,
            }
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
        let mut total_offset = self.capacity;
        let mut total_index = self.length;
        if let Some(mut offset) = self.front {
            while total_index > 0 {
                unsafe {
                    out.push_str(&format!(
                        "{}{}{}",
                        LEFT,
                        self.pointer.add(offset).read(),
                        RIGHT
                    ));
                }
                offset = (offset + 1) % self.capacity;
                total_index -= 1;
                total_offset -= 1;
            }
            while total_offset > 0 {
                out.push_str(&format!("{}{}{}", LEFT, PHI, RIGHT));
                total_offset -= 1;
            }
        } else {
            while total_offset > 0 {
                out.push_str(&format!("{}{}{}", LEFT, PHI, RIGHT));
                total_offset -= 1;
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
        let mut total_offset = self.capacity;
        let mut total_index = self.length;
        if let Some(mut offset) = self.front {
            while total_index > 0 {
                unsafe {
                    out.push_str(&format!(
                        "{}{:?}{}",
                        LEFT,
                        self.pointer.add(offset).read(),
                        RIGHT
                    ));
                }
                offset = (offset + 1) % self.capacity;
                total_index -= 1;
                total_offset -= 1;
            }
            while total_offset > 0 {
                out.push_str(&format!("{}{}{}", LEFT, PHI, RIGHT));
                total_offset -= 1;
            }
        } else {
            while total_offset > 0 {
                out.push_str(&format!("{}{}{}", LEFT, PHI, RIGHT));
                total_offset -= 1;
            }
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if !self.pointer.is_null() && self.length > 0 && index < self.length {
            if let Some(mut front_ixd) = self.front {
                front_ixd = (front_ixd + index) % self.capacity;
                return unsafe { self.pointer.add(front_ixd).as_ref() };
            } else {
                return None;
            }
        } else {
            return None;
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
            out.push_str(&format!("{}", PHI));
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
            out.push_str(&format!("{}", PHI));
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
