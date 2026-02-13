use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlyphRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Default)]
pub struct GlyphAtlas {
    width: u32,
    height: u32,
    cursor_x: u32,
    cursor_y: u32,
    row_h: u32,
    slots: HashMap<char, GlyphRect>,
}

impl GlyphAtlas {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, ..Self::default() }
    }

    pub fn insert(&mut self, ch: char, w: u32, h: u32) -> Option<GlyphRect> {
        if let Some(r) = self.slots.get(&ch).copied() {
            return Some(r);
        }
        if w == 0 || h == 0 || w > self.width || h > self.height {
            return None;
        }
        if self.cursor_x + w > self.width {
            self.cursor_x = 0;
            self.cursor_y = self.cursor_y.saturating_add(self.row_h);
            self.row_h = 0;
        }
        if self.cursor_y + h > self.height {
            return None;
        }
        let rect = GlyphRect { x: self.cursor_x, y: self.cursor_y, w, h };
        self.cursor_x += w;
        self.row_h = self.row_h.max(h);
        self.slots.insert(ch, rect);
        Some(rect)
    }

    pub fn get(&self, ch: char) -> Option<GlyphRect> {
        self.slots.get(&ch).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn o4_5_glyph_atlas_generation() {
        let mut atlas = GlyphAtlas::new(32, 32);
        let a = atlas.insert('A', 8, 10).expect("insert A");
        let b = atlas.insert('B', 8, 10).expect("insert B");
        let c = atlas.insert('C', 20, 10).expect("insert C with row wrap");
        assert_ne!(a, b);
        assert_eq!(c.y, 10);
        assert_eq!(atlas.get('A'), Some(a));
        assert!(atlas.insert('Z', 64, 1).is_none());
    }
}
