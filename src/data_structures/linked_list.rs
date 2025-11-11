//! single linked list

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>
}

impl<T> Node<T> {
    fn new(t:T) -> Self {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        let node_ptr = Some( unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>> , index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(node_ptr) => match index {
                0 => Some(unsafe { &(*node_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*node_ptr.as_ptr()).next }, index - 1),
            }
        }
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self 
    where 
        T: PartialOrd + Clone,
    {
        let mut merge_list = LinkedList::new();
        let mut current_a = list_a.start;
        let mut current_b = list_b.start;

        while current_a.is_some() && current_b.is_some() {
            let node_a = unsafe { current_a.unwrap().as_ref() };
            let node_b = unsafe { current_b.unwrap().as_ref() };
            match node_a.val <= node_b.val {
                true => { 
                    merge_list.add(node_a.val.clone());
                    current_a = node_a.next ;
                },
                false => {
                    merge_list.add(node_b.val.clone());
                    current_b = node_b.next ;
                }
            } 
        }

        while current_a.is_some() {
            let node_a = unsafe { current_a.unwrap().as_ref() };
            merge_list.add(node_a.val.clone());
            current_a = node_a.next;
        }

        while current_b.is_some() {
            let node_b = unsafe { current_b.unwrap().as_ref() };
            merge_list.add(node_b.val.clone());
            current_b = node_b.next;
        }

        merge_list
    }
}

impl<T> Display for LinkedList<T> 
where
    T: Display
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.start;
        while let Some(node_ptr) = current {
            // 获取下一个节点的指针（在释放当前节点前）
            current = unsafe { (*node_ptr.as_ptr()).next };
            // 将原始指针转换回Box，然后自动释放
            unsafe {
                let drop_node = Box::from_raw(node_ptr.as_ptr());
                println!("-----drop done------");
            }
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1,3,5,7];
        let vec_b = vec![2,4,6,8];
        let target_vec = vec![1,2,3,4,5,6,7,8];

        for i in 0..vec_a.len(){
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len(){
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a,list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len(){
            assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11,33,44,88,89,90,100];
        let vec_b = vec![1,22,30,45];
        let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

        for i in 0..vec_a.len(){
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len(){
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a,list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len(){
            assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
        }
    }
}