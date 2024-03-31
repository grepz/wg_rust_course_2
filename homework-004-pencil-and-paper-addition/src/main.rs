#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn box_new(val: i32) -> Box<Self> {
        Box::new(
            ListNode {
                next: None,
                val
            }
        )
    }

    pub fn overflow(a: i32, b: i32, rem: i32) -> (i32, i32) {
        match a + b + rem {
            n if n >= 10 => { (n - 10, 1) },
            m => { (m, 0) }
        }
    }

    pub fn reverse(&mut self) {
        let mut current = self.next.take();
        let mut head = Some(ListNode::box_new(self.val));

        while let Some(mut node) = current {
            let next = node.next;
            node.next = head;
            head = Some(node);
            current = next;
        }

        _ = std::mem::replace(self, *head.unwrap());
    }


    fn reverse2(&mut self) {
        let mut head_old = Some(ListNode::box_new(self.val));
        let head = &mut self.next;
        let tail = &mut head_old;
        while head.is_some() {
            std::mem::swap(&mut head.as_mut().unwrap().next, tail);
            std::mem::swap(head, tail);
        }
        std::mem::swap(self, tail.as_mut().unwrap());
    }

    fn pencil_addition(&mut self, other: ListNode) {
        let mut other = other;
        self.reverse();
        other.reverse();

        let mut rem;
        let mut self_head = &mut self.next;
        let mut other_head = other.next;

        println!("Starting point:\n\tself: {:?}\n\tother: {:?}", self_head, other_head);

        (self.val, rem) = ListNode::overflow(self.val, other.val, 0);

        loop {
            println!("Loop:\n\tself: {:?}\n\tother: {:?}\n\trem: {}", self_head, other_head, rem);
            match (self_head, other_head) {
                (Some(self_node), Some(other_node)) => {
                    (self_node.val, rem) =
                        ListNode::overflow(self_node.val, other_node.val, rem);
                    self_head = &mut self_node.next;
                    other_head = other_node.next;
                },
                (Some(self_node), None) => {
                    (self_node.val, rem) =
                        ListNode::overflow(self_node.val, 0, rem);
                    self_head = &mut self_node.next;
                    other_head = None;
                },
                (f @ None, Some(mut other_node)) => {
                    (other_node.val, rem) =
                        ListNode::overflow(0, other_node.val, rem);
                    other_head = None;
                    *f = Some(other_node);
                    self_head = &mut f.as_mut().unwrap().next;
                },
                (f @ None, None) => {
                    if rem != 0 {
                        *f = Some(ListNode::box_new(rem));
                    }
                    break;
                },
            }
        }

        self.reverse();
    }
}


fn main() {
    let mut one = from_slice(&[9, 8]).unwrap();
    let two = from_slice(&[2, 1]).unwrap();
    one.pencil_addition(two);
    dbg!(one);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero() {
        let mut one = from_slice(&[0]).unwrap();
        let two = from_slice(&[0]).unwrap();
        one.pencil_addition(two);
        assert_eq!(to_vec(Some(Box::new(one))), vec![0]);
    }

    #[test]
    fn simple() {
        let mut one = from_slice(&[1, 2, 3]).unwrap();
        let two = from_slice(&[4, 5, 6]).unwrap();
        one.pencil_addition(two);
        assert_eq!(to_vec(Some(Box::new(one))), vec![5, 7, 9]);
    }

    #[test]
    fn nine_in_the_middle() {
        let mut one = from_slice(&[1, 9, 8, 3]).unwrap();
        let two = from_slice(&[1, 2, 7]).unwrap();

        one.pencil_addition(two);
        assert_eq!(to_vec(Some(Box::new(one))), vec![2, 1, 1, 0]);
    }

    #[test]
    fn nine_in_the_middle_trans() {
        let mut two = from_slice(&[1, 2, 7]).unwrap();
        let one = from_slice(&[1, 9, 8, 3]).unwrap();

        two.pencil_addition(one);
        assert_eq!(to_vec(Some(Box::new(two))), vec![2, 1, 1, 0]);
    }

    #[test]
    fn nines() {
        let mut one = from_slice(&[9, 9, 9, 9, 9]).unwrap();
        let two = from_slice(&[9, 9, 9, 9]).unwrap();
        one.pencil_addition(two);
        assert_eq!(to_vec(Some(Box::new(one))), vec![1, 0, 9, 9, 9, 8]);
    }

    #[test]
    fn nines_trans() {
        let one = from_slice(&[9, 9, 9, 9, 9]).unwrap();
        let mut two = from_slice(&[9, 9, 9, 9]).unwrap();
        two.pencil_addition(one);
        assert_eq!(to_vec(Some(Box::new(two))), vec![1, 0, 9, 9, 9, 8]);
    }
}

fn from_slice(slice: &[i32]) -> Option<ListNode> {
    slice.iter().rev().fold(None, |next, &val| {
        Some(ListNode { val, next: next.map(Box::new) })
    })
}

fn to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    while let Some(node) = list {
        result.push(node.val);
        list = node.next;
    }
    result
}
