use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Style {
    pub props: HashMap<String, String>,
}

impl Style {
    pub fn set(mut self, key: &str, value: impl ToString) -> Self {
        self.props.insert(key.to_string(), value.to_string());
        self
    }

    pub fn merge(&mut self, other: &Style) {
        for (k, v) in &other.props {
            self.props.insert(k.clone(), v.clone());
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Theme {
    pub vars: HashMap<String, String>,
}

impl Theme {
    pub fn set(mut self, key: &str, value: impl ToString) -> Self {
        self.vars.insert(key.to_string(), value.to_string());
        self
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.vars.get(key)
    }
}

pub trait StylePipe {
    fn apply(&self, style: &mut Style);
}

pub struct FontSize(pub i32);
impl StylePipe for FontSize {
    fn apply(&self, style: &mut Style) {
        style.props.insert("fontSize".into(), self.0.to_string());
    }
}

pub struct Color(pub &'static str);
impl StylePipe for Color {
    fn apply(&self, style: &mut Style) {
        style.props.insert("color".into(), self.0.to_string());
    }
}

pub struct Padding(pub i32);
impl StylePipe for Padding {
    fn apply(&self, style: &mut Style) {
        style.props.insert("padding".into(), self.0.to_string());
    }
}

