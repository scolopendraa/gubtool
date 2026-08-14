use {
    crate::{
        attach_options::AttachOptions,
        common::controls::{Control, HelpPopup},
        darksouls2_screen::DarkSouls2Screen,
        debug_screen::DebugPopup,
        eldenring_screen::EldenRingScreen,
        event::{
            AnyhowExt,
            Event,
            InfoType,
            KeyContext,
            ResultExt,
            send_event,
            start_event_loop_thread,
        },
        game_screen_selector::GameScreenSelector,
        impl_game_screen,
        input::{fuzzy_finder::FuzzyFinder, input_prompt::InputPrompt},
        memory_viewer_screen::MemoryViewerScreen,
        popup::Popup,
        process_selector::ProcessSelector,
        screen::GameScreen,
        spawn_task,
        theme::{ThemeSelector, theme},
        ui_state::UiState,
    },
    color_eyre::eyre::Result,
    config::Config,
    crossterm::event::{KeyCode, KeyModifiers},
    gubtool_core::{
        attached::{self, detach_if_invalid, is_attached},
        game_version::Game,
    },
    ratatui::{
        DefaultTerminal,
        Frame,
        layout::{Alignment, Constraint, Direction, Layout},
        style::Stylize,
        widgets::{Block, Paragraph},
    },
    std::{
        cell::{LazyCell, RefCell},
        rc::Rc,
    },
};

pub struct App {
    running:            bool,
    pub game_screen:    Rc<RefCell<Game>>,
    help:               HelpPopup,
    debug:              DebugPopup,
    show_info:          bool,
    info_message:       String,
    info_type:          InfoType,
    input:              InputPrompt,
    fuzzy_finder:       FuzzyFinder,
    pub has_pressed_f1: bool,

    theme_selector:           ThemeSelector,
    process_selector:         ProcessSelector,
    game_screen_selector:     GameScreenSelector,
    pub attach_options:       AttachOptions,
    pub memory_viewer_screen: MemoryViewerScreen,

    elden_ring:   LazyCell<EldenRingScreen>,
    dark_souls_2: LazyCell<DarkSouls2Screen>,
}

impl App {
    pub fn new() -> Self {
        let game_screen = Rc::new(RefCell::new(Game::EldenRing));
        App {
            running:        true,
            game_screen:    Rc::clone(&game_screen),
            help:           HelpPopup::new(&HELP_ENTRIES),
            debug:          DebugPopup::default(),
            show_info:      false,
            has_pressed_f1: false,
            info_message:   "".to_string(),
            info_type:      InfoType::SysError,
            input:          InputPrompt::default(),
            fuzzy_finder:   FuzzyFinder::default(),

            theme_selector:       ThemeSelector::new(),
            process_selector:     ProcessSelector::new(),
            game_screen_selector: GameScreenSelector::new(),
            attach_options:       AttachOptions::new(Rc::clone(&game_screen)),
            memory_viewer_screen: MemoryViewerScreen::new(),

            elden_ring:   LazyCell::new(EldenRingScreen::new),
            dark_souls_2: LazyCell::new(DarkSouls2Screen::new),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let rx = start_event_loop_thread();
        config::start_watcher_thread();
        UiState::apply(&mut self);

        try_auto_attach();

        while self.running {
            terminal.draw(|frame| self.draw(frame))?;

            match rx.recv()? {
                Event::Key(mut ctx) => self.handle_keys(&mut ctx),
                Event::Info((text, info_type)) => {
                    self.info_message = text;
                    self.info_type = info_type;
                    self.show_info = true;
                }
                Event::RenderTick => {
                    detach_if_invalid();

                    match attached::game() {
                        Ok(Game::EldenRing) => eldenring::update(),
                        Ok(Game::DarkSouls2) => darksouls2::update(),
                        Err(_) => try_auto_attach(),
                    }
                }
                Event::Input((prompt, sender, type_id)) => self.input.show(prompt, sender, type_id),
                Event::SearchRequest(request) => {
                    self.fuzzy_finder.show(request);
                }
                Event::SearchResult(selected) => {
                    self.fuzzy_finder.request.unwrap().jump(&mut self, selected);
                }
                Event::GameScreen(game) => {
                    *self.game_screen.borrow_mut() = game;
                    let _ = UiState::update(|c| c.global.game_screen = game);
                }
                Event::Attach => {
                    if let Ok(game) = attached::game() {
                        send_event(Event::GameScreen(game));
                        spawn_task! {
                            match game {
                                Game::DarkSouls2 => {
                                    darksouls2::reset();
                                    darksouls2::attach().await
                                }
                                Game::EldenRing => {
                                    eldenring::reset();
                                    eldenring::attach().await
                                }
                            }
                            .send_error()
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let background = Block::default().bg(theme().bg);
        frame.render_widget(background, frame.area());

        let constraints = if self.show_info || !self.has_pressed_f1 {
            vec![
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ]
        } else {
            vec![Constraint::Length(1), Constraint::Fill(1)]
        };

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(frame.area());

        let [pid_area, version_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Max(25), Constraint::Fill(1)])
            .areas(layout[0]);

        frame.render_widget(pid_paragraph(), pid_area);
        frame.render_widget(version_paragraph(), version_area);

        if !self.has_pressed_f1 {
            let info_paragraph = Paragraph::new("Press F1 to show controls").style(theme().warning);
            frame.render_widget(info_paragraph, layout[2]);
        } else if self.show_info {
            let style = match self.info_type {
                InfoType::SysError => theme().error,
                InfoType::GameError => theme().warning,
                InfoType::Success => theme().success,
            };
            let info_paragraph = Paragraph::new(self.info_message.to_string()).style(style);
            frame.render_widget(info_paragraph, layout[2]);
        }

        self.current_screen().draw(frame, layout[1]);

        self.fuzzy_finder.draw_if_open(frame);
        self.attach_options.draw_if_open(frame);
        self.help.draw_if_open(frame);
        self.theme_selector.draw_if_open(frame);
        self.game_screen_selector.draw_if_open(frame);
        self.process_selector.draw_if_open(frame);
        self.debug.draw_if_open(frame);
        self.memory_viewer_screen.draw_if_open(frame);
        self.input.draw_if_open(frame);
    }

    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        if ctx.key_with_modifiers(KeyCode::Char('c'), KeyModifiers::CONTROL) {
            self.running = false
        }

        if self.show_info {
            self.show_info = false;
        }

        let popups: [&mut dyn Popup; 9] = [
            &mut self.help,
            &mut self.fuzzy_finder,
            &mut self.input,
            &mut self.theme_selector,
            &mut self.game_screen_selector,
            &mut self.process_selector,
            &mut self.debug,
            &mut self.memory_viewer_screen,
            &mut self.attach_options,
        ];

        for popup in popups {
            if popup.handle_keys_if_open(ctx) {
                return;
            }
        }

        self.current_screen().handle_keys(ctx);

        if ctx.key_char('a') {
            self.attach_options.show();
        }

        if ctx.key_char('p') {
            self.process_selector.show();
        }

        if ctx.key_char('o') {
            self.game_screen_selector.show();
        }

        if ctx.key_f(1) {
            self.help.show();
            if !self.has_pressed_f1 {
                self.has_pressed_f1 = true;
                let _ = UiState::update(|c| c.global.has_pressed_f1 = true);
            }
        }

        if ctx.key_with_modifiers(KeyCode::F(12), KeyModifiers::CONTROL) {
            self.memory_viewer_screen.show();
        }

        if ctx.key_f(12) {
            self.theme_selector.show();
        }

        #[cfg(debug_assertions)]
        if ctx.key_with_modifiers(KeyCode::F(5), KeyModifiers::CONTROL) {
            self.debug.show();
        }
    }

    pub fn current_screen(&mut self) -> &mut dyn GameScreen {
        match *self.game_screen.borrow() {
            Game::EldenRing => &mut *self.elden_ring,
            Game::DarkSouls2 => &mut *self.dark_souls_2,
        }
    }
}

fn try_auto_attach() {
    if let Some(result) = attached::try_auto_attach() {
        result.send_error();
        send_event(Event::Attach);
    }
}

fn pid_paragraph() -> Paragraph<'static> {
    if is_attached() {
        Paragraph::new(format!("Process ID: {}", attached::pid().unwrap()))
    } else {
        Paragraph::new("Scanning for game...")
    }
    .style(theme().fg)
}

fn version_paragraph() -> Paragraph<'static> {
    if let Ok(game_version) = attached::game_version() {
        Paragraph::new(format!("{}", game_version))
    } else {
        Paragraph::new("")
    }
    .style(theme().fg)
    .alignment(Alignment::Right)
}

impl_game_screen!(DarkSouls2Screen, EldenRingScreen);

const HELP_ENTRIES: [Control; 17] = [
    Control::new("hjkl, ← ↑ ↓ → ", "Navigate list"),
    Control::new("ctrl-hjkl, ← ↑ ↓ → ", "Switch list"),
    Control::new("Enter", "Select"),
    Control::new("f", "Search"),
    Control::new("o", "Select game screen"),
    Control::new("a", "Attach options"),
    Control::new("p", "Process selector"),
    Control::new("1-6", "Switch tab"),
    Control::new("tab", "Select next tab"),
    Control::new("backtab", "Select previous tab"),
    Control::new("ctrl-u", "Scroll up"),
    Control::new("ctrl-d", "Scroll down"),
    Control::new("g", "Jump to first entry"),
    Control::new("G", "Jump to last entry"),
    Control::new("f12", "Change theme"),
    Control::new("ctrl-f12", "Memory editor"),
    Control::new("f1", "Help"),
];
