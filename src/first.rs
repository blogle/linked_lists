/// enum List {
///     Nil,
///     Cons(i32, Box<List>)
/// }
///
/// let ll = List::Cons (1, Box<List::Cons (2, Box<List::Cons(3, Box< List::Nil >)>))
/// * (1 : 2 : 3 : []) without all the line noise
///
/// The reason this implementation is bad,
/// is because it non-uniformly allocates nodes
/// 
/// the outer Cons exists on the stack while everyone else is on the heap
/// [] = stack, () = heap
/// [1, ptr] -> (2, ptr) -> (3, ptr) -> (nullptr)
/// * variants with no fields are represented as a nullptr
///
/// splitting and joining require extra work
///
/// for example if I want the lists
/// [1, 2] and [3]
///
/// then we need to traverse to 2 pull out the pointer to the sublist 3
/// allocate an Empty node on the heap, stuff a pointer to it back in where 3 
/// previously was, and finally dereference the pointer to sublist 
/// 3 to get it on the stack
///
/// resulting in:
/// [1, ptr] -> (2, ptr) -> (nullptr)
/// [3, ptr] -> (nullptr)
///
/// a better representation would only have a pointer on the stack
/// and avoid allocating the space for the nullptr which will be the
/// size of the largest variant. eg something like
///
/// [ptr] -> (1, ptr) -> (2, ptr) -> (3, nullptr)
/// we can effectively do this by putting the entire contents of 
/// Cons behind a ptr

use std::mem;

// I think this struct is rather needless
// but the book insists
pub struct List {
    head: Link
}

// Alternatively you could achieve a similar
// effect by keeping the implementation local
// by using a tuple. eg Cons(Box<(i32, Link)>)
enum Link {
    Nil,
    Cons(Box<Node>)
}

struct Node {
    elem: i32,
    next: Link
}


impl List {

    pub fn new() -> List {
        // Return an emtpy list
        List { head: Link::Nil }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            // mem::replace replaces the value at a mutable
            // location with a new value returning the old
            next: mem::replace(&mut self.head, Link::Nil),
        });
     
        // modify the contents of the list
        // with the new node
        self.head = Link::Cons(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Nil) {
            Link::Nil => None,
            Link::Cons(boxed_node) => {
                let node = *boxed_node;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {

        let mut cur_link = mem::replace(&mut self.head, Link::Nil);
        // while let == do this thing until this pattern fails
        while let Link::Cons(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Nil);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's next field has been set to Link::Empty
            // so no unbounded recursion occurs
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
