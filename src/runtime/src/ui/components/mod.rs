use super::{ViewNode, ViewNodeKind};

pub fn vstack(id: u64) -> ViewNode {
    ViewNode::new(id, ViewNodeKind::VStack)
}

pub fn hstack(id: u64) -> ViewNode {
    ViewNode::new(id, ViewNodeKind::HStack)
}

pub fn zstack(id: u64) -> ViewNode {
    ViewNode::new(id, ViewNodeKind::Custom("ZStack".into()))
}

pub fn padding(id: u64) -> ViewNode {
    ViewNode::new(id, ViewNodeKind::Custom("Padding".into()))
}

pub fn button(id: u64) -> ViewNode {
    ViewNode::new(id, ViewNodeKind::Button)
}

pub fn text_input(id: u64) -> ViewNode {
    ViewNode::new(id, ViewNodeKind::Custom("TextInput".into()))
}

pub fn slider(id: u64) -> ViewNode {
    ViewNode::new(id, ViewNodeKind::Custom("Slider".into()))
}

pub fn toggle(id: u64) -> ViewNode {
    ViewNode::new(id, ViewNodeKind::Custom("Toggle".into()))
}

pub fn text(id: u64) -> ViewNode {
    ViewNode::new(id, ViewNodeKind::Text)
}

pub fn image(id: u64) -> ViewNode {
    ViewNode::new(id, ViewNodeKind::Image)
}

