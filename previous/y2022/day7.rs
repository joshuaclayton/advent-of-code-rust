use crate::parser::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::alpha1,
    combinator::{all_consuming, map, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use slab_tree::{
    tree::{Tree, TreeBuilder},
    NodeId,
};

pub fn solve() {
    let input = include_str!("input-day7");
    println!("Answer: {:?}", run(input));
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Entry {
    File(String, usize),
    Directory(String),
}

#[derive(Debug, Clone, PartialEq)]
enum Line {
    File(String, usize),
    Directory(String),
    ListDirectory,
    ChangeDirectoryToRoot,
    ChangeDirectoryTo(String),
    ChangeDirectoryUp,
}

fn run(input: &str) -> Option<usize> {
    let (_, lines) = all_consuming(separated_list1(tag("\n"), parse_line))(input.trim()).ok()?;

    let graph = build_fs(lines);
    let mut scorer: Box<dyn EntryTreeVisitor> = Box::new(Scorer::default());
    scorer.visit_root(size(&graph.root_id().unwrap(), &graph));

    visit_node(&graph.root_id().unwrap(), &graph, &mut scorer);

    Some(scorer.score())
}

trait EntryTreeVisitor {
    fn visit_root(&mut self, size: usize);
    fn visit_file(&mut self, size: usize);
    fn visit_directory(&mut self, size: usize);
    fn score(&self) -> usize;
}

struct Scorer {
    qualifying_scores: Vec<usize>,
    unused_space: usize,
}

impl Default for Scorer {
    fn default() -> Self {
        Self {
            qualifying_scores: vec![],
            unused_space: 70000000,
        }
    }
}

impl EntryTreeVisitor for Scorer {
    fn visit_root(&mut self, size: usize) {
        self.unused_space -= size;
    }

    fn visit_file(&mut self, _: usize) {}
    fn visit_directory(&mut self, size: usize) {
        if size >= 30000000 - self.unused_space {
            self.qualifying_scores.push(size);
        }
    }

    fn score(&self) -> usize {
        *self.qualifying_scores.iter().min().unwrap()
    }
}

fn visit_node(node_id: &NodeId, tree: &Tree<Entry>, visitor: &mut Box<dyn EntryTreeVisitor>) {
    for child in tree.get(node_id.clone()).expect("tree").children() {
        match child.data() {
            Entry::File(_, size) => visitor.visit_file(*size),
            Entry::Directory(_) => {
                visit_node(&child.node_id(), tree, visitor);
                visitor.visit_directory(size(&child.node_id(), tree));
            }
        }
    }
}

fn size(node_id: &NodeId, tree: &Tree<Entry>) -> usize {
    let mut total = 0;
    for child in tree.get(node_id.clone()).expect("tree").children() {
        match child.data() {
            Entry::File(_, size) => total += size,
            Entry::Directory(_) => total += size(&child.node_id(), tree),
        }
    }

    total
}

fn build_fs(lines: Vec<Line>) -> Tree<Entry> {
    let mut fs = TreeBuilder::new()
        .with_root(Entry::Directory("/".into()))
        .build();

    let mut current_dir: Option<_> = None;

    for line in lines {
        match (line, current_dir) {
            (Line::ChangeDirectoryToRoot, _) => current_dir = fs.root_id(),
            (Line::ChangeDirectoryUp, Some(dir)) => {
                current_dir = fs.get(dir).and_then(|v| v.parent().map(|x| x.node_id()));
            }
            (Line::File(name, size), Some(dir)) => {
                fs.get_mut(dir)
                    .expect("found dir")
                    .append(Entry::File(name.clone(), size));
            }
            (Line::Directory(name), Some(dir)) => {
                fs.get_mut(dir)
                    .expect("found dir")
                    .append(Entry::Directory(name.clone()));
            }
            (Line::ListDirectory, _) => (),
            (Line::ChangeDirectoryTo(child_dir), Some(dir)) => {
                for child in fs.get(dir).unwrap().children() {
                    if child.data() == &Entry::Directory(child_dir.to_string()) {
                        current_dir = Some(child.node_id())
                    }
                }
            }
            _ => (),
        }
    }

    fs
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((
        value(Line::ChangeDirectoryToRoot, tag("$ cd /")),
        value(Line::ChangeDirectoryUp, tag("$ cd ..")),
        value(Line::ListDirectory, tag("$ ls")),
        map(preceded(tag("$ cd "), alpha1), |v: &str| {
            Line::ChangeDirectoryTo(v.to_string())
        }),
        map(preceded(tag("dir "), alpha1), |v: &str| {
            Line::Directory(v.to_string())
        }),
        map(
            separated_pair(parse_usize, tag(" "), take_till(|c| c == '\n')),
            |(filesize, v)| Line::File(v.to_string(), filesize),
        ),
    ))(input)
}

mod tests {
    #[test]
    fn solve_returns_the_correct_value() {
        let input = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
        "#;
        assert_eq!(super::run(input.trim()), Some(24933642))
    }
}
