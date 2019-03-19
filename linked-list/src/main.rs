#![feature(box_into_raw_non_null)]

pub mod linkedlist;

fn main() {
    println!("Hello, world!");
}

impl<T> linkedlist::LinkedList<T>
where
    T: std::cmp::Eq + std::hash::Hash,
{
    // fn remove_duplicates(&self) {
    //     let set: std::collections::HashSet<T> = std::collections::HashSet::new();
    //     let mut temp = self.head;
    //     loop {
    //         match temp {
    //             None => break,
    //             Some(node) => set.get(node.data),
    //         }
    //     }
    // }
}
