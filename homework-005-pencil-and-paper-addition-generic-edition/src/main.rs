use std::fmt::Debug;
use std::ops::{Add, Sub};
use std::cmp::PartialOrd;

#[derive(Debug)]
struct ListNode<T> {
    val: T,
    next: Option<Box<ListNode<T>>>,
}

pub trait SmartPencilAdd<X> {
    fn add_spec(&self) -> (X, X);
}

impl SmartPencilAdd<i8> for ListNode<i8> {
    fn add_spec(&self) -> (i8, i8) {
        (1, 2)
    }
}

impl SmartPencilAdd<i16> for ListNode<i16> {
    fn add_spec(&self) -> (i16, i16) {
        (7, 8)
    }
}

impl SmartPencilAdd<i32> for ListNode<i32> {
    fn add_spec(&self) -> (i32, i32) {
        (9, 10)
    }
}

impl SmartPencilAdd<i64> for ListNode<i64> {
    fn add_spec(&self) -> (i64, i64) {
        (15, 16)
    }
}

impl SmartPencilAdd<i128> for ListNode<i128> {
    fn add_spec(&self) -> (i128, i128) {
        (-1, -1)
    }
}



// impl SmartPencilAdd<i128> for ListNode<i128> {
// }


// impl From<ListNode<i8>> for ListNode<i16> {
//     fn from(node: ListNode<i8>) -> ListNode<i16> {
//         let mut head_node  = ListNode{
//             val: node.val as i16,
//             next: None
//         };
//         let mut ptr = &mut head_node.next;
//         let mut current = node.next;
//         while let Some(node) = current {
//             *ptr = Some(ListNode::box_new(node.val as i16));
//             ptr = &mut (*ptr).as_mut().unwrap().next;
//             current = node.next;
//         }
//         head_node
//     }
// }

impl<T> ListNode<T> where
    T: From<i8> +
    Add<Output = T> + Sub<Output = T> + PartialOrd +
    Copy + Debug,
    ListNode<T>: SmartPencilAdd<T>
{
    fn convert<F>(node: ListNode<F>) -> ListNode<T> where T: From<F> {
        let mut head_node: ListNode<T>  = ListNode{
            val: node.val.into(),
            next: None
        };
        let mut ptr = &mut head_node.next;
        let mut current = node.next;
        while let Some(node) = current {
            let new_node: Box<ListNode<T>> =
                ListNode::box_new(node.val.into());
            *ptr = Some(new_node);
            ptr = &mut (*ptr).as_mut().unwrap().next;
            current = node.next;
        }
        head_node
    }

    pub fn box_new(val: T) -> Box<ListNode<T>> {
        Box::new(
            ListNode {
                next: None,
                val
            }
        )
    }

    pub fn overflow(a: T, b: T, max: T, divider: T, rem: i8) -> (T, i8)  {
        match a + b.into() + rem.into() {
            n if n > max => { (n - divider, 1) },
            m => { (m, 0) }
        }
    }

    fn reverse(&mut self) {
        let mut head_old = Some(ListNode::box_new(self.val));
        let head = &mut self.next;
        let tail = &mut head_old;
        while head.is_some() {
            std::mem::swap(&mut head.as_mut().unwrap().next, tail);
            std::mem::swap(head, tail);
        }
        std::mem::swap(self, tail.as_mut().unwrap());
    }

    fn pencil_addition_smart<X>(&mut self, other: ListNode<X>) where T: From<X> {
        let max;
        let divider;
        (max, divider) = self.add_spec();
        if max < 0.into() {
            panic!("Can't use smart addition for i128.");
        }
        self.pencil_addition(other, max, divider);
    }

    fn pencil_addition<X>(&mut self, other: ListNode<X>, max: T, divider: T) where T: From<X> {
        let mut other = ListNode::convert(other);
        self.reverse();
        other.reverse();

        let mut rem;
        let mut self_head = &mut self.next;
        let mut other_head = other.next;

        println!("Starting point:\n\tself: {:?}\n\tother: {:?}", self_head, other_head);

        (self.val, rem) = ListNode::overflow(self.val, other.val, max, divider, 0);

        loop {
            println!("Loop:\n\tself: {:?}\n\tother: {:?}\n\trem: {}", self_head, other_head, rem);
            match (self_head, other_head) {
                (Some(self_node), Some(other_node)) => {
                    (self_node.val, rem) =
                        ListNode::overflow(self_node.val, other_node.val, max, divider, rem);
                    self_head = &mut self_node.next;
                    other_head = other_node.next;
                },
                (Some(self_node), None) => {
                    (self_node.val, rem) =
                        ListNode::overflow(self_node.val, 0.into(), max, divider, rem);
                    self_head = &mut self_node.next;
                    other_head = None;
                },
                (f @ None, Some(mut other_node)) => {
                    (other_node.val, rem) =
                        ListNode::overflow(0.into(), other_node.val, max, divider, rem);
                    other_head = None;
                    *f = Some(other_node);
                    self_head = &mut f.as_mut().unwrap().next;
                },
                (f @ None, None) => {
                    if rem != 0 {
                        *f = Some(ListNode::box_new(rem.into()));
                    }
                    break;
                },
            }
        }

        self.reverse();
    }

    // Change solution to be generic
    // ListNode<T> + ListNode<T> -> ListNode<T>
    // T may be Copy

    // *
    // ListNode<i64> + ListNode<i32> -> ListNode<i64>
    // ListNode<i128> + ListNode<i32> -> ListNode<i128>
    // ListNode<i32> + ListNode<i16> -> ListNode<i32>

    // **
    // fn pencil_addition(&mut self, other: ListNode) {}
    // ListNode<i8> + ListNode<i8> -> binary number system
    // ListNode<i16> + ListNode<i16> -> octal number system
    // ListNode<i32> + ListNode<i32> -> decimal number system
    // ListNode<i64> + ListNode<i64> -> hexadecimal number system
}

fn main() {
    let mut one = from_slice(&[9, 9, 9, 9, 9]).unwrap();
    let two = from_slice(&[1, 1, 1, 1, 1]).unwrap();

    let val: i64 = 1024;
    let node = ListNode{
        val,
        next: None
    };

    dbg!(ListNode::overflow(node.val, 0, 1023, 1024, 0));

    one.pencil_addition(two, 9, 10);
    dbg!(one);

    // let slice_one: &[i8] = &[5, 4, 3, 2, 1];
    // let test = from_slice(slice_one);
    // let test_i16: ListNode<i16> = test.unwrap().into();
    // println!("==> {:?}", test_i16);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero() {
        let mut one = from_slice(&[0]).unwrap();
        let two = from_slice(&[0]).unwrap();
        one.pencil_addition(two, 9, 10);
        assert_eq!(to_vec(Some(Box::new(one))), vec![0]);
    }

    #[test]
    fn simple() {
        let mut one = from_slice(&[1, 2, 3]).unwrap();
        let two = from_slice(&[4, 5, 6]).unwrap();
        one.pencil_addition(two, 9, 10);
        assert_eq!(to_vec(Some(Box::new(one))), vec![5, 7, 9]);
    }

    #[test]
    fn nine_in_the_middle() {
        let mut one = from_slice(&[1, 9, 8, 3]).unwrap();
        let two = from_slice(&[1, 2, 7]).unwrap();

        one.pencil_addition(two, 9, 10);
        assert_eq!(to_vec(Some(Box::new(one))), vec![2, 1, 1, 0]);
    }

    #[test]
    fn nine_in_the_middle_trans() {
        let mut two = from_slice(&[1, 2, 7]).unwrap();
        let one = from_slice(&[1, 9, 8, 3]).unwrap();

        two.pencil_addition(one, 9, 10);
        assert_eq!(to_vec(Some(Box::new(two))), vec![2, 1, 1, 0]);
    }

    #[test]
    fn nines() {
        let mut one = from_slice(&[9, 9, 9, 9, 9]).unwrap();
        let two = from_slice(&[9, 9, 9, 9]).unwrap();
        one.pencil_addition(two, 9, 10);
        assert_eq!(to_vec(Some(Box::new(one))), vec![1, 0, 9, 9, 9, 8]);
    }

    #[test]
    fn nines_trans() {
        let one = from_slice(&[9, 9, 9, 9, 9]).unwrap();
        let mut two = from_slice(&[9, 9, 9, 9]).unwrap();
        two.pencil_addition(one, 9, 10);
        assert_eq!(to_vec(Some(Box::new(two))), vec![1, 0, 9, 9, 9, 8]);
    }

    #[test]
    fn test_8_and_8_nines_trans() {
        let slice_one: &[i8] = &[9, 9, 9, 9, 9];
        let slice_two: &[i8] = &[9, 9, 9, 9];
        let one = from_slice(slice_one).unwrap();
        let mut two = from_slice(slice_two).unwrap();
        two.pencil_addition(one, 9, 10);
        assert_eq!(to_vec(Some(Box::new(two))), vec![1, 0, 9, 9, 9, 8]);
    }

    #[test]
    fn test_16_and_8_nines_trans() {
        let slice_one: &[i8] = &[9, 9, 9, 9, 9];
        let slice_two: &[i16] =   &[9, 9, 9, 9];
        let one = from_slice(slice_one).unwrap();
        let mut two = from_slice(slice_two).unwrap();
        two.pencil_addition(one, 9, 10);
        assert_eq!(to_vec(Some(Box::new(two))), vec![1, 0, 9, 9, 9, 8]);
    }

    #[test]
    fn test_128_and_8_nines_trans() {
        let slice_one: &[i8] = &[9, 9, 9, 9, 9];
        let slice_two: &[i128] =   &[9, 9, 9, 9];
        let one = from_slice(slice_one).unwrap();
        let mut two = from_slice(slice_two).unwrap();
        two.pencil_addition(one, 9, 10);
        assert_eq!(to_vec(Some(Box::new(two))), vec![1, 0, 9, 9, 9, 8]);
    }

    #[test]
    fn test_64_and_32_nines_trans() {
        let slice_one: &[i32] = &[9, 9, 9, 9, 9];
        let slice_two: &[i64] =   &[9, 9, 9, 9];
        let one = from_slice(slice_one).unwrap();
        let mut two = from_slice(slice_two).unwrap();
        two.pencil_addition(one, 9, 10);
        assert_eq!(to_vec(Some(Box::new(two))), vec![1, 0, 9, 9, 9, 8]);
    }

    #[test]
    fn test_smart_add_non_i128() {
        let slice_one: &[i16] = &[9, 9, 9, 9, 9];
        let slice_two: &[i32] = &[9, 9, 9, 9];
        let one = from_slice(slice_one).unwrap();
        let mut two = from_slice(slice_two).unwrap();
        two.pencil_addition_smart(one);
        assert_eq!(to_vec(Some(Box::new(two))), vec![1, 0, 9, 9, 9, 8]);
    }

    #[test]
    #[should_panic]
    fn test_smart_add_i128_panic() {
        let slice_one: &[i16] = &[9, 9, 9, 9, 9];
        let slice_two: &[i128] = &[9, 9, 9, 9];
        let one = from_slice(slice_one).unwrap();
        let mut two = from_slice(slice_two).unwrap();
        two.pencil_addition_smart(one);
    }

    #[test]
    fn test_1024_trans() {
        let mut one = from_slice(&[1023, 0, 1023]).unwrap();
        let two = from_slice(&[1]).unwrap();
        one.pencil_addition(two, 1023, 1024);
        assert_eq!(to_vec(Some(Box::new(one))), vec![1023, 1, 0]);
    }


    fn to_vec<T>(mut list: Option<Box<ListNode<T>>>) -> Vec<T> {
        let mut result = vec![];
        while let Some(node) = list {
            result.push(node.val);
            list = node.next;
        }
        result
    }
}

fn from_slice<T: Copy>(slice: &[T]) -> Option<ListNode<T>> {
    slice.iter().rev().fold(None, |next, &val| {
        Some(ListNode { val, next: next.map(Box::new) })
    })
}
