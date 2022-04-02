#![feature(test)]

crate avltree::AvlTree;

use avltree::AvlTree;

use test::Bencher;

extern crate test;

#[bench]
fn bench_insert_1000(){
    let mut avl_tree = AvlTree::new();
    for i in 0..1000{
        avl_tree.insert(i,i);
    }
}

#[bench]
fn bench_insert_2000(){
    let mut avl_tree = AvlTree::new();
    for i in 0..2000{
        avl_tree.insert(i,i);
    }
}

#[bench]
fn bench_insert_10000(){
    let mut avl_tree = AvlTree::new();
    for i in 0..10000{
        avl_tree.insert(i,i);
    }
}