# Beauty Matters — Making egui apps not look like shit

egui's defaults are ugly. That doesn't mean your app has to be. A tiny amount of styling turns "looks like a debug tool" into "looks like a real app". Here's how.

## The single biggest thing: dark mode + accent color

```rust
// In CreationContext::new(cc):
let mut style = (*cc.egui_ctx.global_style()).clone();
style.visuals.dark_mode = true;
cc.egui_ctx.set_global_style(style);
```

Dark mode hides a thousand sins. The default light theme looks like Windows 95. Dark instantly feels premium.

**Pick one accent color and use it everywhere.** Blue (`#5C9CE6`) works. Teal works. Purple works. Just pick ONE and be consistent — buttons, headers, selection highlights, active states.

## Corner radius is free dopamine

```rust
// eg
Frame {
    corner_radius: CornerRadius::same(8),
    ...
}
```

Default corners are sharp (radius = 0). That's the #1 thing that makes egui look like a debug UI. Give everything a radius of 6–10. Cards, panels, buttons, input fields. **It costs nothing and instantly makes it look intentional.**

## Color palette — don't use the defaults

Pick your own. The defaults are washed out.

```rust
const SURFACE: Color32 = Color32::from_rgb(0x1E, 0x1E, 0x2E);  // dark card bg
const CARD: Color32   = Color32::from_rgb(0x25, 0x25, 0x3A);    // slightly lighter
const TEXT: Color32   = Color32::from_rgb(0xE0, 0xE0, 0xE0);    // readable white
const TEXT_MUTED: Color32 = Color32::from_rgb(0x88, 0x88, 0x99); // secondary text
const BORDER: Color32 = Color32::from_rgb(0x3A, 0x3A, 0x50);    // subtle borders
const ACCENT: Color32 = Color32::from_rgb(0x5C, 0x9C, 0xE6);    // your brand color
```

The trick: **nothing is pure black or pure white.** Everything is slightly off. Surface is dark navy, not `#000`. Text is warm white, not `#FFF`.

## Cards with border + fill = magic

Don't just dump content into panels. Wrap sections in `Frame` with a background fill and a subtle border:

```rust
Frame {
    fill: SURFACE,
    corner_radius: CornerRadius::same(8),
    stroke: Stroke::new(1.0, BORDER),
    inner_margin: Margin::symmetric(14, 10),
    ..
}
```

This one pattern — dark card with rounded corners and a hairline border — is 80% of the visual improvement. Use it everywhere.

## Section headers with tinted background

```rust
Frame {
    fill: accent.gamma_multiply(0.12f32),  // tinted, not solid
    corner_radius: CornerRadius { nw: 8, ne: 8, sw: 0, se: 0 },
    inner_margin: Margin::symmetric(12, 6),
    ..
}
```

Top corners rounded, bottom corners sharp — it feels like a real UI component. The 12% opacity tint gives color without overwhelming.

## Use RichText. Always.

```rust
// Don't:
ui.label("Hello");
// Do:
ui.label(egui::RichText::new("Hello").size(13.0).color(TEXT).strong());
```

Every label gets a color and size. `TEXT_MUTED` for labels, `TEXT` for values. This creates visual hierarchy. Headings get `strong()` and a larger size. Versions get `color(ACCENT)`.

## Spacing is design

```rust
style.spacing.item_spacing = Vec2::new(8.0, 6.0);
```

Default spacing is too tight. Give things room to breathe. Use `ui.add_space()` between sections. 8–12px between cards, 4–6px between items.

## Striped grids for data

```rust
egui::Grid::new("grid").striped(true).show(ui, |ui| { ... });
```

Alternating row colors make tables readable. Pair with a subtle background frame around the value cells.

## Icons and symbols

Use Unicode symbols as lightweight icons — they render in any font and cost nothing:
- `☰` — hamburger menu
- `✕` — close
- `▸` — bullet
- `●` / `○` — status indicators
- `✓` / `✗` — check/cross
- `📦`, `⚙`, `🚀`, `👥`, `📝`, `🌐`, `📋` — section icons

Pick a consistent set and use them as section markers.

## Role-based color coding

```rust
fn role_color(role: &str) -> Color32 {
    match role {
        "admin" => GOLD,
        "editor" => BLUE,
        _ => GRAY,
    }
}
```

Color gives instant visual parsing. Admin = gold, editor = blue, viewer = gray. Users scan and understand without reading.

## Scroll areas: hide the scrollbar

```rust
egui::ScrollArea::vertical()
    .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
    .show(ui, |ui| { ... });
```

The default scrollbar is ugly and steals width. Hide it when the content fits, or use the `AlwaysHidden` + `scroll_offset` approach if you know your content is shorter than the viewport. On web, the native scrollbar is fine — use `AlwaysVisible` for web content that overflows.

## Buttons: transparent fill, hover only

```rust
egui::Button::new(egui::RichText::new("✕").size(14.0).color(TEXT_MUTED))
    .fill(Color32::TRANSPARENT)
    .min_size(Vec2::new(24.0, 24.0));
```

Buttons don't need a background. Transparent fill + visible text = clean. The hover state is handled by egui's default interaction.

## The CollapsingHeader is your friend

Collapsible sections keep the UI clean. Default-open for the first section, collapsed for details. Users can expand what they care about.

```rust
egui::CollapsingHeader::new(
    egui::RichText::new(title).size(11.0).color(color).strong(),
)
.default_open(true)
.show(ui, |ui| { ... });
```

## Recap: quick checklist

| What | Why |
|---|---|
| `dark_mode = true` | instantly premium |
| `CornerRadius::same(8)` | the #1 "looks intentional" trick |
| Custom color palette | no washed-out defaults |
| Cards with `fill` + `stroke` | visual structure |
| `RichText` on every label | hierarchy and readability |
| Margin and spacing | breathing room |
| Unicode icons | free visual cues |
| Color-coded badges | instant parsing |
| Striped grids | readable tables |
| Transparent buttons | clean interaction |

## The philosophy

egui gives you pixel-level control. The defaults are optimized for "works everywhere", not "looks good". You have to opt in to beauty — but every single thing on this list is **one line of code**. There's no excuse for ugly.
