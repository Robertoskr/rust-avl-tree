mod avltree;
use rand::Rng;

fn main() {
    let mut avl_tree = avltree::AvlTree::new();
    for i in (0..31).rev() {
        avl_tree.insert(rand::thread_rng().gen_range(0..100), i);
    }
    avl_tree.print_tree();
}
