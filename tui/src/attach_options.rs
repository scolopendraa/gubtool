use {
    crate::{
        common::helpers::bordered_block,
        event::KeyContext,
        impl_tablecontroller_for_commands,
        panes::{TabPane, TablePane},
        popup::{Popup, PopupState, centered_popup},
        screen::Screen,
        theme::theme,
    },
    crossterm::event::KeyCode,
    gubtool_core::game_version::Game,
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Rect},
        text::{Line, Span},
        widgets::{Paragraph, Wrap},
    },
    shared::command::{Command, OptCmd},
    std::{cell::RefCell, rc::Rc},
};

pub struct AttachOptions {
    game_screen: Rc<RefCell<Game>>,
    popup_state: PopupState,
    ds2_tabs:    TabPane,
    er_tabs:     TabPane,
}

impl AttachOptions {
    pub fn new(game_screen: Rc<RefCell<Game>>) -> Self {
        Self {
            game_screen,
            popup_state: PopupState::default(),
            ds2_tabs: TabPane::new(&["Player", "Utility"], vec![
                TablePane::new_static(&Ds2Player),
                TablePane::new_static(&Ds2Utility),
            ]),
            er_tabs: TabPane::new(&["Player", "Utility"], vec![
                TablePane::new_static(&ErPlayer),
                TablePane::new_static(&ErUtility),
            ]),
        }
    }

    fn paragraph(&self) -> Paragraph<'static> {
        const INFO: &str = "These options will be automatically applied when gubtool attaches to ";

        Paragraph::new(Line::from(vec![
            Span::raw(INFO),
            Span::raw(self.game_screen.borrow().to_string()),
        ]))
        .wrap(Wrap {
            trim: true,
        })
        .style(theme().muted)
        .block(bordered_block(Some("Attach Options")))
    }
}

impl Popup for AttachOptions {
    fn popup_state(&mut self) -> &mut PopupState {
        &mut self.popup_state
    }
    fn screen(&mut self) -> &mut dyn Screen {
        self
    }
    fn popup_rect(&self, frame: &mut Frame) -> Rect {
        centered_popup(75, 75, frame.area())
    }
}

impl Screen for AttachOptions {
    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let paragraph = self.paragraph();
        let lines = paragraph.line_count(area.width).clamp(3, 5);

        let [info, tabs] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(lines as u16), Constraint::Fill(1)])
            .areas(area);

        frame.render_widget(paragraph, info);

        match *self.game_screen.borrow() {
            Game::DarkSouls2 => self.ds2_tabs.draw(frame, tabs),
            Game::EldenRing => self.er_tabs.draw(frame, tabs),
        }
    }
    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        match *self.game_screen.borrow() {
            Game::DarkSouls2 => self.ds2_tabs.handle_keys(ctx),
            Game::EldenRing => self.er_tabs.handle_keys(ctx),
        }
    }
}

const DS2_PLAYER: [Command; 10] = [
    Command::Toggle(&darksouls2::attach::NoDeath),
    Command::Toggle(&darksouls2::attach::NoDamage),
    Command::Toggle(&darksouls2::attach::InfinitePoise),
    Command::Toggle(&darksouls2::attach::InfiniteStamina),
    Command::Toggle(&darksouls2::attach::InfiniteDurability),
    Command::Toggle(&darksouls2::attach::InfiniteConsumables),
    Command::Toggle(&darksouls2::attach::NoHollowing),
    Command::Toggle(&darksouls2::attach::NoSoulLoss),
    Command::Toggle(&darksouls2::attach::Hidden),
    Command::Toggle(&darksouls2::attach::Silent),
];

const DS2_UTILITY: [Command; 7] = [
    Command::Toggle(&darksouls2::attach::SkipCredits),
    Command::Toggle(&darksouls2::attach::FastQuitout),
    Command::Toggle(&darksouls2::attach::DisableRoll),
    Command::Toggle(&darksouls2::attach::DisableBackstep),
    Command::Toggle(&darksouls2::attach::SkipIvoryKingGauntlet),
    Command::Toggle(&darksouls2::attach::DisableLoyceKnights),
    Command::Toggle(&darksouls2::attach::StartEventLogger),
];

const ER_PLAYER: [Command; 14] = [
    Command::Toggle(&eldenring::attach::NoDeath),
    Command::Toggle(&eldenring::attach::NoDamage),
    Command::Toggle(&eldenring::attach::InfinitePoise),
    Command::Toggle(&eldenring::attach::OneShot),
    Command::Toggle(&eldenring::attach::RuneArc),
    Command::Toggle(&eldenring::attach::SetRfbsOnLoad),
    Command::Toggle(&eldenring::attach::Hidden),
    Command::Toggle(&eldenring::attach::Silent),
    Command::Toggle(&eldenring::attach::InfiniteStamina),
    Command::Toggle(&eldenring::attach::InfiniteFp),
    Command::Toggle(&eldenring::attach::InfiniteConsumables),
    Command::Toggle(&eldenring::attach::InfiniteArrows),
    Command::Toggle(&eldenring::attach::TorrentAnywhere),
    Command::Toggle(&eldenring::attach::TorrentNoDeath),
];

const ER_UTILITY: [Command; 14] = [
    Command::Option(OptCmd::F32(&eldenring::attach::FpsCap)),
    Command::Option(OptCmd::F32(&eldenring::attach::GameSpeed)),
    Command::Toggle(&eldenring::attach::DisableLogos),
    Command::Toggle(&eldenring::attach::MuteMusic),
    Command::Toggle(&eldenring::attach::DisableAreaWelcomeMessage),
    Command::Toggle(&eldenring::attach::StutterFix),
    Command::Toggle(&eldenring::attach::MapInCombat),
    Command::Toggle(&eldenring::attach::TravelInDungeons),
    Command::Toggle(&eldenring::attach::DrawHitboxes),
    Command::Toggle(&eldenring::attach::ShowAllGraces),
    Command::Toggle(&eldenring::attach::ShowAllMaps),
    Command::Toggle(&eldenring::attach::DisableRoll),
    Command::Toggle(&eldenring::attach::DisableJump),
    Command::Toggle(&eldenring::attach::DisableBackstep),
];

impl_tablecontroller_for_commands!(Ds2Player, DS2_PLAYER);
impl_tablecontroller_for_commands!(Ds2Utility, DS2_UTILITY);
impl_tablecontroller_for_commands!(ErPlayer, ER_PLAYER);
impl_tablecontroller_for_commands!(ErUtility, ER_UTILITY);
