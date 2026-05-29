mod component;
mod layout;
mod render;
mod theme;

pub use component::*;
pub use layout::*;
pub use render::*;
pub use theme::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_block() {
        let tb = TextBlock::from_text("Hello, world!");
        assert_eq!(tb.text, "Hello, world!");
        let md = tb.render(RenderTarget::Markdown);
        assert!(md.contains("Hello, world!"));
    }

    #[test]
    fn test_card() {
        let card = Card::from_text("Title|Body text here");
        assert_eq!(card.title, "Title");
        assert_eq!(card.body, "Body text here");
        let html = card.render(RenderTarget::Html);
        assert!(html.contains("<div"));
        assert!(html.contains("Title"));
    }

    #[test]
    fn test_table_from_data() {
        let t = Table::from_text("Name|Age|City\nAlice|30|NYC\nBob|25|LA");
        assert_eq!(t.headers, vec!["Name", "Age", "City"]);
        assert_eq!(t.rows.len(), 2);
        let md = t.render(RenderTarget::Markdown);
        assert!(md.contains("Alice"));
        assert!(md.contains("|"));
    }

    #[test]
    fn test_progress_bar() {
        let pb = ProgressBar::from_text("Upload|75");
        assert_eq!(pb.label, "Upload");
        assert!((pb.percentage - 75.0).abs() < f64::EPSILON);
        let ansi = pb.render(RenderTarget::Ansi);
        assert!(ansi.contains("████████"));
    }

    #[test]
    fn test_badge_colors() {
        let green = StatusBadge::from_text("OK|green");
        assert_eq!(green.label, "OK");
        assert_eq!(green.color, BadgeColor::Green);

        let red = StatusBadge::from_text("Error|red");
        assert_eq!(red.color, BadgeColor::Red);

        let yellow = StatusBadge::from_text("Warning|yellow");
        assert_eq!(yellow.color, BadgeColor::Yellow);
    }

    #[test]
    fn test_action_button() {
        let btn = ActionButton::from_text("Submit|/api/submit");
        assert_eq!(btn.label, "Submit");
        assert_eq!(btn.action, "/api/submit");
        let html = btn.render(RenderTarget::Html);
        assert!(html.contains("<button"));
    }

    #[test]
    fn test_tile_grid() {
        let tg = TileGrid::from_text("A|B|C|D|E|F");
        assert_eq!(tg.tiles.len(), 6);
        let md = tg.render(RenderTarget::Markdown);
        assert!(md.contains("A"));
    }

    #[test]
    fn test_chart() {
        let chart = Chart::from_text("Sales|Q1:10|Q2:25|Q3:18|Q4:30");
        assert_eq!(chart.title, "Sales");
        assert_eq!(chart.bars.len(), 4);
        let ansi = chart.render(RenderTarget::Ansi);
        assert!(ansi.contains("Q1"));
    }

    #[test]
    fn test_html_output() {
        let card = Card::from_text("Test|Content");
        let html = card.render(RenderTarget::Html);
        assert!(html.starts_with("<div"));
        assert!(html.contains("class="));
    }

    #[test]
    fn test_ansi_output() {
        let pb = ProgressBar::from_text("Progress|50");
        let ansi = pb.render(RenderTarget::Ansi);
        assert!(ansi.contains("█"));
    }

    #[test]
    fn test_markdown_output() {
        let t = Table::from_text("A|B\n1|2");
        let md = t.render(RenderTarget::Markdown);
        assert!(md.contains("| A | B |"));
        assert!(md.contains("| --- | --- |"));
    }

    #[test]
    fn test_voice_description() {
        let chart = Chart::from_text("Revenue|Jan:100|Feb:200");
        let voice = chart.render(RenderTarget::Voice);
        assert!(voice.contains("Revenue"));
        assert!(voice.contains("chart"));
    }

    #[test]
    fn test_theme_application() {
        let theme = ComponentTheme::dark();
        let mut card = Card::from_text("Hello|World");
        card.apply_theme(&theme);
        assert!(card.theme.is_some());
    }

    #[test]
    fn test_layout_ordering() {
        let mut layout = ComponentLayout::Horizontal;
        let tb1 = TextBlock::from_text("First");
        let tb2 = TextBlock::from_text("Second");
        let rendered = layout.render_items(&[
            Box::new(tb1.clone()) as Box<dyn Renderable>,
            Box::new(tb2.clone()) as Box<dyn Renderable>,
        ], RenderTarget::Markdown);
        assert!(rendered.contains("First"));
        assert!(rendered.contains("Second"));
    }
}
