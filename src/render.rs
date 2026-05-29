/// Target rendering format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderTarget {
    Html,
    Ansi,
    Markdown,
    Voice,
}
