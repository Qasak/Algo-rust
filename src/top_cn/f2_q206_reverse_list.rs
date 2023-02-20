use crate::linked_list::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head;
    let mut pre = None;

    while let Some(mut cur_node) = cur {
        let nex = cur_node.next.take();
        cur_node.next = pre;
        pre = Some(cur_node);
        cur = nex;
    }
    pre
}
#[cfg(test)]
mod test {
    use crate::linked_list::ListNode;
    use crate::top_cn::f2_q206_reverse_list::reverse_list;

    #[test]
    pub fn t() {
        let h = Some(Box::from(ListNode {
            val: 1,
            next: Some(Box::from(ListNode {
                val: 2,
                next: Some(Box::from(ListNode {
                    val: 3,
                    next: Some(Box::from(ListNode {
                        val: 4,
                        next: Some(Box::from(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));

        let mut cur = reverse_list(h);
        while let Some(mut cur_node) = cur {
            println!("{}", cur_node.val);
            cur = cur_node.next;
        }
    }
}
