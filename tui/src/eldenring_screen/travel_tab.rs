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
    eldenring::{
        event,
        resources::{
            bosses::{self, BOSSES},
            graces::{self, GRACES},
        },
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
};

pub(super) struct TravelTab {
    pub pane_manager: PaneManager,
}

impl TravelTab {
    pub fn new() -> Self {
        TravelTab {
            pane_manager: PaneManager::new(vec![
                TablePane::new_static(&BossTable)
                    .with_title("Bosses")
                    .with_controls(&CONTROLS)
                    .freeze()
                    .boxed(),
                TablePane::new_static(&BonfireTable)
                    .with_title("Graces")
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
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rect);

        self.pane_manager.draw(frame, &layout);

        let text = self.revive_status_line();
        let width = layout[0].width;
        let len = text.width();
        frame.render_widget(text, layout[0] + Offset::new(width as i32 - len as i32 - 1, 0));
    }

    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        self.pane_manager.handle_keys(ctx);
    }
}

struct BossTable;
impl TableController for BossTable {
    fn make_table_view(&self) -> TableView {
        let rows: Vec<Row> = BOSSES
            .iter()
            .map(|boss| {
                Row::new([
                    Cell::from(boss.name),
                    Cell::from(boss.main_area).fg(theme().muted),
                ])
            })
            .collect();

        TableView::new(rows).with_widths(&[Constraint::Min(44), Constraint::Max(33)])
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            spawn_task! {
                BOSSES[selected].warp().await.send_error()
            }
        }

        if ctx.key_with_modifiers(KeyCode::Char('r'), KeyModifiers::CONTROL) {
            BOSSES[selected].revive(true).send_error()
        }

        if ctx.key_char('r') {
            BOSSES[selected].revive(false).send_error()
        }

        if ctx.key_char('f') {
            request_search(&BossesSearch);
        }
    }
}

struct BonfireTable;
impl TableController for BonfireTable {
    fn make_table_view(&self) -> TableView {
        let rows: Vec<Row> = GRACES
            .iter()
            .map(|bonfire| {
                Row::new([
                    Cell::from(bonfire.name),
                    Cell::from(bonfire.main_area).fg(theme().muted),
                ])
            })
            .collect();

        TableView::new(rows).with_widths(&[Constraint::Min(42), Constraint::Max(33)])
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            GRACES[selected].warp().send_error();
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
            .map(|boss| Utf32String::from(format!("{}|{}", boss.name, boss.main_area)))
            .collect()
    }
}

struct BonfireSearch;
impl SearchRequest for BonfireSearch {
    fn items(&self) -> Vec<Utf32String> {
        graces::GRACES
            .iter()
            .map(|grace| Utf32String::from(format!("{}|{}", grace.name, grace.main_area)))
            .collect()
    }
}

const CONTROLS: [Control; 2] = [
    Control::new("r", "Revive"),
    Control::new("ctrl-r", "Revive FE"),
];

impl TravelTab {
    fn revive_status_line(&self) -> Span<'static> {
        let selected_idx = self.pane_manager.get_list_selected(0).unwrap_or_default();
        let boss = BOSSES[selected_idx];
        let revive_status = boss.revive_status();

        let style = match revive_status {
            event::AliveStatus::Dead => Style::from(theme().error),
            _ => Style::from(theme().success),
        };

        let text = if !eldenring::is_player_loaded() {
            "".to_string()
        } else {
            revive_status.to_string()
        };

        Span::from(text).style(style)
    }
}
