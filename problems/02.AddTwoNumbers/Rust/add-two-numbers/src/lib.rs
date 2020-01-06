#![allow(dead_code)]
#![allow(unused_variables)]
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn tolink(val: i32) -> Option<Box<ListNode>> {
        let digit: i32 = (val as f32).log10().trunc() as i32;
        let mut current: Option<Box<ListNode>> = None;

        for i in (0..=digit).rev() {
            let base = 10i32.pow(i as u32);
            let div = val / base;
            let res = div - (div / 10) * 10;

            let mut node = ListNode::new(res);
            node.next = current;
            current = Some(Box::new(node));
        }

        current
    }
}
struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut res: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut res;
        let (mut p, mut q) = (&l1, &l2);
        let mut v1: i32;
        let mut v2: i32;
        let mut carry: i32 = 0;
        let (mut is_v1_none, mut is_v2_none, mut is_carry) = (false, false, false);

        loop {
            v1 = match p {
                Some(v) => {
                    is_v1_none = false;
                    p = &(v.next);
                    v.val
                }
                _ => {
                    is_v1_none = true;
                    0
                }
            };

            v2 = match q {
                Some(v) => {
                    is_v2_none = false;
                    q = &(v.next);
                    v.val
                }
                _ => {
                    is_v2_none = true;
                    0
                }
            };

            println!("is_v1_none: {:?}", is_v1_none);
            println!("is_v2_none: {:?}", is_v2_none);
            println!("is_carry: {:?}", is_carry);

            if is_v1_none && is_v2_none && !is_carry {
                res = res.unwrap().next;
                break;
            }

            let sum_v = v1 + v2 + carry;
            println!("sum_v: {}", sum_v);

            let val = if sum_v >= 10 {
                is_carry = true;
                carry = sum_v / 10;
                sum_v - (sum_v / 10) * 10
            } else {
                is_carry = false;
                carry = 0;
                sum_v
            };

            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(val)));
            tail = &mut tail.as_mut().unwrap().next
        }

        res
    }
}

#[test]
fn test_add_two_numbers() {
    let l1_v = 342;
    let l2_v = 465;
    let l1 = ListNode::tolink(l1_v);
    let l2 = ListNode::tolink(l2_v);
    let ans = ListNode::tolink(l1_v + l2_v);
    println!("l1: {:?}", l1);
    println!("l2: {:?}", l2);

    assert_eq!(Solution::add_two_numbers(l1, l2), ans);
}
