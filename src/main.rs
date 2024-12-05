struct ListNode {
    val:Option<i32>,
    next:Option<Box<ListNode>>
}

trait LinkedList {
    fn init() -> ListNode;
    fn add_item(&mut self,itemval:i32);
    fn display_list(self);
}

impl LinkedList for  ListNode {
    fn init() -> ListNode {
        ListNode {
            val:None,
            next:None
        }
    } 

    fn add_item(&mut self, itemval:i32) {
        let mut current_node = self;

        while let Some(ref mut next_node) = current_node.next  {
            current_node = next_node   
        }

        let new_node = ListNode {val:Some(itemval),next:None};

        current_node.next = Some(Box::new(new_node))
    }

    fn display_list(self) {
        if let Some(val) = self.val  {
            println!("{:?}", val)
        }

        if let Some(next) = self.next {
            next.display_list();
        }else {
            println!("List has been displayed..");
            return;
        }

    }
    
}

fn main() {
    let mut node = ListNode::init();
    node.add_item(12);
    node.add_item(14);
    node.add_item(16);
    node.add_item(18);
    node.display_list();
}


