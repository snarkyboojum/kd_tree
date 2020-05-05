extern crate kdtree;

use kdtree::tree::*;
// use kdtree::vec3::*;

fn main() {
    // let mut pointlist = Vec::new();
    // for _ in 0..8 {
    //     pointlist.push(Vec3::random());
    // }

    let pointlist = vec![
        [0.1, 2.8, 4.7],
        [8.1, 12.8, 1.3],
        [0.1, 2.8, 4.7],
        [8.1, 12.8, 1.3],
        [0.1, 2.8, 4.7],
        [8.1, 12.8, 1.3],
    ];
    let depth: u32 = 8;

    let tree = Node::build_balanced(&pointlist, depth);
}
