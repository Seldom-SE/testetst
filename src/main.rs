use chumsky::prelude::*;

struct Node(Vec<Node>);

fn main() {
    recursive(|node: Recursive<dyn Parser<&str, Node>>| {
        node.separated_by(any()).collect().map(Node)
    });
}
