#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    FlexRow,
    Grid { columns: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Box2D {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub fn compute_flex_row(container: Box2D, count: usize) -> Vec<Box2D> {
    if count == 0 {
        return Vec::new();
    }
    let each = container.w / (count as u32);
    (0..count)
        .map(|i| Box2D {
            x: container.x + each * (i as u32),
            y: container.y,
            w: each,
            h: container.h,
        })
        .collect()
}

pub fn compute_grid(container: Box2D, count: usize, columns: u32) -> Vec<Box2D> {
    if count == 0 || columns == 0 {
        return Vec::new();
    }
    let col_w = container.w / columns;
    let rows = (count as u32).div_ceil(columns);
    let row_h = if rows == 0 { container.h } else { container.h / rows };
    (0..count)
        .map(|i| {
            let idx = i as u32;
            let c = idx % columns;
            let r = idx / columns;
            Box2D {
                x: container.x + c * col_w,
                y: container.y + r * row_h,
                w: col_w,
                h: row_h,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn o4_8_layout_flex_and_grid() {
        let root = Box2D { x: 0, y: 0, w: 300, h: 120 };
        let flex = compute_flex_row(root, 3);
        assert_eq!(flex.len(), 3);
        assert_eq!(flex[0], Box2D { x: 0, y: 0, w: 100, h: 120 });
        assert_eq!(flex[2], Box2D { x: 200, y: 0, w: 100, h: 120 });

        let grid = compute_grid(root, 4, 2);
        assert_eq!(grid.len(), 4);
        assert_eq!(grid[0], Box2D { x: 0, y: 0, w: 150, h: 60 });
        assert_eq!(grid[3], Box2D { x: 150, y: 60, w: 150, h: 60 });
    }
}
