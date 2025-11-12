//! double linked list


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(t: T) -> Self {
        Node {
            val: t,
            prev: None,
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


impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
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
        node.prev = self.end;
        let node_ptr = Some( unsafe { NonNull::new_unchecked(Box::into_raw(node)) } );
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

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T>{
        match node {
            None => None,
            Some(node_ptr) => match index {
                0 => Some( unsafe {  &(*node_ptr.as_ptr()).val  }),
                _ => self.get_ith_node( unsafe { (*node_ptr.as_ptr()).next }, index - 1 ),
            }
        }
    }

    pub fn reverse(&mut self) {
        // 如果链表为空或只有一个元素，不需要反转
        if self.length <= 1 {
            return;
        }
        // 遍历所有节点，交换每个节点的prev 和 next 指针
        let mut current = self.start;
        while let Some(node_ptr) = current {
            unsafe {
                let node = node_ptr.as_ptr();
                // 保存下一个节点（反转前的next）
                let next_node = (*node).next;

                // 交换当前节点 prev 和 next 指针
                std::mem::swap(&mut (*node).prev, &mut (*node).next);

                // 移动到下一个节点（现在存储在原来的next中）
                current = next_node;
            }
        }
        std::mem::swap(&mut self.start, &mut self.end);
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
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2,3,5,11,9,7];
        let reverse_vec = vec![7,9,11,5,3,2];
        for i in 0..original_vec.len(){
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len(){
            assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![34,56,78,25,90,10,19,34,21,45];
        let reverse_vec = vec![45,21,34,19,10,90,25,78,56,34];
        for i in 0..original_vec.len(){
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len(){
            assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
        }
    }
}




