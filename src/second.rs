use std::mem;

pub struct List<T>{
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;


struct Node<T> {
    elem: T,
    next: Link<T>
}


impl <T> List<T> {

    pub fn new() -> List<T>{
        // Return an emtpy list
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            // Option has a take method that implements this idiom
            // next: mem::replace(&mut self.head, None),
            next: self.head.take()
        });
     
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // map encapsulates the case 
        self.head.take().map(| node | {
            let node = *node;
            self.head = node.next;
            node.elem
        })
    }

}

impl <T> Drop for List<T> {
    fn drop(&mut self) {

        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

// tests R good
#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn push_pop(){
        // Create an empty list
        let mut list = List::new();

        // Are we sure its empty?
        assert_eq!(list.pop(), None);

        // Ok lets make it not empty
        let mut elems = [1, 2, 3, 4, 5, 6, 7, 8];
        for x in elems.iter() {
            list.push(*x);
        }

        // Make sure those elems made it in the list
        elems.reverse();
        for x in elems.iter(){
            assert_eq!(list.pop(), Some(*x));
        }

        // All elems should be gone
        // make sure nothing wonk is going on
        assert_eq!(list.pop(), None);
    }
    

}
