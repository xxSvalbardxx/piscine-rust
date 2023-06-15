pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }
    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }
    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        //self.ref_list.retain(|x| x != &element); // retain() keeps all elements that satisfy the condition
        self.ref_list.retain(|x| x == &element); // retain() keeps all elements that satisfy the condition
        //self.ref_list.shrink_to_fit(); // shrink_to_fit() reduces the capacity to the minimum. the condition is that the reference count is greater than 0
        //self.ref_list.retain(|x| Rc::strong_count(x) > 0); // retain() keeps all elements that satisfy the condition. the condition is that the reference count is greater than 0
    
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}
