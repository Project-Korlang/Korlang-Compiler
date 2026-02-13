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

pub fn apply_diff(root: &mut ViewNode, ops: &[DiffOp]) {
    for op in ops {
        apply_one(root, op);
    }
}

fn apply_one(root: &mut ViewNode, op: &DiffOp) {
    match op {
        DiffOp::Replace { id, node } => {
            if let Some(slot) = find_node_mut(root, *id) {
                *slot = node.clone();
            }
        }
        DiffOp::UpdateProps { id, props } => {
            if let Some(slot) = find_node_mut(root, *id) {
                slot.props = props.clone();
            }
        }
        DiffOp::InsertChild { parent, index, node } => {
            if let Some(slot) = find_node_mut(root, *parent) {
                let idx = (*index).min(slot.children.len());
                slot.children.insert(idx, node.clone());
            }
        }
        DiffOp::RemoveChild { parent, index } => {
            if let Some(slot) = find_node_mut(root, *parent) {
                if *index < slot.children.len() {
                    slot.children.remove(*index);
                }
            }
        }
    }
}

fn find_node_mut(node: &mut ViewNode, id: ViewId) -> Option<&mut ViewNode> {
    if node.id == id {
        return Some(node);
    }
    for child in &mut node.children {
        if let Some(found) = find_node_mut(child, id) {
            return Some(found);
        }
    }
    None
}

pub mod components;
pub mod event_loop;
pub mod glyph;
pub mod image;
pub mod layout;
pub mod render;
pub mod style;

#[no_mangle]
pub extern "C" fn korlang_ui_demo_window() -> i64 {
    // Native-only stub for environments where a real window backend is not linked.
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn o4_6_scenegraph_diff_and_patch() {
        let mut old = ViewNode::new(1, ViewNodeKind::VStack);
        old.children.push(ViewNode::new(2, ViewNodeKind::Text));

        let mut new = ViewNode::new(1, ViewNodeKind::VStack);
        let mut text = ViewNode::new(2, ViewNodeKind::Text);
        text.props.insert("value".into(), "hello".into());
        new.children.push(text);
        new.children.push(ViewNode::new(3, ViewNodeKind::Button));

        let mut ops = Vec::new();
        diff(&old, &new, &mut ops);
        assert!(!ops.is_empty(), "expected non-empty diff ops");

        apply_diff(&mut old, &ops);
        assert_eq!(old.children.len(), 2);
        assert_eq!(old.children[0].props.get("value").map(String::as_str), Some("hello"));
        assert_eq!(old.children[1].id, 3);
    }
}
