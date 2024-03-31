#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn middle(self) -> Self {
        let mut count: usize = 1;
        let mut head = &self.next;
        loop {
            match head {
                Some(node) => {
                    count += 1;
                    head = &node.next;
                },
                _ => {
                    break;
                }
            }
        }
        let middle: usize = (count + (2 - 1)) / 2;;
        println!("Linked list length is {count}, middle: {middle}");
        count %= 2;
        let mut head: Option<Box<ListNode>> = Some(Box::new(self));
        while count != middle {
            count += 1;
            head = head.unwrap().next;
        }
        *head.unwrap()
    }
}

fn main() {
    let list = from_slice(&[1, 2, 3, 4, 5]).unwrap();
    let middle = list.middle();
    dbg!(middle);
    let list = from_slice(&[1, 2]).unwrap();
    let middle = list.middle();
    dbg!(middle);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn middle_one() {
        let list = from_slice(&[1]).unwrap();
        let middle = list.middle();

        assert_eq!(middle.val, 1);
        assert!(middle.next.is_none());
    }

    #[test]
    fn middle_two() {
        let list = from_slice(&[1, 2]).unwrap();
        let middle = list.middle();
        let vec = to_vec(Some(Box::new(middle)));

        assert_eq!(&vec, &[2]);
    }

    #[test]
    fn middle_three() {
        let list = from_slice(&[1, 2, 3]).unwrap();
        let middle = list.middle();
        let vec = to_vec(Some(Box::new(middle)));

        assert_eq!(&vec, &[2, 3]);
    }

    #[test]
    fn middle_five() {
        let list = from_slice(&[1, 2, 3, 4, 5]).unwrap();
        let middle = list.middle();
        let vec = to_vec(Some(Box::new(middle)));

        assert_eq!(&vec, &[3, 4, 5]);
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
