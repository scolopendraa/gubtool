use {
    crate::{
        common::controls::Control,
        event::{AnyhowExt, KeyContext, request_search},
        input::fuzzy_finder::SearchRequest,
        panes::{PaneManager, TableController, TablePane, TableView},
        screen::Screen,
        spawn_task,
        theme::theme,
    },
    crossterm::event::{KeyCode, KeyModifiers},
    darksouls2::{
        bonfire,
        resources::{bonfires, bosses},
    },
    nucleo_matcher::Utf32String,
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Offset, Rect},
        style::Stylize,
        text::Span,
        widgets::{Cell, Row},
    },
    ratatui_themes::Style,
    std::thread,
};

pub(super) struct TravelTab {
    pub pane_manager: PaneManager,
}

impl TravelTab {
    pub fn new() -> Self {
        TravelTab {
            pane_manager: PaneManager::new(vec![
                TablePane::new_static(&BossList)
                    .with_title("Bosses")
                    .with_controls(&BOSSES_CONTROLS)
                    .freeze()
                    .boxed(),
                TablePane::new_static(&BonfireTable)
                    .with_title("Bonfires")
                    .with_controls(&BONFIRE_CONTROLS)
                    .freeze()
                    .boxed(),
            ]),
        }
    }
}

impl Screen for TravelTab {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(rect);

        self.pane_manager.draw(frame, &layout);

        let text = self.boss_alive_status_line();
        let width = layout[0].width;
        let len = text.width();
        frame.render_widget(text, layout[0] + Offset::new(width as i32 - len as i32 - 1, 0));

        let text = self.bonfire_lit_status_line();
        let width = layout[1].width;
        let len = text.width();
        frame.render_widget(text, layout[1] + Offset::new(width as i32 - len as i32 - 1, 0));
    }

    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        self.pane_manager.handle_keys(ctx);
    }
}

struct BossList;
impl TableController for BossList {
    fn make_table_view(&self) -> TableView {
        let rows = bosses::BOSSES
            .iter()
            .map(|boss| Row::new([boss.name]))
            .collect();

        TableView::new(rows)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            thread::spawn(move || bosses::BOSSES[selected].warp().send_error());
        }

        if ctx.key_char('f') {
            request_search(&BossesSearch);
        }

        if ctx.key_char('r') {
            spawn_task! {
                bosses::BOSSES[selected].revive().await.send_error();
            }
        }
    }
}

struct BonfireTable;
impl TableController for BonfireTable {
    fn make_table_view(&self) -> TableView {
        let rows = bonfires::BONFIRES
            .iter()
            .map(|bonfire| {
                Row::new([
                    Cell::from(bonfire.name),
                    Cell::from(bonfire.map_id.to_string()).fg(theme().muted),
                ])
            })
            .collect();
        TableView::new(rows).with_widths(&[Constraint::Min(30), Constraint::Max(26)])
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            thread::spawn(move || bonfires::BONFIRES[selected].warp().send_error());
        }

        if ctx.key_with_modifiers(KeyCode::Char('t'), KeyModifiers::CONTROL) {
            bonfire::light_all_bonfires().send_error();
        }

        if ctx.key_char('t') {
            bonfires::BONFIRES[selected].unlock().send_error();
        }

        if ctx.key_char('r') {
            bonfires::BONFIRES[selected].rest().send_error();
        }

        if ctx.key_char('f') {
            request_search(&BonfireSearch);
        }
    }
}

struct BossesSearch;
impl SearchRequest for BossesSearch {
    fn items(&self) -> Vec<Utf32String> {
        bosses::BOSSES
            .iter()
            .map(|boss| Utf32String::from(boss.name))
            .collect()
    }
}

struct BonfireSearch;
impl SearchRequest for BonfireSearch {
    fn items(&self) -> Vec<Utf32String> {
        bonfires::BONFIRES
            .iter()
            .map(|bonfire| Utf32String::from(format!("{}|{}", bonfire.name, bonfire.map_id)))
            .collect()
    }
}

const BOSSES_CONTROLS: [Control; 1] = [Control::new("r", "Revive")];

const BONFIRE_CONTROLS: [Control; 3] = [
    Control::new("r", "Rest"),
    Control::new("t", "Light"),
    Control::new("ctrl-t", "Light All"),
];

impl TravelTab {
    fn boss_alive_status_line(&self) -> Span<'static> {
        let selected_idx = self.pane_manager.get_list_selected(0).unwrap_or_default();
        let boss = &bosses::BOSSES[selected_idx];
        let is_alive = boss.is_alive();
        let text = if darksouls2::is_player_loaded() {
            match is_alive {
                true => "Alive",
                false => "Dead",
            }
        } else {
            ""
        };
        let style =
            if is_alive { Style::from(theme().success) } else { Style::from(theme().error) };
        Span::raw(text).style(style)
    }

    fn bonfire_lit_status_line(&self) -> Span<'static> {
        let selected_idx = self.pane_manager.get_list_selected(1).unwrap_or_default();
        let bonfire = &bonfires::BONFIRES[selected_idx];
        let lit = bonfire.is_lit().unwrap_or_default();
        let text = if !darksouls2::is_player_loaded() {
            ""
        } else if lit {
            "Lit"
        } else {
            "Unlit"
        };
        let style = if lit { Style::from(theme().success) } else { Style::from(theme().error) };
        Span::raw(text).style(style)
    }
}
