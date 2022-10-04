use anyhow::anyhow;

pub type NodeId = usize;

#[derive(Debug)]
pub struct Node<T> {
    pub idx: NodeId,
    pub val: T,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
}

impl<T> Node<T> {
    pub fn new(idx: NodeId, val: T) -> Self {
        Node {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

pub struct Arena<T> {
    pub nodes: Vec<Node<T>>,
}

impl<T> Arena<T> {
    pub fn new(val: T) -> Self {
        let root = Node::new(0, val);
        Arena { nodes: vec![root] }
    }

    pub fn root_id(&self) -> NodeId {
        0
    }

    pub fn get_node(&self, id: NodeId) -> anyhow::Result<&Node<T>> {
        self.nodes.get(id).ok_or(anyhow!("Invalid node {}", id))
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> anyhow::Result<&mut Node<T>> {
        self.nodes.get_mut(id).ok_or(anyhow!("Invalid node {}", id))
    }

    pub fn new_node(&mut self, val: T, parent_id: NodeId) -> anyhow::Result<NodeId> {
        let next_id = self.nodes.len();
        if parent_id >= next_id {
            return Err(anyhow!(
                "Invalid parent_id {}, current tree size is {}",
                parent_id,
                next_id
            ));
        }
        let parent = self.nodes.get_mut(parent_id).unwrap();
        let mut node = Node::new(next_id, val);
        parent.children.push(next_id);
        node.parent = Some(parent_id);
        self.nodes.push(node);
        Ok(next_id)
    }
}
