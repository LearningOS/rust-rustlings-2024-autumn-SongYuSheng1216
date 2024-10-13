/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
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
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
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

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    fn result_add(&mut self, value : T) -> () {
        self.add(value);
    }

    #[allow(unused)]
    // 其实这算法是有问题的
	pub fn merge(mut list_a:LinkedList<T>,mut list_b:LinkedList<T>) -> Self
        where   T: Copy + PartialOrd
    {
		//TODO
        let mut result : LinkedList<T> = LinkedList::new();
        // NonNull as_ptr获取底层的*mut指针
        // NonNull as_ref返回该值的共享引用
        let mut a_start = list_a.start;
        let mut b_start = list_b.start;
        while a_start.is_some() || b_start.is_some() {
            match (a_start, b_start){
                (Some(a), Some(b)) => {
                    let a_value = unsafe {a.as_ref() }.val;
                    let b_value = unsafe {b.as_ref() }.val;
                    if a_value > b_value{
                        result.add(b_value);
                        b_start = unsafe {b.as_ref() }.next;
                    } else {
                        result.add(a_value);
                        a_start = unsafe {a.as_ref() }.next;
                    }
                }
                (Some(a), None) => {
                    let a_value = unsafe {a.as_ref() }.val;
                    result.add(a_value);
                    a_start = unsafe {a.as_ref() }.next;
                }
                (None, Some(b)) => {
                    let b_value = unsafe {b.as_ref() }.val;
                    result.add(b_value);
                    b_start = unsafe {b.as_ref() }.next;      
                }
                (None, None) => {

                }
            }
        }
        result
	}
//     pub fn merge(mut list_a:LinkedList<T>,mut list_b:LinkedList<T>) -> Self
//     where   T: Copy + PartialOrd + Debug
// {
//     //TODO
//     let mut all_element : LinkedList<T> = LinkedList::new();
//     let mut result : LinkedList<T> = LinkedList::new();
//     // NonNull as_ptr获取底层的*mut指针
//     // NonNull as_ref返回该值的共享引用
//     let all_length = list_a.length + list_b.length;
//     println!("all_length : {:?}", all_length);
//     for i in 0..all_length {
//         if i < list_a.length {
//             println!("i : {:?}", i);
//             all_element.add(*list_a.get(i as i32).unwrap());
//         }else {
//             println!("i - list_a.length : {:?}", i - list_a.length);
//             println!("i : {:?}", i);
//             all_element.add(*list_b.get(i as i32 - list_a.length as i32).unwrap());
//         }
//     }

//     //list_a排序
//     let list_start = all_element.start;
//     let mut min = -1;
//     let mut min_record;
//     for i in 0..all_element.length {
//         // 获取min的值，完成min的递增操作
//         min += 1;
//         min_record = min;
//         let mut min_value = *all_element.get(min).unwrap();
//         println!("min_value : {:?}", min_value);
//         for j in min + 1..all_element.length as i32 {
//             // 获取j的值
//             let j_index_value = *all_element.get(j).unwrap();

//             // 让j的值和min的值进行比较
//             if(min_value > j_index_value) {
//                 min_value = j_index_value;
//                 min_record = j;
//             }
//         }
//         // 让index j和index min交换
//         if min_record != min {
            // 交换太难写了
//         }
//         //result.add(min_value);
//     }
//     result
// }
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

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        //let mut list = LinkedList::<i32>::new();
        let mut list : LinkedList<i32> = LinkedList::new();
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