use crate::RenderTarget;
use crate::Renderable;

/// Layout arrangement for multiple components.
#[derive(Debug, Clone)]
pub enum ComponentLayout {
    Vertical,
    Horizontal,
    Grid { cols: usize },
}

impl ComponentLayout {
    /// Render a list of boxed renderables into a combined string.
    pub fn render_items(&self, items: &[Box<dyn Renderable>], target: RenderTarget) -> String {
        let rendered: Vec<String> = items.iter().map(|i| i.render(target)).collect();
        match self {
            ComponentLayout::Vertical => rendered.join("\n\n"),
            ComponentLayout::Horizontal => rendered.join("  "),
            ComponentLayout::Grid { cols } => {
                let mut s = String::new();
                for (i, r) in rendered.iter().enumerate() {
                    if i > 0 && i % cols == 0 {
                        s.push('\n');
                    } else if i > 0 {
                        s.push_str("  ");
                    }
                    s.push_str(r);
                }
                s
            }
        }
    }
}
