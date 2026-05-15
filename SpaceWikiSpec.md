# Rocket Wiki TUI — Revised Fresh Spec (Complete)

## Project Overview

A terminal user interface (TUI) wiki in Rust about rockets, launch vehicles, engines, and spacecraft. Articles are stored as `.toml` files in an `articles/` directory. Built with `ratatui` + `crossterm`.

---

## Tech Stack

- **Language**: Rust, edition 2021
- **TUI**: `ratatui = "0.30"`, `crossterm = "0.29"`
- **Serialization**: `toml = "0.8"` (generic `toml::Value` tree; not using serde derive)

---

## Implementation Roadmap

### Phase 1: Core Infrastructure
- [x] Project structure and `Cargo.toml` setup
- [x] Terminal initialization and event loop
- [x] Basic TUI rendering with `ratatui`
- [x] Graceful teardown and quit functionality

### Phase 2: Article System
- [x] Define article types and data structures
- [x] TOML file loading and parsing with error handling
- [x] Article storage in a `HashMap<String, Article>`
- [x] Type-based organization (Engine, LaunchVehicle, Spacecraft)

### Phase 3: Parser & Content Processing
- [x] Markdown-like body text parser (headings with `#`/`##`/`###`)
- [x] Link syntax parser (`[[link]]` and `[[link|label]]`)
- [x] Infobox field parsing (flat fields, subtables with styling, stages)
- [x] Body text rendering into `Segment` enum (`Text` or `Link`)

### Phase 4: Sidebar Navigation
- [x] Group articles by type heading (e.g., `ENGINES`)
- [x] Subtype grouping (e.g., `CRYOGENIC`)
- [x] Arrow navigation (`↑↓` / `j/k`) through sidebar entries
- [x] Visual indicator (`▶`) for currently open article
- [x] Highlight on selected entry

### Phase 5: Main Display Area
- [x] Layout: title bar + infobox + body (stacked vertically)
- [x] Title bar showing article name and metadata
- [x] Infobox rendering with flat fields, subtables, and stages
- [x] Body text rendering with headings and paragraphs
- [x] Minimum terminal size check with fallback message

### Phase 6: Link Handling & Unified List
- [x] Build unified link list from infobox flat fields (ordered), stages, then body
- [x] Track `infobox_link_count()` for rendering boundary
- [x] Link validation (check if target article exists)
- [x] Link cycling with `←→` / `h/l` navigation
- [x] Visual focus indication (Black on Cyan highlight)

### Phase 7: Navigation & History
- [x] History stack for visited articles (simple `VecDeque`, no max size limit yet)
- [x] `Backspace` to go back in history
- [x] `[`/`]` to jump between type headings
- [x] Type/subtype headings used for grouping; category pages not separately implemented
- [x] `Enter` to open/follow focused link
- [ ] Scroll state management (sidebar + body independent) — NOT YET IMPLEMENTED

### Phase 8: Advanced Features
- [ ] Tab sidebar modes: Sections / On This Page / History
- [ ] "On This Page" sidebar mode (table of contents from body headings)
- [ ] History sidebar mode showing visited articles
- [ ] Status bar at bottom with current context
- [ ] Broken link detection and dimmed rendering
- [ ] Body text scrolling with independent scroll offset
- [ ] Sidebar scrolling with independent scroll offset

Notes: Infobox subtables and stages (Phase 8 content) are implemented and rendered in `ui.rs`, but the sidebar modes, status bar, broken-link UI, and explicit scroll offsets are not yet implemented.

---

## File Structure (Target)

SpaceWiki/
├── Cargo.toml
├── articles/
│   ├── rs-25.toml
│   ├── space-shuttle.toml
│   └── ... (more articles)
└── src/
    ├── main.rs       — entry point, terminal setup, event loop
    ├── app.rs        — App struct, state management, navigation logic
    ├── article.rs    — article loading, TOML parsing
    ├── parser.rs     — body parser, link extraction, segment processing
    ├── types.rs      — all type definitions
    ├── ui.rs         — rendering, layout, sidebar, main content
    ├── infobox.rs    — infobox rendering, flat fields, subtables, stages
    └── link.rs       — link management, unified link list building

---

## Key Types to Define (types.rs)

### Articles & Metadata

```rust
pub struct Article {
    pub title: String,
    pub article_type: ArticleType,
    pub infobox: Infobox,
    pub body: Document,
}

pub enum ArticleType {
    Engine(EngineSubtype),
    LaunchVehicle(LaunchVehicleSubtype),
    Spacecraft(SpacecraftSubtype),
}

pub enum EngineSubtype {
    Cryogenic,
    Storable,
    SolidRocket,
}

pub enum LaunchVehicleSubtype {
    HeavyLift,
    MediumLift,
    SmallSat,
}

pub enum SpacecraftSubtype {
    Orbiter,
    Station,
    Probe,
}
```
### Body and Parsing

```rust
pub struct Document {
    pub blocks: Vec<Block>,
    pub links: Vec<Link>,
}

pub enum Block {
    Heading { level: u8, text: String },
    Paragraph { segments: Vec<Segment> },
}

pub enum Segment {
    Text(String),
    Link { display: String, target: String },
}

pub struct Link {
    pub display: String,
    pub target: String,
}
```

### Infobox and Fields

```rust
pub struct Infobox {
    pub flat_fields: HashMap<String, String>,
    pub field_order: Vec<String>,
    pub subtables: HashMap<String, SubtableData>,
    pub stages: Vec<Stage>,
}

pub struct SubtableData {
    pub rows: Vec<HashMap<String, String>>,
    pub style: SubtableStyle,
}

pub struct SubtableStyle {
    pub columns: usize,
    pub order: usize,
}

pub struct Stage {
    pub name: String,
    pub link: Option<String>,
    pub engines: Vec<String>,
    pub propellant: String,
    pub description: String,
}

pub enum SidebarEntry {
    TypeHeading(String),
    SubtypeHeading(String),
    Article(String),
}

pub enum SidebarMode {
    Sections,
    OnThisPage,
    History,
}
```

### Link Management
```rust
pub struct UnifiedLink {
    pub display: String,
    pub target: String,
    pub source: LinkSource,
}

pub enum LinkSource {
    InfboxFlatField,
    InfboxStage,
    Body,
}
```

---

## Article File Format (TOML)

```toml
[meta]
title = "RS-25"
type = "engine"
subtype = "cryogenic"

[infobox]
first_flight = "1981-04-12"
manufacturer = "[[Rocketdyne]]"
cost_unit = "$40M"
predecessor = "[[HG-3]]"
successor = ""
country = "United States"

[infobox.performance]
_style = "columns: 2; order: 1"
thrust_vac = "2,279 kN"
isp_vac = "452.3 s"
isp_sl = "418.7 s"

[infobox.design]
_style = "columns: 1; order: 2"
engine_type = "Cryogenic"
mass = "3,177 kg"

[[stages]]
name = "Space Shuttle Main"
link = "[[External Tank]]"
engines = ["[[RS-25]] x3"]
propellant = "LOX / LH2"
description = "Main engines for the Space Shuttle orbiter."

[body]
text = """
# Overview
The RS-25 is a liquid-fueled cryogenic rocket engine.

## History
Developed by [[Rocketdyne]] in the 1970s.

## Performance
High specific impulse and reliable performance.

[[Learn More|NASA RS-25]]
"""
```

TOML Parsing Notes:
- _style keys in subtables have format "columns: N; order: M" (both optional, defaults: columns=1, order=table order)
- field_order in [infobox] meta controls display order: "first_flight, manufacturer, country, predecessor, successor"
- Stages are ordered top-to-bottom as listed
- Body text supports [[link]] and [[link|label]] syntax

---

## App State (app.rs)

```rust
pub struct App {
    pub loaded_articles: HashMap<String, Article>,
    pub current_article: Option<String>,
    
    pub history: VecDeque<String>,
    pub sidebar_mode: SidebarMode,
    pub selected_sidebar_index: usize,
    pub sidebar_scroll_offset: usize,
    pub body_scroll_offset: usize,
    
    pub unified_links: Vec<UnifiedLink>,
    pub focused_link: Option<usize>,
    pub infobox_link_count: usize,
}

impl App {
    pub fn load_articles() -> Result<HashMap<String, Article>, Box<dyn std::error::Error>>
    
    pub fn open_article(&mut self, name: &str) -> Result<(), String>
    
    pub fn build_unified_links(&mut self) -> Result<(), String>
    pub fn compute_infobox_link_count(&mut self)
    pub fn link_is_valid(&self, target: &str) -> bool
    pub fn get_focused_link(&self) -> Option<&UnifiedLink>
    
    pub fn move_sidebar_up(&mut self)
    pub fn move_sidebar_down(&mut self)
    pub fn jump_to_next_type_heading(&mut self)
    pub fn jump_to_prev_type_heading(&mut self)
    pub fn cycle_link_next(&mut self)
    pub fn cycle_link_prev(&mut self)
    pub fn go_back(&mut self) -> bool
    pub fn follow_focused_link(&mut self) -> Result<(), String>
    pub fn cycle_sidebar_mode(&mut self)
    
    pub fn sidebar_entries(&self) -> Vec<SidebarEntry>
    pub fn get_selected_entry(&self) -> Option<&SidebarEntry>
    
    pub fn current_article_type_label(&self) -> String
    pub fn get_infobox_height(&self) -> u16
    pub fn body_needs_scrolling(&self) -> bool
}
```

Key Algorithm: Building Unified Links

Order matters for highlighting. Link indices must follow this exact order:

1. Infobox flat fields in field_order sequence
2. Remaining flat fields sorted alphabetically
3. For each stage in order:
   - Engine links left to right
   - Footer link (if present)
4. Body links in document order

Example: Article with 3 flat field links + 2 stages (stage 1 has 3 engines + 1 footer, stage 2 has 2 engines + 0 footer) + 5 body links = 14 total

Indices 0–2:     Flat field links
Indices 3–8:     Stage links (engines + footer)
Indices 9–13:    Body links
infobox_link_count = 9 (body links start at index 9)

---

## Parser (parser.rs)

```rust
pub fn parse_body(text: &str) -> Result<Document, String>

pub fn segments_from_str(s: &str) -> Vec<Segment>
pub fn extract_body_links(document: &Document) -> Vec<Link>
pub fn strip_link_markup(s: &str) -> String

pub fn parse_infobox_flat_fields(toml_table: &toml::Table) -> Result<HashMap<String, String>, String>
pub fn parse_infobox_subtables(toml_table: &toml::Table) -> Result<HashMap<String, SubtableData>, String>
pub fn parse_infobox_stages(toml_array: &[toml::Table]) -> Result<Vec<Stage>, String>
pub fn extract_links_from_infobox(infobox: &Infobox) -> Vec<Link>
```

Link Extraction Details:

- Flat fields: Any value containing [[...]] is a link. Extract target and display text.
- Stages: Engine strings like "[[RS-25]] x3" → link to RS-25, display "RS-25 x3"
- Stage footer: link field is extracted directly
- Body: Parse segments, collect all Link segments

---

## Layout Structure (ui.rs)

```
┌─ sidebar (20 cols) ──┬─ main area ──────────────────────────────┐
│ ENGINES              │ ┌─ title bar (3 rows) ──────────────────┐ │
│   CRYOGENIC          │ │ RS-25  Engine · Cryogenic             │ │
│ ▶ RS-25              │ ├─ infobox (dynamic height) ────────────┤ │
│   LOX/LH2 Engines    │ │ flat fields (columns)                 │ │
│                      │ │ ─────────────────────────────────────  │ │
│ SPACECRAFT           │ │ PERFORMANCE subtable                  │ │
│   ISS                │ │ ─────────────────────────────────────  │ │
│   Orbiter            │ │ Stage 1 | Stage 2                     │ │
│                      │ ├─ body (rest, scrollable) ─────────────┤ │
│                      │ │ # Overview                            │ │
│                      │ │ Prose with [[links]]...               │ │
│                      │ │ (scrollable with ↑↓)                  │ │
└──────────────────────┴────────────────────────────────────────────┘
```

Constraints:
- Sidebar: fixed 20 columns
- Main area: title bar 3 rows fixed, infobox dynamic, body fills remaining
- Minimum terminal: 80x24 (check in draw(), show "Terminal too small (min 80x24)" and skip rendering)
- Scroll offsets: clamped to valid ranges

---

## Infobox Rendering (infobox.rs)

Three sections inside one outer border:

1. Flat Fields (height: 6 rows + border)
   - Dynamic column layout: columns = max(1, area.width / 18)
   - Each column is vertical stack of label+value pairs
   - Wrap enabled on values
   - Link values: Cyan, non-link values: White
   - Link counter starts at index 0, carries across all columns
   - Focused link: Black on Cyan

2. Subtables (height: dynamic)
   - One section per subtable (displayed left-to-right if space allows, else stacked)
   - Divider line above
   - Format: columns determines layout
   - Header row, then data rows
   - No link highlighting yet (reserved for Phase 8)

3. Stages (height: up to 10 rows + border)
   - Horizontal card layout: [Stage 1 | Stage 2 | Stage 3]
   - Each card: name, engines (stripped [[...]]), propellant, footer link
   - Engine links and footer links counted in unified list
   - Link counter continues from infobox_link_count
   - Focused link: Black on Cyan

Height Calculation:
flat (6) + 1 (border) 
+ subtables_height 
+ stages_height 
+ dividers (2)

---

## Scroll State Management

Sidebar:
- Scroll offset tracks first visible entry
- Scroll offsets clamp to: [0, max(0, total_entries - visible_rows)]
- Navigation (↑↓) adjusts offset to keep selection visible

Body:
- Independent scroll offset
- Body height = total terminal height - title bar (3) - infobox (dynamic)
- Content height = number of rendered body lines
- Scroll offset clamps to: [0, max(0, content_height - body_height)]
- Navigation (↑↓ when body has focus) adjusts offset

---

## Link Validation and Error Handling

Loading Articles:

```rust
pub fn load_articles() -> Result<HashMap<String, Article>, Box<dyn std::error::Error>> {
    // 1. Scan articles/ directory
    // 2. For each .toml file:
    //    - Parse TOML
    //    - Validate required fields: [meta] with title, type, subtype
    //    - Parse infobox and body
    //    - Return error if parsing fails, log and skip invalid files
    // 3. Return HashMap of successfully loaded articles
}
```

Link Validation:

```rust
pub fn link_is_valid(&self, target: &str) -> bool {
    // Check if target article exists in loaded_articles
    // Return false if not found or if target is external URL (for Phase 9)
}
```

Opening Articles:

```rust
pub fn open_article(&mut self, name: &str) -> Result<(), String> {
    // 1. Check if article exists
    // 2. Add current article to history (if any)
    // 3. Set current_article = name
    // 4. Build unified link list
    // 5. Reset: focused_link = None, offsets = 0
    // 6. Return Ok or descriptive error
}
```

Following Links:

```rust
pub fn follow_focused_link(&mut self) -> Result<(), String> {
    // 1. Get focused link
    // 2. Validate target exists
    // 3. Open article at target
    // 4. Return Ok or error
}
```

---

## Navigation Keybindings

| Key | Action |
|-----|--------|
| ↑ / k | Scroll sidebar up (or body if in body mode) |
| ↓ / j | Scroll sidebar down (or body if in body mode) |
| [ | Jump to previous type heading in sidebar |
| ] | Jump to next type heading in sidebar |
| ← / h | Cycle focused link backward |
| → / l | Cycle focused link forward |
| Enter | Open selected sidebar article OR follow focused link |
| Backspace | Go back in history |
| Tab | Cycle sidebar mode (Sections → On This Page → History → Sections) |
| q | Quit application |

---

## Visual Style

- Background: Terminal default (dark)
- Type/subtype headings (sidebar): DarkGray
- Article entries (sidebar): Gray
- Current article marker: ▶ prefix
- Selected sidebar entry: Black on Gray
- Body headings: White bold (h1), Gray bold (h2), DarkGray bold (h3)
- Body text: DarkGray
- Links (unfocused): Cyan
- Links (focused): Black on Cyan
- Infobox labels: DarkGray
- Infobox values (non-link): White
- Infobox values (link, unfocused): Cyan
- Infobox values (link, focused): Black on Cyan
- Stage names: Gray bold (or Cyan if linked)
- Status bar (Phase 8): DarkGray text on default background
- Broken links (Phase 8): DarkGray (dimmed)

---

## Implementation Phases (Suggested Order)

1. Phase 1–2: Project setup, terminal loop, article loading
2. Phase 3: Parser (body + infobox)
3. Phase 4–5: Sidebar + main display rendering
4. Phase 6: Link management and unified list (critical for proper highlighting)
5. Phase 7: History, link following, navigation
6. Phase 8: Scrolling, sidebar modes, polish

---

## Error Handling Strategy

- TOML parsing errors: Log to stderr, skip file, continue
- Missing articles: Return Err(String) with description
- Malformed links: Strip [[...]] gracefully, treat as text if parsing fails
- Terminal too small: Render "Terminal too small (min 80x24)" message and skip rest

---

## Testing Checklist (Before Phase 8)

- [ ] Load 3+ test articles with varied types/subtypes
- [ ] Sidebar groups correctly by type/subtype
- [ ] Navigate sidebar with ↑↓ without crashing
- [ ] Open articles and display title bar, infobox, body
- [ ] Verify unified link list order (infobox flat → stages → body)
- [ ] Focus links with ←→ and verify highlighting
- [ ] Follow links with Enter and verify navigation
- [ ] Go back with Backspace and verify history
- [ ] Jump type headings with [/] and verify position
- [ ] Cycle sidebar modes with Tab (basic switching, not full rendering)
- [ ] Quit with q without errors

---

## Notes for Next Steps

- This spec is complete enough to hand to another LLM with confidence
- Each phase builds on previous ones; test incrementally
- Use 2–3 sample TOML articles for rapid iteration
- Consider helper utilities early (e.g., render_link(), highlight_segment())
- Phase 6 (link management) is the trickiest; design it carefully before implementation