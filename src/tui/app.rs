use crate::{
    config::{Config, ui_state::UiState},
    core::attach::{self, ATTACHED_PROCESS, Game, GameProcess, game, pid, version},
    tui::{
        ds2::DarkSouls2,
        er::EldenRing,
        event::{Event, ResultExt, start_event_loop_thread},
        fuzzy_finder::FuzzyFinder,
        game_screen_selector::GameScreenSelector,
        help,
        input::Input,
        process_selector::ProcessSelector,
        theme::{THEME, ThemeSelector, theme},
    },
};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    widgets::{Block, Paragraph},
};
use ratatui_themes::ThemeName;
use std::{sync::RwLock, thread, time::Duration};

pub struct App {
    running: bool,
    current_screen: CurrentScreen,
    pub game_screen: Game,
    attached: bool,
    show_err: bool,
    err_message: String,

    pub theme: ThemeName,
    theme_selector: ThemeSelector,
    input: Input,
    input_enter_fn: fn(String, &mut App),
    pub fuzzy_finder: FuzzyFinder,
    fuzzy_picker: fn(&mut App),
    process_selector: ProcessSelector,
    game_screen_selector: GameScreenSelector,

    pub elden_ring: EldenRing,
    pub dark_souls_2: DarkSouls2,
}

impl App {
    pub fn new() -> App {
        App {
            running: true,
            game_screen: Game::EldenRing,
            current_screen: CurrentScreen::Game,
            attached: false,
            show_err: false,
            err_message: "".to_string(),

            theme: ThemeName::default(),
            theme_selector: ThemeSelector::new(),
            input: Input::default(),
            input_enter_fn: |_,_| {},
            fuzzy_finder: FuzzyFinder::default(),
            fuzzy_picker: |_| {},
            process_selector: ProcessSelector::new(),
            game_screen_selector: GameScreenSelector::new(),

            elden_ring: EldenRing::new(),
            dark_souls_2: DarkSouls2::new(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        UiState::apply(&mut self);
        THEME.set(RwLock::new(self.theme.palette())).unwrap();
        let rx = start_event_loop_thread();

        self.try_attach(None, false).send_error();

        while self.running {
            terminal.draw(|frame| Self::draw(&mut self, frame))?;

            match rx.recv()? {
                Event::Key(key) => {
                    Self::handle_keys(&mut self, key)
                }
                Event::Error(err) => {
                    self.err_message = err;
                    self.show_err = true;
                }
                Event::BackgroundTick => {
                    if !self.attached {
                        self.try_attach(None, true).send_error()
                    } else if !attach::is_pid_valid() {
                        self.detach()
                    }
                    if self.attached && game() == self.game_screen {
                        match self.game_screen {
                            Game::EldenRing => self.elden_ring.background_tick(),
                            Game::DarkSoulsII => self.dark_souls_2.background_tick(),
                        }
                    }
                }
                Event::RenderTick => {
                    if self.attached && game() == self.game_screen {
                        match self.game_screen {
                            Game::EldenRing => self.elden_ring.render_tick(),
                            Game::DarkSoulsII => self.dark_souls_2.render_tick(),
                        }
                    }
                }
                Event::Search((list, f)) => {
                    self.fuzzy_finder.entries = Some(list);
                    self.fuzzy_finder.update_matches();
                    self.fuzzy_picker = f;
                    self.current_screen = CurrentScreen::Search
                }
                Event::Input(f) => {
                    self.input_enter_fn = f;
                    self.current_screen = CurrentScreen::Input;
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let background = Block::default().bg(theme().bg);
        frame.render_widget(background, frame.area());

        let constraints = if self.show_err || self.current_screen == CurrentScreen::Input {
            vec![
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ]
        } else {
            vec![
                Constraint::Length(1),
                Constraint::Fill(1),
            ]
        };

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(frame.area());

        let [pid_area, version_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Max(25),
                Constraint::Fill(1)
            ])
            .areas(layout[0]);

        frame.render_widget(self.pid_paragraph(), pid_area);
        frame.render_widget(self.version_paragraph(), version_area);

        if self.show_err {
            let err_paragraph = Paragraph::new(self.err_message.to_string()).style(theme().error);
            frame.render_widget(err_paragraph, layout[2]);
        } else if self.current_screen == CurrentScreen::Input {
            let input = Paragraph::new(self.input.to_string()).style(theme().fg);
            self.input.set_cursor(frame, layout[2]);
            frame.render_widget(input, layout[2]);
        }

        match self.game_screen {
            Game::EldenRing => self.elden_ring.draw(frame, layout[1]),
            Game::DarkSoulsII => self.dark_souls_2.draw(frame, layout[1]),
        }

        match self.current_screen {
            CurrentScreen::Search => {
                self.fuzzy_finder.draw(frame)
            }
            CurrentScreen::ProcessSelection => {
                self.process_selector.draw(frame)
            }
            CurrentScreen::ThemeSelection => {
                self.theme_selector.draw(frame, &self.theme)
            }
            CurrentScreen::GameScreenSelection => {
                self.game_screen_selector.draw(frame)
            }
            CurrentScreen::Help => {
                help::draw(frame)
            }
            CurrentScreen::Debug => {
            }
            _ => (),
        }
    }

    fn handle_keys(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('c') &&
        key.modifiers == KeyModifiers::CONTROL {
            self.running = false
        }
        if self.show_err {
            self.show_err = false;
        }
        match self.current_screen {
            CurrentScreen::ProcessSelection => {
                if let Some(process) = self.process_selector.handle_keys(key, &mut self.current_screen) {
                    self.try_attach(Some(process), false).send_error()
                }
            },
            CurrentScreen::GameScreenSelection => {
                self.game_screen_selector.handle_keys(key, &mut self.game_screen, &mut self.current_screen)
            },
            CurrentScreen::ThemeSelection => {
                self.theme_selector.handle_keys(key, &mut self.theme, &mut self.current_screen)
            },
            CurrentScreen::Help | CurrentScreen::Debug => {
                self.current_screen = CurrentScreen::Game
            },
            CurrentScreen::Search => {
                match key.code {
                    KeyCode::Enter => {
                        (self.fuzzy_picker)(self);
                        self.fuzzy_finder.reset();
                        self.current_screen = CurrentScreen::Game;
                    }
                    KeyCode::Esc => {
                        self.fuzzy_finder.reset();
                        self.current_screen = CurrentScreen::Game;
                    }
                    _ => {
                        self.fuzzy_finder.handle_keys(key)
                    }
                }
            }
            CurrentScreen::Input => {
                match key.code {
                    KeyCode::Enter => {
                        let text = self.input.text.clone();
                        (self.input_enter_fn)(text, self);
                        self.input.set_text("");
                        self.current_screen = CurrentScreen::Game;
                    }
                    KeyCode::Esc => {
                        self.input.set_text("");
                        self.current_screen = CurrentScreen::Game;
                    }
                    _ => {
                        self.input.handle_keys(key);
                    }
                }
            },
            CurrentScreen::Game => {
                match self.game_screen {
                    Game::EldenRing => self.elden_ring.handle_keys(key),
                    Game::DarkSoulsII => self.dark_souls_2.handle_keys(key),
                }
            }
        }
        match (key.code, key.modifiers) {
            (KeyCode::F(1), _) => self.current_screen = CurrentScreen::Help,
            (KeyCode::F(2), _) => self.current_screen = {
                self.process_selector.update_processes();
                CurrentScreen::ProcessSelection
            },
            (KeyCode::F(3), _) => self.current_screen = CurrentScreen::GameScreenSelection,
            (KeyCode::F(12), _) => self.current_screen = CurrentScreen::ThemeSelection,
            _ => ()
        }
    }

    fn try_attach(&mut self, process: Option<GameProcess>, wait: bool) -> anyhow::Result<()> {
        let mut result = Ok(());
        if let Some(process) = process {
            if let Err(err) = attach::attach_to_process(process) {
                result = Err(err)
            }
        } else {
            match attach::auto_attach() {
                Ok(val) => if !val { return Ok(()) },
                Err(err) => result = Err(err),
            }
        }
        if wait {
            thread::sleep(Duration::from_millis(500));
        }
        self.attached = true;
        self.game_screen = game();
        let _ = UiState::update(|c| c.global.game_screen = game() );

        if let Err(err) = match self.game_screen {
            Game::EldenRing => self.elden_ring.on_attach(),
            Game::DarkSoulsII => Ok(()),
        } && result.is_ok() {
            result = Err(err)
        }
        result
    }

    fn detach(&mut self) {
        unsafe {
            ATTACHED_PROCESS = GameProcess::detached()
        }
        match game() {
            Game::EldenRing => self.elden_ring.on_unattach(),
            Game::DarkSoulsII => (),
        }
        self.attached = false;
    }

    fn pid_paragraph(&self) -> Paragraph<'static> {
        if self.attached {
            Paragraph::new(format!("Process ID: {}", pid()))
        } else {
            Paragraph::new("Scanning for game...")
        }.style(theme().fg)
    }
    fn version_paragraph(&self) -> Paragraph<'static> {
        if self.attached {
            Paragraph::new(format!("{}", version()))
        } else {
            Paragraph::new("")
        } .style(theme().fg)
            .alignment(Alignment::Right)
    }
}

#[derive(PartialEq)]
pub enum CurrentScreen {
    Game,
    Search,
    Input,
    Help,
    ProcessSelection,
    ThemeSelection,
    GameScreenSelection,
    Debug,
}