//! minimal tree structure written `Tree<T>`
//!
//! formula for tree type sada storage functionality
//! it is implemented as n type tree and support any general case of tree like bst
//! # Example
//! ```
//! # use dsa_sport::datastruct::tree_struct::Tree;
//! let mut tree = Tree::new();
//! ```
use core::mem;
use core::ptr;
use std::alloc;
use crate::datastruct::vec_struct::Vector;

const EDGE: &str = "╠═";
const LINE: &str = "║ ";
const CORNER: &str = "╚═";
const BLANK: &str = "  ";

struct Member<T> {
    data: T,
    stem: Vector<*mut Member<T>>,
}

impl<T> Member<T> {
    fn new(element: T) -> Self {
        return Self {
            data: element,
            stem: Vector::new(),
        };
    }

    fn allocate_memory(candidate: Member<T>) -> *mut Member<T> {
        let size = mem::size_of::<Member<T>>();
        let align = mem::align_of::<Member<T>>();
        let node_ptr = unsafe {
            let layout = alloc::Layout::from_size_align_unchecked(size, align);
            let ptr = alloc::alloc(layout) as *mut Member<T>;
            ptr.write(candidate);
            ptr
        };
        return node_ptr;
    }
}

pub struct Tree<T> {
    root: *mut Member<T>,
}

impl Tree<String> {
    pub fn new() -> Self {
        return Self {
            root: ptr::null_mut(),
        };
    }

    /// initiate the tree with a special tree grammar
    /// ``` text
    ///  ┍━━━━┑ ┍━━━━━┑
    ///  root 3 A B C 2 D E 2 F G 0 0 0 0 1 H 0
    ///           ┕━┿━━━━━━━┙     │
    ///             ┕━━━━━━━━━━━━━┙
    /// ```
    /// # Example
    /// ``` rust
    /// use dsa_sport::datastruct::tree_struct::Tree;
    /// let mut tree = Tree::new();
    /// let tree_code = "Plants;2;no seed;seed;3;algae;Mosses;ferns;2;Conifers;sunflower";
    /// tree.init_tree(tree_code);
    /// ```
    pub fn init_tree(&mut self, tree_code: &str) {
        let tree_code_raw = tree_code.split(";");

        // let mut tree_code: Vector<Vec<u8>> = tree_code.map(|x| x.as_bytes().to_owned()).collect();
        let mut tree_code: Vector<Vec<u8>> = Vector::new();
        for i in tree_code_raw {
            tree_code.push_back(i.as_bytes().to_owned());
        }


        if tree_code.len() > 0 {
            let mut member_queue = Vector::new();
            if let Some(root) = tree_code.pop_front() {
                if let Ok(root) = std::str::from_utf8(&root) {
                    let root = Member::new(root.to_owned());
                    let root_ptr = Member::allocate_memory(root);
                    self.root = root_ptr;
                    member_queue.push_back(root_ptr);
                };

                while member_queue.len() > 0 {
                    if let Some(candidate_ptr) = member_queue.pop_front() {
                        if let Some(stem_size) = tree_code.pop_front() {
                            if let Ok(stem_size) = std::str::from_utf8(&stem_size) {
                                if let Ok(mut stem_size) = stem_size.parse::<usize>() {
                                    while stem_size > 0 {
                                        if let Some(node) = tree_code.pop_front() {
                                            if let Ok(node) = std::str::from_utf8(&node) {
                                                let node = Member::new(node.to_owned());
                                                let node_ptr = Member::allocate_memory(node);
                                                unsafe {
                                                    (*candidate_ptr).stem.push_back(node_ptr);
                                                }
                                                member_queue.push_back(node_ptr);
                                            };
                                        };
                                        stem_size -= 1;
                                    }
                                };
                            };
                        };
                    };
                }
            };
        }
    }

    fn tree_display(root: *mut Member<String>, states: &mut Vector<&str>, out: &mut String) {
        if root.is_null() {
            return;
        }
        let node = unsafe { &(*root) };
        for i in 0..states.len() {
            out.push_str(states[i]);
        }
        out.push_str(&format!("{}\n", node.data));
        if states[states.len() - 1] == CORNER {
            states.pop_back();
            states.push_back(BLANK);
        } else {
            states.pop_back();
            states.push_back(LINE);
        }
        for i in 0..node.stem.len() {
            if i == node.stem.len() - 1 {
                states.push_back(CORNER);
            } else {
                states.push_back(EDGE);
            }
            Tree::tree_display(node.stem[i], states, out);
            states.pop_back();
        }
    }


    fn tree_debug(root: *mut Member<String>, out: &mut String) {
        if root.is_null() {
            out.push_str("--");
        } else {
            out.push_str(unsafe { &format!("{}:", (*root).data) });
            unsafe {
                for i in 0..(*root).stem.len() {
                    out.push_str(&format!("{},", (*(*root).stem[i]).data));
                }
            }
            out.push_str("\n");
            unsafe {
                for i in 0..(*root).stem.len() {
                    Tree::tree_debug((*root).stem[i], out);
                }
            }
        }
    }
}

impl std::fmt::Display for Tree<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        let mut states = Vector::new();
        states.push_back(CORNER);
        Tree::tree_display(self.root, &mut states, &mut out);
        write!(f, "{}", out)
    }
}

impl std::fmt::Debug for Tree<String> {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        Tree::tree_debug(self.root, &mut out);
        write!(f, "{}", out)
    }
}
