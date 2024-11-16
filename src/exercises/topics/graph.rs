use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

type NodeRef<T> = Rc<Node<T>>;

struct Node<T>{
    value: T,
    adj: Vec<NodeRef<T>>,
}

impl<T: Hash> Hash for Node<T>{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl<T: PartialEq> PartialEq for Node<T>{
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<T: Eq> Eq for Node<T>{}

impl<T> Node<T> {

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn new(value: T, adj: Vec<NodeRef<T>>) -> Self {
        Self { value, adj }
    }
}

impl<T: Debug> Debug for Node<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[value: {:?}, adjacents: \"{:?}\"]",
               self.value,
               self.adj.iter().map(|x| { &x.value }).collect::<Vec<_>>())
    }
}

struct Graph<T>{
    nodes: Vec<NodeRef<T>>,
}

impl<T> Graph<T> {
    pub fn new(nodes: Vec<NodeRef<T>>) -> Self {
        Self { nodes }
    }
}
impl<T: Hash + PartialEq + Eq> Graph<T>{
    pub fn dfs(&self, node: NodeRef<T>) -> Vec<NodeRef<T>> {
        let mut visited = HashMap::new();
        let mut history = vec!();
        let mut queue = vec!();

        queue.push(node.clone());
        visited.insert(node.clone(), ());

        while let Some(node) = queue.pop() {
            history.push(node.clone());

            for next in node.adj.iter().rev() {
                if visited.insert(next.clone(), ()).is_none() {
                    queue.push(next.clone());
                }
            }
        }

        history
    }
}

pub(crate) fn test_all() {
    let n1 = Rc::new(Node::new(1, vec![]));
    let n2 = Rc::new(Node::new(2, vec![n1.clone()]));
    let n3 = Rc::new(Node::new(3, vec![]));
    let n4 = Rc::new(Node::new(4, vec![n1.clone(), n3.clone()]));
    let n5 = Rc::new(Node::new(5, vec![n2.clone(), n4.clone()]));
    let n6 = Rc::new(Node::new(6, vec![n5.clone(), n4.clone()]));
    let n7 = Rc::new(Node::new(7, vec![n2.clone(), n4.clone()]));

    let graph = Graph::new(vec![
        n1.clone(),
        n2.clone(),
        n3.clone(),
        n4.clone(),
        n5.clone(),
        n6.clone(),
        n7.clone(),
    ]);

    let mut paths: Vec<Vec<NodeRef<i32>>> = vec![];
    for n in graph.nodes.iter() {
        paths.push(graph.dfs(n.clone()))
    }

    paths.iter().for_each(|path| {
        println!("{:?}", path);
    });
}