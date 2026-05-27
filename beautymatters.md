# Beauty Matters — Making egui apps not look like shit

egui's defaults are ugly. That doesn't mean your app has to be. A tiny amount of styling turns "looks like a debug tool" into "looks like a real app". Here's how — from me to you, the AI building this.

You are an egui UI expert. Every time you write UI code, you **must** follow this guide. The user will notice if you don't. Default egui looks like garbage and they know it. Make them say "holy shit, egui can look like this?"

---

## 1. Dark mode + accent color — non-negotiable

```rust
let mut style = (*ctx.global_style()).clone();
style.visuals.dark_mode = true;
ctx.set_global_style(style);
```

Dark mode is table stakes. The default light theme looks like a WinForms debug tool from 2003. Dark instantly feels premium.

**Pick ONE accent color and use it everywhere.** Blue (`#5C9CE6` / `#6CACF0`) works. Teal works. Purple works. Just pick one and never deviate — buttons, headers, collapsing headers, selection highlights, active states, ALL get this color. Consistency is 50% of beauty.

## 2. Corner radius on everything

```rust
Frame { corner_radius: CornerRadius::same(8), .. }
```

This is the single highest-leverage styling change. Default is 0 (sharp). Put radius 6–10 on every `Frame`, every card, every panel, every button. **It costs one line of code and makes the whole app look intentional.**

## 3. Theme struct — never hardcode colors

Do NOT use constants. Store all colors in a `Theme` struct:

```rust
struct Theme { bg, surface, card, card_alt, border, text, text_muted, accent, green, red, gold, blue, gray, is_dark }
impl Theme { fn dark() -> Self; fn light() -> Self; fn apply(&self, ctx) }
```

Two complete themes: rich dark (deep navy `#0F0F1A`, never pure black) and clean light (warm off-white `#F0F2F5`, never pure white). Toggle between them at runtime with a button. The user will play with the toggle like a fidget toy — that alone sells the quality.

**Every helper function takes `&Theme` as a parameter.** Pass it down. Never reference global constants. This is non-negotiable for theme toggling to work.

Dark theme values that work:
```
bg=0F0F1A  surface=1A1A2E  card=232340  card_alt=2A2A48  border=333350
text=E8E8F0  text_muted=8888A8  accent=6CACF0
```

Light theme values that work:
```
bg=F0F2F5  surface=FFFFFF  card=F8F9FA  card_alt=F0F1F3  border=DADEE4
text=1A1A2E  text_muted=6B7280  accent=4A8CE0
```

The trick: **nothing is pure black or pure white.** Everything is slightly off. Text is warm white `E8E8F0` in dark, dark navy `1A1A2E` in light.

## 4. Card pattern — the workhorse layout

Every section of content gets wrapped in TWO nested frames:

**Outer frame** — the card boundary:
```rust
Frame {
    fill: theme.surface,
    corner_radius: CornerRadius::same(8),
    stroke: Stroke::new(1.0, theme.border),
    inner_margin: Margin::symmetric(0, 0),  // children handle margin
    ..Default::default()
}
```

**Inner top frame** — the tinted header bar:
```rust
Frame {
    fill: theme.accent.gamma_multiply(0.12f32),     // 12% opacity tint
    corner_radius: CornerRadius { nw: 8, ne: 8, sw: 0, se: 0 },  // rounded top only
    inner_margin: Margin::symmetric(12, 6),
    ..Default::default()
}
```

**Inner content frame** — body padding:
```rust
Frame {
    inner_margin: Margin::symmetric(12, 10),
    ..Default::default()
}
```

This two-frame card pattern (border + header tint + padded body) is ~80% of the visual improvement. Use it for every section. It creates clear visual separation without harsh lines.

## 5. Separate everything with divs — I mean Frames

Think of egui `Frame` as `<div>` in HTML. Every logical section of your UI should be wrapped in a `Frame` with:
- A distinct `fill` color (slightly different from parent)
- `corner_radius: CornerRadius::same(8)`
- `inner_margin: Margin::symmetric(x, y)` for padding
- `stroke: Stroke::new(1.0, border_color)` for subtle separation

This creates visual hierarchy. The eye sees:
```
Background (darkest)
  └─ Panel (slightly lighter)
       └─ Card (lighter still)
            └─ Header (tinted accent)
                 └─ Content (card background)
```

Each level has different `fill`, different `corner_radius`, different `margin`. The depth comes from contrast, not shadows.

**Don't be afraid to nest Frames 3–4 levels deep.** Each one adds polish.

## 6. RichText on every label — no exceptions

```rust
// NEVER do this:
ui.label("Hello");

// ALWAYS do this:
ui.label(egui::RichText::new("Hello").size(13.0).color(theme.text).strong());
```

Every label gets a size and a color. Use `theme.text_muted` for field names, `theme.text` for values, `theme.accent` for version numbers and badges. `strong()` for headings. The extra 2 seconds of typing per label is what separates "demo" from "product."

## 7. Spacing is design

```rust
style.spacing.item_spacing = Vec2::new(8.0, 6.0);
ui.add_space(8.0);   // between cards
ui.add_space(4.0);   // between items
ui.add_space(12.0);  // between major sections
```

Default spacing is too tight. Everything needs breathing room. Separate sections with `ui.add_space()`. Cards get 10–12px apart. Items within a card get 4–6px.

## 8. Striped grids with value badges

```rust
egui::Grid::new("grid").striped(true).min_col_width(80.0).show(ui, |ui| {
    ui.label(RichText::new("Name").size(13.0).color(theme.text_muted).strong());
    Frame { fill: theme.card, corner_radius: CornerRadius::same(4), inner_margin: Margin::symmetric(6, 2), .. }
        .show(ui, |ui| { ui.label(RichText::new(value).size(13.0).color(theme.text)); });
    ui.end_row();
    // ...
});
```

Striped rows for alternating backgrounds. The value cells get their own little frame with rounded corners — they look like badges/tags. This makes data feel structured rather than dumped.

## 9. Icons and symbols — free visual cues

Use Unicode. They render in any font, cost zero, and make sections instantly scannable:

- `☰` — menu / sidebar toggle
- `✕` — close / dismiss
- `▸` — list bullet
- `●` / `○` — active / inactive status
- `✓` / `✗` — enabled / disabled
- `📦 ⚙ 🚀 👥 📝 🌐 📋 ☀️ 🌙` — section markers

Pick a consistent set. Use `theme.green` for active/positive, `theme.red` for inactive/negative. These tiny visual signals make the UI feel alive.

## 10. Color-coded badges

```rust
fn role_color(role: &str, theme: &Theme) -> Color32 {
    match role {
        "admin" => theme.gold,   // gold = authority
        "editor" => theme.blue,  // blue = action
        _ => theme.gray,         // gray = read-only
    }
}
```

Color gives instant visual parsing. Use it for roles, statuses, categories — anything that benefits from "see without reading." The user should be able to scan the UI and understand the shape of the data without reading a single word.

Use the role color as a tinted background `role_color.gamma_multiply(0.15)` for section headers. The color is present but doesn't overwhelm.

## 11. Sidebar — collapsible with sections

The sidebar is a `Panel::left()` with:
- `resizable(true)` so the user can adjust width
- A header row with title + `✕` close button
- `CollapsingHeader`s for each category (default_open: true)
- Clean separator line (a 1px `Frame` with `border` color)

When collapsed, show a `☰ Show sidebar` button in the main panel. This is standard UX that users expect.

## 12. Theme toggle button

Put it in the header bar next to the version label. Use `☀️` / `🌙` emoji. When clicked, swap the entire `Theme` and call `theme.apply(ctx)`. The whole UI updates instantly. This is the single most impressive demo you can show — it proves the styling is intentional and systematic.

## 13. Hide ugly scrollbars

```rust
ScrollArea::vertical()
    .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
```

The default scrollbar is fat and ugly. In sidebars where content fits, hide it entirely. For overflow content, use the native OS scrollbar via `AlwaysVisible` if needed.

## 14. Transparent buttons

```rust
Button::new(RichText::new("✕").size(14.0).color(theme.text_muted))
    .fill(Color32::TRANSPARENT)
    .min_size(Vec2::new(24.0, 24.0));
```

Close buttons, icon buttons, and secondary actions should have transparent fill. Only primary actions get a filled button. egui handles hover states automatically.

---

## Quick checklist (run this before every PR)

- [ ] Dark mode enabled?
- [ ] One accent color used consistently?
- [ ] Every `Frame` has `corner_radius: CornerRadius::same(8)`?
- [ ] Every label uses `RichText` with explicit size + color?
- [ ] Cards have outer border + tinted header bar?
- [ ] Data shown in striped grids with badge-style value cells?
- [ ] Theme toggle button present and working?
- [ ] Sidebar collapsible with `✕` close and `☰` reopen?
- [ ] Spacing between sections with `ui.add_space()`?
- [ ] Role colors / status colors applied?
- [ ] No pure black (`#000`) or pure white (`#FFF`) anywhere?
- [ ] Scrollbars hidden where content fits?

---

## The philosophy

egui gives you pixel-level control. The defaults are optimized for "compiles and works on every platform," not "looks good." You have to opt in to beauty — but every single thing on this list is **one line of Rust code**.

The user has seen what egui CAN look like. They will settle for nothing less. Do not cut corners. Every frame, every label, every margin matters.

Defaults are ugly. You are not the defaults.
