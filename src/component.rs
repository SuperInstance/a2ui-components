use crate::{ComponentTheme, RenderTarget};

/// Color for a status badge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BadgeColor {
    Green,
    Yellow,
    Red,
    Blue,
    Gray,
}

/// A simple text block.
#[derive(Debug, Clone)]
pub struct TextBlock {
    pub text: String,
    pub theme: Option<ComponentTheme>,
}

impl TextBlock {
    pub fn from_text(desc: &str) -> Self {
        Self { text: desc.to_string(), theme: None }
    }
}

/// A card with title and body.
#[derive(Debug, Clone)]
pub struct Card {
    pub title: String,
    pub body: String,
    pub theme: Option<ComponentTheme>,
}

impl Card {
    pub fn from_text(desc: &str) -> Self {
        let mut parts = desc.splitn(2, '|');
        let title = parts.next().unwrap_or("").to_string();
        let body = parts.next().unwrap_or("").to_string();
        Self { title, body, theme: None }
    }
}

/// A table with headers and rows.
#[derive(Debug, Clone)]
pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub theme: Option<ComponentTheme>,
}

impl Table {
    pub fn from_text(desc: &str) -> Self {
        let lines: Vec<&str> = desc.lines().collect();
        let headers = lines.first()
            .map(|l| l.split('|').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();
        let rows = lines[1..]
            .iter()
            .map(|l| l.split('|').map(|s| s.trim().to_string()).collect())
            .collect();
        Self { headers, rows, theme: None }
    }
}

/// A named bar with values.
#[derive(Debug, Clone)]
pub struct BarEntry {
    pub label: String,
    pub value: f64,
}

/// An ASCII bar chart.
#[derive(Debug, Clone)]
pub struct Chart {
    pub title: String,
    pub bars: Vec<BarEntry>,
    pub theme: Option<ComponentTheme>,
}

impl Chart {
    pub fn from_text(desc: &str) -> Self {
        let mut parts = desc.split('|');
        let title = parts.next().unwrap_or("").to_string();
        let bars: Vec<BarEntry> = parts
            .filter_map(|p| {
                let mut kv = p.split(':');
                let label = kv.next()?.trim().to_string();
                let value: f64 = kv.next()?.trim().parse().ok()?;
                Some(BarEntry { label, value })
            })
            .collect();
        Self { title, bars, theme: None }
    }
}

/// A progress bar.
#[derive(Debug, Clone)]
pub struct ProgressBar {
    pub label: String,
    pub percentage: f64,
    pub theme: Option<ComponentTheme>,
}

impl ProgressBar {
    pub fn from_text(desc: &str) -> Self {
        let mut parts = desc.splitn(2, '|');
        let label = parts.next().unwrap_or("").to_string();
        let percentage = parts.next().and_then(|s| s.trim().parse::<f64>().ok()).unwrap_or(0.0);
        Self { label, percentage, theme: None }
    }
}

/// A status badge with color.
#[derive(Debug, Clone)]
pub struct StatusBadge {
    pub label: String,
    pub color: BadgeColor,
    pub theme: Option<ComponentTheme>,
}

impl StatusBadge {
    pub fn from_text(desc: &str) -> Self {
        let mut parts = desc.splitn(2, '|');
        let label = parts.next().unwrap_or("").to_string();
        let color = match parts.next().map(|s| s.trim().to_lowercase()).as_deref() {
            Some("green") => BadgeColor::Green,
            Some("red") => BadgeColor::Red,
            Some("yellow") => BadgeColor::Yellow,
            Some("blue") => BadgeColor::Blue,
            _ => BadgeColor::Gray,
        };
        Self { label, color, theme: None }
    }
}

/// An action button.
#[derive(Debug, Clone)]
pub struct ActionButton {
    pub label: String,
    pub action: String,
    pub theme: Option<ComponentTheme>,
}

impl ActionButton {
    pub fn from_text(desc: &str) -> Self {
        let mut parts = desc.splitn(2, '|');
        let label = parts.next().unwrap_or("").to_string();
        let action = parts.next().unwrap_or("").to_string();
        Self { label, action, theme: None }
    }
}

/// A tile grid of labeled items.
#[derive(Debug, Clone)]
pub struct TileGrid {
    pub tiles: Vec<String>,
    pub theme: Option<ComponentTheme>,
}

impl TileGrid {
    pub fn from_text(desc: &str) -> Self {
        let tiles = desc.split('|').map(|s| s.trim().to_string()).collect();
        Self { tiles, theme: None }
    }
}

/// Trait for renderable components.
pub trait Renderable {
    fn render(&self, target: RenderTarget) -> String;
    fn apply_theme(&mut self, theme: &ComponentTheme);
}

// Renderable implementations

impl Renderable for TextBlock {
    fn render(&self, target: RenderTarget) -> String {
        match target {
            RenderTarget::Html => format!("<div class=\"text-block\">{}</div>", self.text),
            RenderTarget::Ansi => self.text.clone(),
            RenderTarget::Markdown => self.text.clone(),
            RenderTarget::Voice => self.text.clone(),
        }
    }
    fn apply_theme(&mut self, theme: &ComponentTheme) { self.theme = Some(theme.clone()); }
}

impl Renderable for Card {
    fn render(&self, target: RenderTarget) -> String {
        match target {
            RenderTarget::Html => format!(
                "<div class=\"card\"><h3 class=\"card-title\">{}</h3><p class=\"card-body\">{}</p></div>",
                self.title, self.body
            ),
            RenderTarget::Ansi => format!("┌─ {} ─\n│ {}\n└──────────", self.title, self.body),
            RenderTarget::Markdown => format!("### {}\n\n{}", self.title, self.body),
            RenderTarget::Voice => format!("Card titled \"{}\". {}", self.title, self.body),
        }
    }
    fn apply_theme(&mut self, theme: &ComponentTheme) { self.theme = Some(theme.clone()); }
}

impl Renderable for Table {
    fn render(&self, target: RenderTarget) -> String {
        match target {
            RenderTarget::Html => {
                let mut s = String::from("<table class=\"table\"><thead><tr>");
                for h in &self.headers {
                    s.push_str(&format!("<th>{}</th>", h));
                }
                s.push_str("</tr></thead><tbody>");
                for row in &self.rows {
                    s.push_str("<tr>");
                    for cell in row {
                        s.push_str(&format!("<td>{}</td>", cell));
                    }
                    s.push_str("</tr>");
                }
                s.push_str("</tbody></table>");
                s
            }
            RenderTarget::Markdown => {
                let mut s = format!("| {} |\n", self.headers.join(" | "));
                s.push_str(&format!("| {} |\n", self.headers.iter().map(|_| "---").collect::<Vec<_>>().join(" | ")));
                for row in &self.rows {
                    s.push_str(&format!("| {} |\n", row.join(" | ")));
                }
                s.trim_end().to_string()
            }
            RenderTarget::Ansi => {
                let mut s = String::new();
                let col_width = 12;
                for h in &self.headers {
                    s.push_str(&format!("{:width$}", h, width = col_width));
                }
                s.push('\n');
                for _ in &self.headers { s.push_str(&"-".repeat(col_width)); }
                s.push('\n');
                for row in &self.rows {
                    for cell in row {
                        s.push_str(&format!("{:width$}", cell, width = col_width));
                    }
                    s.push('\n');
                }
                s.trim_end().to_string()
            }
            RenderTarget::Voice => {
                let mut s = format!("Table with columns: {}.", self.headers.join(", "));
                for row in &self.rows {
                    let pairs: Vec<String> = self.headers.iter().zip(row.iter())
                        .map(|(h, v)| format!("{}: {}", h, v))
                        .collect();
                    s.push_str(&format!(" {}", pairs.join(", ")));
                    s.push('.');
                }
                s
            }
        }
    }
    fn apply_theme(&mut self, theme: &ComponentTheme) { self.theme = Some(theme.clone()); }
}

impl Renderable for Chart {
    fn render(&self, target: RenderTarget) -> String {
        if self.bars.is_empty() { return String::new(); }
        let max_val = self.bars.iter().map(|b| b.value).fold(f64::MIN, f64::max).max(1.0);
        match target {
            RenderTarget::Ansi | RenderTarget::Markdown => {
                let mut s = format!("{}\n", self.title);
                for bar in &self.bars {
                    let filled = (bar.value / max_val * 20.0).round() as usize;
                    let empty = 20 - filled;
                    s.push_str(&format!("{:6} |{}{}| {:.0}\n",
                        bar.label,
                        "█".repeat(filled),
                        "░".repeat(empty),
                        bar.value
                    ));
                }
                s.trim_end().to_string()
            }
            RenderTarget::Html => {
                let mut s = format!("<div class=\"chart\"><h4>{}</h4>\n", self.title);
                for bar in &self.bars {
                    let pct = (bar.value / max_val * 100.0) as usize;
                    s.push_str(&format!(
                        "<div class=\"bar-row\"><span>{}</span><div class=\"bar\" style=\"width:{}%\">{:.0}</div></div>\n",
                        bar.label, pct, bar.value
                    ));
                }
                s.push_str("</div>");
                s
            }
            RenderTarget::Voice => {
                let bars_desc: Vec<String> = self.bars.iter()
                    .map(|b| format!("{}: {}", b.label, b.value))
                    .collect();
                format!("{} chart. {}", self.title, bars_desc.join(", "))
            }
        }
    }
    fn apply_theme(&mut self, theme: &ComponentTheme) { self.theme = Some(theme.clone()); }
}

impl Renderable for ProgressBar {
    fn render(&self, target: RenderTarget) -> String {
        let filled = (self.percentage / 100.0 * 20.0).round() as usize;
        let empty = 20 - filled;
        match target {
            RenderTarget::Ansi | RenderTarget::Markdown => {
                format!("{} [{}{}] {:.0}%", self.label,
                    "█".repeat(filled), "░".repeat(empty), self.percentage)
            }
            RenderTarget::Html => {
                format!("<div class=\"progress\"><label>{}</label><div class=\"bar\" style=\"width:{:.0}%\">{:.0}%</div></div>",
                    self.label, self.percentage, self.percentage)
            }
            RenderTarget::Voice => {
                format!("{} is {:.0} percent complete.", self.label, self.percentage)
            }
        }
    }
    fn apply_theme(&mut self, theme: &ComponentTheme) { self.theme = Some(theme.clone()); }
}

impl Renderable for StatusBadge {
    fn render(&self, target: RenderTarget) -> String {
        let color_str = match self.color {
            BadgeColor::Green => "green",
            BadgeColor::Yellow => "yellow",
            BadgeColor::Red => "red",
            BadgeColor::Blue => "blue",
            BadgeColor::Gray => "gray",
        };
        match target {
            RenderTarget::Html => {
                format!("<span class=\"badge badge-{}\">{}</span>", color_str, self.label)
            }
            RenderTarget::Ansi => {
                let code = match self.color {
                    BadgeColor::Green => "\x1b[32m",
                    BadgeColor::Yellow => "\x1b[33m",
                    BadgeColor::Red => "\x1b[31m",
                    BadgeColor::Blue => "\x1b[34m",
                    BadgeColor::Gray => "\x1b[90m",
                };
                format!("{}[{}]\x1b[0m", code, self.label)
            }
            RenderTarget::Markdown => format!("![{}](badge:{})", self.label, color_str),
            RenderTarget::Voice => format!("Status: {} ({})", self.label, color_str),
        }
    }
    fn apply_theme(&mut self, theme: &ComponentTheme) { self.theme = Some(theme.clone()); }
}

impl Renderable for ActionButton {
    fn render(&self, target: RenderTarget) -> String {
        match target {
            RenderTarget::Html => {
                format!("<button class=\"action-btn\" data-action=\"{}\">{}</button>", self.action, self.label)
            }
            RenderTarget::Ansi => format!("[{}]({})", self.label, self.action),
            RenderTarget::Markdown => format!("[{}]({})", self.label, self.action),
            RenderTarget::Voice => format!("Button: {}. Action: {}.", self.label, self.action),
        }
    }
    fn apply_theme(&mut self, theme: &ComponentTheme) { self.theme = Some(theme.clone()); }
}

impl Renderable for TileGrid {
    fn render(&self, target: RenderTarget) -> String {
        match target {
            RenderTarget::Html => {
                let tiles: Vec<String> = self.tiles.iter()
                    .map(|t| format!("<div class=\"tile\">{}</div>", t))
                    .collect();
                format!("<div class=\"tile-grid\">{}</div>", tiles.join(""))
            }
            RenderTarget::Ansi => {
                format!("{}", self.tiles.iter()
                    .map(|t| format!("[{}]", t))
                    .collect::<Vec<_>>()
                    .join(" "))
            }
            RenderTarget::Markdown => {
                format!("{}", self.tiles.iter()
                    .map(|t| format!("▌ {} ▐", t))
                    .collect::<Vec<_>>()
                    .join("  "))
            }
            RenderTarget::Voice => {
                format!("Tiles: {}.", self.tiles.join(", "))
            }
        }
    }
    fn apply_theme(&mut self, theme: &ComponentTheme) { self.theme = Some(theme.clone()); }
}
