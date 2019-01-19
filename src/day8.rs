use std::str::FromStr;

#[derive(Debug)]
pub struct Node {
    childs: Vec<Node>,
    metadata: Vec<u64>,
}

#[derive(Debug)]
pub struct Tree {
    root: Node,
}

impl Tree {
    fn node_metadata_sum(n: &Node) -> u64 {
        n.metadata.iter().sum::<u64>() + n.childs.iter().map(Tree::node_metadata_sum).sum::<u64>()
    }

    fn metadata_sum(&self) -> u64 {
        Tree::node_metadata_sum(&self.root)
    }

    fn node_value(n: &Node) -> u64 {
        if n.childs.len() == 0 {
            n.metadata.iter().sum::<u64>()
        } else {
            n.metadata
                .iter()
                .map(|&m| match n.childs.get(m as usize - 1) {
                    Some(child) => Tree::node_value(child),
                    None => 0,
                })
                .sum()
        }
    }

    fn root_value(&self) -> u64 {
        Tree::node_value(&self.root)
    }
}

// Return parsed nodes and where parsing ended in data.
fn parse_node(data: &[u64], idx: usize, n_siblings: u64) -> (Vec<Node>, usize) {
    let n_child = data[idx];
    let n_metadata = data[idx + 1] as usize;

    let (childs, metadata_start) = {
        if n_child == 0 {
            (vec![], idx + 2)
        } else {
            parse_node(data, idx + 2, n_child)
        }
    };

    let (mut sibling_nodes, sibling_metadata_start) = {
        if n_siblings <= 1 {
            (vec![], metadata_start + n_metadata)
        } else {
            parse_node(data, metadata_start + n_metadata, n_siblings - 1)
        }
    };

    sibling_nodes.insert(
        0,
        Node {
            childs: childs,
            metadata: data[metadata_start..(metadata_start + n_metadata)].to_vec(),
        },
    );

    (sibling_nodes, sibling_metadata_start)
}

impl FromStr for Tree {
    type Err = Box<::std::error::Error>;

    // input: "1, 2"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<u64> = s.split(' ').map(|s| s.trim().parse().unwrap()).collect();
        let mut parsed = parse_node(&data, 0, 0).0;
        assert!(parsed.len() == 1);

        Ok(Tree {
            root: parsed.pop().unwrap(),
        })
    }
}

// Solution: 47464
pub fn solve1(mut input: Vec<Tree>) -> u64 {
    assert!(input.len() == 1);
    let tree = input.pop().unwrap();
    tree.metadata_sum()
}

// Solution: 23054
pub fn solve2(mut input: Vec<Tree>) -> u64 {
    assert!(input.len() == 1);
    let tree = input.pop().unwrap();
    tree.root_value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = ["2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"]
            .iter()
            .map(|&s| String::from(s).parse::<Tree>().unwrap())
            .collect();
        println!("{:?}", input);
        assert_eq!(solve1(input), 138);
    }

    #[test]
    fn test2() {
        let input = ["2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"]
            .iter()
            .map(|&s| String::from(s).parse::<Tree>().unwrap())
            .collect();
        println!("{:?}", input);
        assert_eq!(solve2(input), 66);
    }
}
