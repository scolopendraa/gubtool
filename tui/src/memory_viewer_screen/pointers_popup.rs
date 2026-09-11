use {
    crate::{
        common::controls::Control,
        event::KeyContext,
        memory_viewer_screen::memory_viewer,
        panes::{Pane, TabPane, TableController, TablePane, TableView},
        popup::{Popup, PopupState, centered_popup},
        screen::Screen,
    },
    gubtool_core::{attached, game_version::Game},
    ratatui::{
        Frame,
        layout::{Constraint, Rect},
        widgets::Row,
    },
};

const CONTROLS: [Control; 1] = [Control::new("r", "Load Pointers")];

pub(super) struct PointersPopup {
    popup_state: PopupState,
    ds2_tabs:    TabPane,
    ds3_tabs:    TabPane,
    er_tabs:     TabPane,
}

impl PointersPopup {
    pub fn new() -> Self {
        Self {
            popup_state: PopupState::default(),
            ds2_tabs:    TabPane::new(&["Globals", "Player", "Target", "Code Cave"], vec![
                TablePane::new_static(&Ds2Globals),
                TablePane::new_static(&Ds2Player),
                TablePane::new_static(&Ds2Target),
                TablePane::new_static(&Ds2CodeCave),
            ])
            .with_controls(&CONTROLS),
            ds3_tabs:    TabPane::new(&["Globals", "Player", "Target", "Code Cave"], vec![
                TablePane::new_static(&Ds3Globals),
                TablePane::new_static(&Ds3Player),
                TablePane::new_static(&Ds3Target),
                TablePane::new_static(&Ds3CodeCave),
            ])
            .with_controls(&CONTROLS),
            er_tabs:     TabPane::new(
                &["Globals", "Player", "Target", "Torrent", "Code Cave"],
                vec![
                    TablePane::new_static(&ErGlobals),
                    TablePane::new_static(&ErPlayer),
                    TablePane::new_static(&ErTarget),
                    TablePane::new_static(&ErTorrent),
                    TablePane::new_static(&ErCodeCave),
                ],
            )
            .with_controls(&CONTROLS),
        }
    }
}

macro_rules! declare_pointers_table {
    ($name:ident, $pointers:expr) => {
        struct $name;

        impl TableController for $name {
            fn make_table_view(&self) -> TableView {
                let rows = $pointers
                    .into_iter()
                    .map(|(name, addr)| Row::new([name, format!("{:#x?}", addr)]))
                    .collect();

                TableView::new(rows).with_widths(&[Constraint::Min(30), Constraint::Fill(1)])
            }
            fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
                if ctx.key_enter() {
                    let (_, address) = $pointers[selected];
                    memory_viewer().jump(address);
                }
            }
        }
    };
}

impl Popup for PointersPopup {
    fn popup_state(&mut self) -> &mut PopupState {
        &mut self.popup_state
    }
    fn screen(&mut self) -> &mut dyn Screen {
        self
    }
    fn popup_rect(&self, frame: &mut Frame) -> Rect {
        centered_popup(70, 70, frame.area())
    }
}

impl Screen for PointersPopup {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        match attached::game() {
            Ok(Game::DarkSouls2) => {
                if self.ds2_tabs.selected().is_none() {
                    self.ds2_tabs.select(0);
                }
                self.ds2_tabs.draw(frame, rect);
            }
            Ok(Game::DarkSouls3) => {
                if self.ds3_tabs.selected().is_none() {
                    self.ds3_tabs.select(0);
                }
                self.ds3_tabs.draw(frame, rect);
            }
            Ok(Game::EldenRing) => {
                if self.er_tabs.selected().is_none() {
                    self.er_tabs.select(0);
                }
                self.er_tabs.draw(frame, rect);
            }
            _ => (),
        }
    }

    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        match attached::game() {
            Ok(Game::DarkSouls2) => {
                if ctx.key_char('r') {
                    darksouls2::load_all_pointers();
                }
                self.ds2_tabs.handle_keys(ctx);
            }
            Ok(Game::DarkSouls3) => {
                if ctx.key_char('r') {
                    darksouls3::load_all_pointers();
                }
                self.ds3_tabs.handle_keys(ctx);
            }
            Ok(Game::EldenRing) => {
                if ctx.key_char('r') {
                    eldenring::load_all_pointers();
                }
                self.er_tabs.handle_keys(ctx);
            }
            _ => (),
        }
    }
}

declare_pointers_table!(Ds2Globals, darksouls2::global_pointers());
declare_pointers_table!(Ds2Player, darksouls2::player_pointers());
declare_pointers_table!(Ds2Target, darksouls2::target_pointers());
declare_pointers_table!(Ds2CodeCave, darksouls2::cave_pointers());

declare_pointers_table!(Ds3Globals, darksouls3::global_pointers());
declare_pointers_table!(Ds3Player, darksouls3::player_pointers());
declare_pointers_table!(Ds3Target, darksouls3::target_pointers());
declare_pointers_table!(Ds3CodeCave, darksouls3::cave_pointers());

declare_pointers_table!(ErGlobals, eldenring::global_pointers());
declare_pointers_table!(ErPlayer, eldenring::player_pointers());
declare_pointers_table!(ErTarget, eldenring::target_pointers());
declare_pointers_table!(ErTorrent, eldenring::torrent_pointers());
declare_pointers_table!(ErCodeCave, eldenring::cave_pointers());