//! Integration tests for a2ui-components

use a2ui_components::*;

#[test]
fn test_text_block_renders_all_targets() {
    let tb = TextBlock::from_text("Hello");
    assert!(tb.render(RenderTarget::Html).contains("Hello"));
    assert!(tb.render(RenderTarget::Ansi).contains("Hello"));
    assert!(tb.render(RenderTarget::Markdown).contains("Hello"));
    assert!(tb.render(RenderTarget::Voice).contains("Hello"));
}

#[test]
fn test_card_html_structure() {
    let card = Card::from_text("Title|Body");
    let html = card.render(RenderTarget::Html);
    assert!(html.contains("card-title"));
    assert!(html.contains("card-body"));
}

#[test]
fn test_card_markdown_format() {
    let card = Card::from_text("Stats|42 items");
    let md = card.render(RenderTarget::Markdown);
    assert!(md.starts_with("### Stats"));
    assert!(md.contains("42 items"));
}

#[test]
fn test_card_voice_description() {
    let card = Card::from_text("Alert|High temperature");
    let voice = card.render(RenderTarget::Voice);
    assert!(voice.contains("Alert"));
    assert!(voice.contains("High temperature"));
}

#[test]
fn test_table_parsing_and_rendering() {
    let t = Table::from_text("Name|Score\nAlice|95\nBob|87");
    assert_eq!(t.headers, vec!["Name", "Score"]);
    assert_eq!(t.rows.len(), 2);

    let md = t.render(RenderTarget::Markdown);
    assert!(md.contains("| Name | Score |"));
    assert!(md.contains("Alice"));
}

#[test]
fn test_chart_bar_rendering() {
    let chart = Chart::from_text("Revenue|Jan:100|Feb:200|Mar:150");
    assert_eq!(chart.bars.len(), 3);
    let ansi = chart.render(RenderTarget::Ansi);
    assert!(ansi.contains("Jan"));
    assert!(ansi.contains("█"));
}

#[test]
fn test_progress_bar_halfway() {
    let pb = ProgressBar::from_text("Upload|50");
    let ansi = pb.render(RenderTarget::Ansi);
    assert!(ansi.contains("50%"));
    assert!(ansi.contains("█"));
    assert!(ansi.contains("░"));
}

#[test]
fn test_status_badge_all_colors() {
    for (desc, expected) in [
        ("OK|green", BadgeColor::Green),
        ("Warn|yellow", BadgeColor::Yellow),
        ("Err|red", BadgeColor::Red),
        ("Info|blue", BadgeColor::Blue),
        ("Unknown|gray", BadgeColor::Gray),
        ("Default|other", BadgeColor::Gray),
    ] {
        let badge = StatusBadge::from_text(desc);
        assert_eq!(badge.color, expected, "Failed for desc: {}", desc);
    }
}

#[test]
fn test_action_button_html() {
    let btn = ActionButton::from_text("Click|/api/go");
    let html = btn.render(RenderTarget::Html);
    assert!(html.contains("data-action=\"/api/go\""));
    assert!(html.contains("Click"));
}

#[test]
fn test_tile_grid() {
    let tg = TileGrid::from_text("A|B|C");
    assert_eq!(tg.tiles, vec!["A", "B", "C"]);
    let md = tg.render(RenderTarget::Markdown);
    assert!(md.contains("A"));
    assert!(md.contains("C"));
}

#[test]
fn test_theme_application() {
    let dark = ComponentTheme::dark();
    assert_eq!(dark.name, "dark");

    let light = ComponentTheme::light();
    assert_eq!(light.name, "light");
}

#[test]
fn test_layout_vertical() {
    let layout = ComponentLayout::Vertical;
    let items: Vec<Box<dyn Renderable>> = vec![
        Box::new(TextBlock::from_text("A")),
        Box::new(TextBlock::from_text("B")),
    ];
    let rendered = layout.render_items(&items, RenderTarget::Markdown);
    assert!(rendered.contains("A"));
    assert!(rendered.contains("B"));
    assert!(rendered.contains("\n"));
}

#[test]
fn test_layout_horizontal() {
    let layout = ComponentLayout::Horizontal;
    let items: Vec<Box<dyn Renderable>> = vec![
        Box::new(TextBlock::from_text("X")),
        Box::new(TextBlock::from_text("Y")),
    ];
    let rendered = layout.render_items(&items, RenderTarget::Markdown);
    assert!(rendered.contains("X"));
    assert!(rendered.contains("Y"));
}

#[test]
fn test_empty_chart_returns_empty() {
    let chart = Chart::from_text("Empty");
    assert!(chart.bars.is_empty());
    let result = chart.render(RenderTarget::Ansi);
    assert!(result.is_empty());
}
