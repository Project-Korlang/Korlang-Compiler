use std::collections::HashMap;

pub type ViewId = u64;

#[derive(Debug, Clone, PartialEq)]
pub enum ViewNodeKind {
    Text,
    Button,
    Image,
    VStack,
    HStack,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct ViewNode {
    pub id: ViewId,
    pub kind: ViewNodeKind,
    pub props: HashMap<String, String>,
    pub children: Vec<ViewNode>,
}

impl ViewNode {
    pub fn new(id: ViewId, kind: ViewNodeKind) -> Self {
        Self {
            id,
            kind,
            props: HashMap::new(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ViewTree {
    pub root: ViewNode,
}

impl ViewTree {
    pub fn new(root: ViewNode) -> Self {
        Self { root }
    }
}

#[derive(Debug, Clone)]
pub enum DiffOp {
    Replace { id: ViewId, node: ViewNode },
    UpdateProps { id: ViewId, props: HashMap<String, String> },
    InsertChild { parent: ViewId, index: usize, node: ViewNode },
    RemoveChild { parent: ViewId, index: usize },
}

pub fn diff(old: &ViewNode, new: &ViewNode, out: &mut Vec<DiffOp>) {
    if old.kind != new.kind {
        out.push(DiffOp::Replace { id: old.id, node: new.clone() });
        return;
    }
    if old.props != new.props {
        out.push(DiffOp::UpdateProps { id: old.id, props: new.props.clone() });
    }
    let max = old.children.len().max(new.children.len());
    for i in 0..max {
        match (old.children.get(i), new.children.get(i)) {
            (Some(o), Some(n)) => diff(o, n, out),
            (None, Some(n)) => out.push(DiffOp::InsertChild { parent: old.id, index: i, node: n.clone() }),
            (Some(_), None) => out.push(DiffOp::RemoveChild { parent: old.id, index: i }),
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub kind: String,
    pub target_id: String,
}

pub fn dispatch_event(root: &ViewNode, event: &Event) -> Vec<ViewId> {
    let mut path = Vec::new();
    find_path(root, &event.target_id, &mut path);
    path
}

fn find_path(node: &ViewNode, target: &str, out: &mut Vec<ViewId>) -> bool {
    if node.id.to_string() == target {
        out.push(node.id);
        return true;
    }
    for child in &node.children {
        if find_path(child, target, out) {
            out.push(node.id);
            return true;
        }
    }
    false
}

pub mod components;
pub mod style;
pub mod render;
