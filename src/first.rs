/// struct List {
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