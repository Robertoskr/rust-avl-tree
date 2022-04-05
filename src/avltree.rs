use std::cmp::Ordering;
use std::mem;
type Link = Option<Box<Node>>;

// InsertResult = (newNode, depeened)
// newNode means if a newNode is added to the tree if not this node already exists
// depeneed means if the subnode has been depeneed (a node without childrens that has been added a new children)
type InsertResult = (bool, bool);

enum ChildDirection{
    Left,
    Right
}

#[derive(Debug, Clone)]
pub struct Node{
    height: i32,
    key: i32, 
    value: i32,
    left: Link,
    right: Link,
    balance_factor: i32,
}

impl Node{
    fn new(key: i32, value: i32) -> Node{
        Node{
            height: 0,
            balance_factor: 0,
            key: key,
            value: value,
            left: None,
            right: None,
        }
    }

    fn default() -> Node{
        Node::new(0, 0)
    }

    fn new_link(key: i32, value: i32) -> Link{
        Some(Box::new(Node::new(key, value)))
    }
}

#[derive(Debug, Clone)]
pub struct AvlTree{
    root: Link,
    size: i32, //the total nodes in the tree
}

impl AvlTree{
    pub fn new() -> AvlTree{
        AvlTree{
            root: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, key: i32, value: i32){
        let new_node: bool;
        (self.root, new_node) = self._recursive_insert(self.root.clone(), key, value);

        if new_node {
            //do something fancy with this info 
        }
        // how to call not recursive : 
        // let result: bool = self._insert(key,value);
        // if result == true {
        //      self.size += 1;
        // }
    }

    //inserts a value in the avlTree (without recursion)
    //we are not using this, because is more difficult and require 
    //more logic for keep udpated the balance factor of each node
    fn _insert(&mut self,key: i32, value: i32) -> bool{
        let mut tree_node = &mut self.root;

        while let Some(current_node) = tree_node{
            match current_node.value.cmp(&key){
                Ordering::Greater => {
                    tree_node = &mut current_node.left;
                },
                Ordering::Less => {
                    tree_node = &mut current_node.right;
                },
                Ordering::Equal => return false,
            }
        }
        *tree_node = Node::new_link(key, value);
        true
    }

    //insert a node recursively
    fn _recursive_insert(&mut self, node: Link, key: i32, value: i32) -> (Link, bool){
        if node.is_some(){
            let mut _node = node.unwrap();
            if key > _node.value{
                let (link, newNode) = self._recursive_insert(
                    _node.right,
                    key,
                    value
                );
                _node.right = link;
                if newNode {
                    AvlTree::update_node(&mut _node);
                    _node = AvlTree::balance_node(_node);
                }
                (Some(_node), newNode)
            }else if key < _node.value{
                let (link, newNode) = self._recursive_insert(
                    _node.left,
                    key,
                    value
                );
                _node.left = link;
                if newNode {
                    AvlTree::update_node(&mut _node);
                    _node = AvlTree::balance_node(_node);
                }
                (Some(_node), newNode)
            }else{
                (Some(_node), false)
            }
        }else{
            self.size += 1;
            (Node::new_link(key, value), true)
        }
    }

    pub fn delete(&mut self, key: i32) {
        self.root = AvlTree::_delete(self.root, key);
    }

    fn _delete(node: Link, key: i32) -> Link {
        match node {
            Some(mut node_value) => {
                match node_value.key.cmp(&key) {
                    Ordering::Equal => {
                        let mut result = AvlTree::delete_node(node_value);
                        match result {
                            Some(mut node) => {
                                AvlTree::update_node(&mut node);
                                node = AvlTree::balance_node(node);
                                return Some(node);
                            }
                        }
                        result //if we return here, the result is none 
                    },
                    Ordering::Less => {
                        node_value.left = AvlTree::_delete(node_value.left, key);
                        Some(node_value)
                    },
                    Ordering::Greater => {
                        node_value.right = AvlTree::_delete(node_value.right, key);
                        Some(node_value)
                    }
                }
            },
            None => {
                None
            }
        }
    }

    pub fn delete_node(node: Box<Node>) -> Link {
        let n_childrens = AvlTree::n_childrens(&node);

        match n_childrens {
            0 => return None,
            1 => {
                if node.left.is_some(){
                    return node.left;
                }else{
                    return node.right;
                }
            },
            2 => {
                //search the inorder predecesor and swap with it 
                let mut node = node;
                return AvlTree::delete_node_with_childrens(node);
            },
            _ => { unreachable!() }
        }

        None
    }

    fn delete_node_with_childrens(mut node: Box<Node>) -> Link {
        //search the inorder predecesor of this node 

        let mut startNode = node.right;
        while let Some(actual_node) = startNode {
            match actual_node.left.is_some() {
                true => startNode = actual_node.left,
                false => break
            }
        }

        node.value = startNode.as_ref().unwrap().value;

        Some(node)
    }

    pub fn compare_link(node: Link, key: i32) -> bool{
        node.as_ref().unwrap().key == key
    }

    pub fn compare_node(node: &Box<Node>, key: i32) -> bool {
        //returns true if the node has the same key
        node.key == key
    }

    //returns the number of childrens in the first level of a node
    pub fn n_childrens(node: &Box<Node>) -> usize {
        let mut result = 0;
        if node.left.is_some(){result += 1;}
        if node.right.is_some(){result += 1;}

        result
    }

    pub fn has_childrens(node: &Box<Node>) -> bool {
        node.left.is_some() && node.right.is_some()
    }

    //balance a node after insertion 
    pub fn balance_node(node: Box<Node>) -> Box<Node>{
        if node.balance_factor == 2{
            //more height in the left
            return AvlTree::rotate_right(node);
        }else if node.balance_factor == -2{
            //more height in the right 
            return AvlTree::rotate_left(node);
        }
        //no balance needed
        node
    }

    //perform a right rotation 
    fn rotate_right(mut node: Box<Node>) -> Box<Node> {
        println!("Performing right rotation");
        let mut left = node.left.unwrap();
        let mut left_right = left.right;
        node.left = left_right;
        //update the node (new right child of left)
        AvlTree::update_node(&mut node);
        left.right = Some(node);

        //update the new root (left)
        AvlTree::update_node(&mut left);
        left
    }

    fn rotate_left(mut node: Box<Node>) -> Box<Node> {
        println!("Performing left rotation");
        let mut right = node.right.unwrap();
        let mut right_left = right.left;
        node.right = right_left;
        //update the node (new left child)
        AvlTree::update_node(&mut node);
        right.left = Some(node);

        //update the new root (right) 
        AvlTree::update_node(&mut right);
        right
    }

    fn rotate_left_right(node: Box<Node>) -> Box<Node> {
        AvlTree::rotate_right(AvlTree::rotate_left(node))
    }

    fn rotate_right_left(node: Box<Node>) -> Box<Node> {
        AvlTree::rotate_left(AvlTree::rotate_right(node))
    }

    //update the height of a node after insertion 
    pub fn update_node(node: &mut Box<Node>) {
        let left_height = AvlTree::get_height(&node.left);
        let right_height = AvlTree::get_height(&node.right);
        node.balance_factor = left_height - right_height;
        node.height = i32::max(
            left_height,
            right_height,
        );
    }

    //return the height of a link
    pub fn get_height(node_link: &Link) -> i32 {
        if node_link.is_some(){
            node_link.as_ref().unwrap().height + 1
        }else{
            0
        }
    }

    //prints the tree into the console (in a nice way)
    pub fn print_tree(&self) {
        if self.size > 0{
            AvlTree::_print_node(vec![&self.root], 0, self.size as usize);
        }
    }

    //prints a level of the tree in the console
    fn _print_node(nodes: Vec<&Link>, depth: usize, total_nodes: usize) {
        let mut next_level: Vec<&Link> = Vec::new();
        let mut nnodes = 0;
        for node in nodes {
            if node.is_some(){
                print!(" {:?} ", (node.as_ref().unwrap().balance_factor));
                next_level.push(&node.as_ref().unwrap().left);
                next_level.push(&node.as_ref().unwrap().right);
                nnodes += 1;
            }else{
                print!(" N ");
                next_level.push(&None);
                next_level.push(&None);
            }
        }

        if next_level.len() > 0 && nnodes > 0{
            print!("\n");
            AvlTree::_print_node(next_level, depth+1, total_nodes);
        }
    }
}

#[test]
fn test_insert_1000(){
    let mut avl_tree = AvlTree::new();
    for i in 0..1000{
        avl_tree.insert(i,i);
    }

    assert_eq!(1000, avl_tree.size);
}

#[test]
fn test_insert_1000_non_unique(){
    let mut avl_tree = AvlTree::new();
    for i in 0..1000{
        avl_tree.insert(i,i);
    }

    for i in 0..1000{
        avl_tree.insert(i,i);
    }

    assert_eq!(1000, avl_tree.size);
}