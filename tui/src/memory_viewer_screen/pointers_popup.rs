use {
    crate::{
        event::KeyContext,
        memory_viewer_screen::memory_viewer,
        panes::{Pane, TableController, TablePane, TableView},
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

pub(super) struct PointersPopup {
    pointers:    TablePane,
    popup_state: PopupState,
}

impl PointersPopup {
    pub fn new() -> Self {
        Self {
            pointers:    TablePane::new_static(&PointersList).with_title("Cached Pointers"),
            popup_state: PopupState::default(),
        }
    }
}

struct PointersList;
impl TableController for PointersList {
    fn make_table_view(&self) -> TableView {
        let pointers = get_pointers();
        let rows = pointers
            .into_iter()
            .map(|(name, addr)| Row::new([name, format!("{:#X?}", addr)]))
            .collect();

        TableView::new(rows).with_widths(&[Constraint::Min(30), Constraint::Fill(1)])
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            let pointers = get_pointers();
            let (_, address) = pointers[selected];
            memory_viewer().jump(address);
        }
    }
}

impl Popup for PointersPopup {
    fn popup_state(&mut self) -> &mut PopupState {
        &mut self.popup_state
    }
    fn screen(&mut self) -> &mut dyn Screen {
        &mut self.pointers
    }
    fn popup_rect(&self, frame: &mut Frame) -> Rect {
        centered_popup(60, 60, frame.area())
    }
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        if self.pointers.selected().is_none() {
            self.pointers.select(0);
        }
        self.screen().draw(frame, rect);
    }
}

fn get_pointers() -> Vec<(String, u64)> {
    let mut pointers: Vec<(String, u64)> = Vec::new();
    match attached::game() {
        Ok(Game::DarkSouls2) => {
            pointers.extend(darksouls2::get_pointers());
            pointers.extend(
                darksouls2::player::player()
                    .pointers()
                    .iter()
                    .map(|(name, addr)| (format!("Player {}", name), *addr))
                    .collect::<Vec<_>>(),
            );
            pointers.extend(
                darksouls2::target::target()
                    .pointers()
                    .iter()
                    .map(|(name, addr)| (format!("Target {}", name), *addr))
                    .collect::<Vec<_>>(),
            );
        }
        Ok(Game::EldenRing) => {
            pointers.extend(eldenring::get_pointers());
            pointers.extend(
                eldenring::player::player()
                    .pointers()
                    .iter()
                    .map(|(name, addr)| (format!("Player {}", name), *addr))
                    .collect::<Vec<_>>(),
            );
            pointers.extend(
                eldenring::target::target()
                    .pointers()
                    .iter()
                    .map(|(name, addr)| (format!("Target {}", name), *addr))
                    .collect::<Vec<_>>(),
            );
            pointers.extend(
                eldenring::player::torrent()
                    .pointers()
                    .iter()
                    .map(|(name, addr)| (format!("Torrent {}", name), *addr))
                    .collect::<Vec<_>>(),
            );
        }
        Err(_) => (),
    }
    pointers
}
