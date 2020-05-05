// See, https://en.wikipedia.org/wiki/K-d_tree

use crate::vec3::Vec3;
use std::cmp::Ordering;

// Every leaf node is a k-dimensional point
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    location: [f32; 3],
}

impl Node {
    // construct a balanced tree
    pub fn build_balanced(v: &Vec<[f32; 3]>, depth: u32) -> Option<Box<Node>> {
        println!("Building balanced kd-tree with pointlist: {:?}", v);

        if v.len() == 0 {
            println!("Leaf!");
            return None;
        }

        // check we have at least one point to determine the dimentionality
        // and assume all points have the same dimension
        assert!(v.len() > 0);
        let k = v[0].len();
        let axis = depth as usize % k;

        // sort a cloned point list by axis
        let mut sorted = v.clone();
        sorted.sort_by(|a, b| a[axis].partial_cmp(&b[axis]).unwrap_or(Ordering::Equal));
        let median = v.len() / 2;

        println!(
            "Sorted pointlist: {:?}, by axis: {}, with median: {}",
            sorted, axis, median
        );

        Some(Box::new(Node {
            location: v[median],
            left: Node::build_balanced(&v[..median].to_vec(), depth + 1),
            right: Node::build_balanced(&v[median + 1..].to_vec(), depth + 1),
        }))
    }
}

pub struct Leaf<T> {
    v: Vec<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        // Node {
        //     left: None,
        //     right: None,
        // };
    }
}
