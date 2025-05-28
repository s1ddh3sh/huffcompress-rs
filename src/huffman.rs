use Tree::*;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree<T> {
    Leaf {
        freq: u64,
        token: T,
    },
    Node {
        freq: u64,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

#[allow(dead_code)]
impl<T: Clone> Tree<T> {
    pub fn freq(&self) -> u64 {
        match self {
            Leaf { freq, .. } => *freq,
            Node { freq, .. } => *freq,
        }
    }

    pub fn token(&self) -> Option<T> {
        match self {
            Leaf { token, .. } => Some(token.clone()),
            Node { .. } => None,
        }
    }
    pub fn left(&self) -> Option<&Tree<T>> {
        match self {
            Node { left, .. } => Some(left),
            Leaf { .. } => None,
        }
    }
    pub fn right(&self) -> Option<&Tree<T>> {
        match self {
            Node { right, .. } => Some(right),
            Leaf { .. } => None,
        }
    }
}

impl<T: Clone + Eq> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq().cmp(&other.freq())
    }
}

impl<T: Clone + Eq> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn huffman_tree<T: Eq + Clone>(freqs: &HashMap<T, u64>) -> Tree<T> {
    let mut heap = BinaryHeap::new();
    for (token, freq) in freqs {
        let (freq, token) = (*freq, token.clone());
        heap.push(Reverse(Leaf { freq, token }));
    }
    while heap.len() > 1 {
        let node1 = heap.pop().unwrap().0;
        let node2 = heap.pop().unwrap().0;

        let merged_node = Node {
            freq: node1.freq() + node2.freq(),
            left: Box::new(node1),
            right: Box::new(node2),
        };
        heap.push(Reverse(merged_node));
    }
    heap.pop().unwrap().0
}
