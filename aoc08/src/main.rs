use std::fs;

#[derive(Debug, Default)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
    len: usize,
}

impl Node {
    fn from_flat(flat: &[u32]) -> Node {
        let (num_children, num_meta) = (flat[0], flat[1]);
        let mut node = Node {
            len: 2,
            ..Node::default()
        };
        for _ in 0..num_children {
            let child = Node::from_flat(&flat[node.len..]);
            node.len += child.len;
            node.children.push(child);
        }

        for _ in 0..num_meta {
            node.metadata.push(flat[node.len]);
            node.len += 1;
        }

        node
    }

    fn sum_all_metadata(&self) -> u32 {
        let mut sum = self.metadata.iter().sum();
        for child in &self.children {
            sum += child.sum_all_metadata();
        }

        sum
    }

    fn value(&self) -> u32 {
        if self.children.is_empty() {
            return self.metadata.iter().sum();
        } else {
            let mut value = 0;
            for index in &self.metadata {
                if index > &0 && index <= &(self.children.len() as u32) {
                    value += self.children[(index - 1) as usize].value();
                }
            }

            return value;
        }
    }
}

fn main() {
    let s = fs::read_to_string("src/input.txt").expect("Couldn't read input");
    let input: Vec<u32> = s
        .split_whitespace()
        .map(|s| s.parse().expect("Couldn't parse input"))
        .collect();
    let root = Node::from_flat(&input);

    part1(&root);
    part2(&root);
}

fn part1(root: &Node) {
    println!("{}", root.sum_all_metadata());
}

fn part2(root: &Node) {
    println!("{}", root.value());
}
