```rust
use chumsky::prelude::*;

struct Node(Vec<Node>);

fn main() {
    recursive(|node: Recursive<char, Node, Simple<char>>| node.separated_by(any()).map(Node));
}
```

```rust
use chumsky::prelude::*;

struct Node(Vec<Node>);

fn main() {
    recursive(|node: Recursive<dyn Parser<&str, Node>>| node.separated_by(any()).map(Node));
}
```