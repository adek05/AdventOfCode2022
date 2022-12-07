use crate::utils::read_lines;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Tree {
    Directory(String, Vec<Tree>),
    File(String, u64),
}

/// ```
/// use aoc::day7::{subtree_sums, Tree};
/// assert_eq!(
///   subtree_sums(
///     &Tree::Directory("root".to_owned(), vec![Tree::Directory("a".to_owned(), vec![]), Tree::Directory("b".to_owned(), vec![Tree::File("file".to_owned(), 32)]), ]),
///   ),
///   vec![]
/// );
/// ```
pub fn subtree_sums(filesystem: &Tree) -> Vec<(String, u64)> {
    let mut results = vec![];
    subtree_sums_impl(filesystem, &mut results, "".to_owned());
    results
}

pub fn subtree_sums_impl(filesystem: &Tree, sizes: &mut Vec<(String, u64)>, path: String) -> u64 {
    match filesystem {
        Tree::Directory(dir_name, trees) => {
            let res: u64 = trees
                .iter()
                .map(|elem| subtree_sums_impl(elem, sizes, path.clone() + "/" + dir_name))
                .sum();
            sizes.push((path + "/" + dir_name, res));
            res
        }
        Tree::File(_, size) => *size,
    }
}

pub fn build_tree(graph: &HashMap<String, Vec<(u64, String)>>) -> Tree {
    build_tree_impl("root".to_owned(), graph)
}

pub fn build_tree_impl(node: String, graph: &HashMap<String, Vec<(u64, String)>>) -> Tree {
    let nodes = &graph[&node];
    return Tree::Directory(
        node.clone(),
        nodes
            .iter()
            .map(|child| {
                if child.0 != 0 {
                    Tree::File(child.1.clone(), child.0)
                } else {
                    build_tree_impl(node.clone() + "/" + &child.1, graph)
                }
            })
            .collect(),
    );
}

pub fn run() {
    let lines = read_lines("in/day7.in").unwrap();

    let mut paths: HashMap<String, Vec<(u64, String)>> = HashMap::new();

    let mut cur_path = vec![];
    let mut cur_ls = vec![];
    for l in lines {
        let line = l.unwrap();
        scan!(&line;
            ("$ cd ", ..cddir) => {
                if !cur_ls.is_empty() {
                    paths.insert(cur_path.join("/"), cur_ls);
                    cur_ls = vec![];
                }
                if cddir == ".." {
                    cur_path.pop();
                } else if cddir == "/" {
                    cur_path = vec!["root".to_owned()];
                } else {
                    cur_path.push(cddir.to_string());
                }
            },
            ("$ ls") => {
            },
            ("dir ", ..dir_name) => {
                cur_ls.push((0, dir_name.to_owned()));
            },
            (let size: u64, ..file_name) => {
                cur_ls.push((size, file_name.to_owned()));
            },
        )
        .unwrap();
    }
    if !cur_ls.is_empty() {
        paths.insert(cur_path.join("/"), cur_ls);
    }

    let tree = build_tree(&paths);
    let sub_sums: u64 = subtree_sums(&tree)
        .iter()
        .map(|elem| elem.1)
        .filter(|elem| elem <= &100000)
        .sum();
    println!("Day 7, part 1: {}", sub_sums);

    let current_used_space = subtree_sums(&tree)
        .into_iter()
        .map(|elem| elem.1)
        .max()
        .unwrap();

    let to_delete = 30000000 - (70000000 - current_used_space);

    let dir_size_to_delete = subtree_sums(&tree)
        .into_iter()
        .map(|elem| elem.1)
        .filter(|elem| elem >= &to_delete)
        .min()
        .unwrap();
    println!("Day 7, part 2: {:?}", &dir_size_to_delete);
}
