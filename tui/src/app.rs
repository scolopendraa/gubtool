use crate::{
    attach_options::AttachOptions,
    darksouls2_screen::{self, DarkSouls2},
    eldenring_screen::{self, EldenRing},
    event::{AnyhowExt, Event, InfoType, ResultExt, send_event, start_event_loop_thread},
    game_screen_selector::GameScreenSelector,
    help,
    input::{fuzzy_finder::FuzzyFinder, input_prompt::InputPrompt, multi_fuzzy_finder::MultiFuzzyFinder},
    memory_viewer_screen::MemoryViewerScreen,
    process_selector::ProcessSelector,
    spawn_task,
    theme::{THEME, ThemeSelector, theme},
    ui_state::UiState,
};
use color_eyre::eyre::Result;
use config::Config;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gubtool_core::{
    attached::{self, is_attached},
    game_version::Game,
};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Clear, Paragraph},
};
use ratatui_themes::ThemeName;
use std::{sync::RwLock, time::Duration};

pub struct App {
    running: bool,
    current_screen: CurrentScreen,
    pub game_screen: Game,
    show_info: bool,
    info_message: String,
    info_type: InfoType,
    block_inputs: bool,
    input: InputPrompt,
    fuzzy_finder: FuzzyFinder,
    multi_fuzzy_finder: MultiFuzzyFinder,

    pub theme: ThemeName,
    theme_selector: ThemeSelector,
    process_selector: ProcessSelector,
    game_screen_selector: GameScreenSelector,
    pub attach_options: AttachOptions,
    pub memory_viewer_screen: MemoryViewerScreen,

    pub elden_ring: EldenRing,
    pub dark_souls_2: DarkSouls2,
}

impl App {
    pub fn new() -> App {
        App {
            running: true,
            game_screen: Game::EldenRing,
            current_screen: CurrentScreen::Main,
            show_info: false,
            info_message: "".to_string(),
            info_type: InfoType::SysError,
            block_inputs: false,
            input: InputPrompt::default(),
            fuzzy_finder: FuzzyFinder::default(),
            multi_fuzzy_finder: MultiFuzzyFinder::default(),

            theme: ThemeName::TokyoNight,
            theme_selector: ThemeSelector::new(),
            process_selector: ProcessSelector::new(),
            game_screen_selector: GameScreenSelector::new(),
            attach_options: AttachOptions::new(),
            memory_viewer_screen: MemoryViewerScreen::new(),

            elden_ring: EldenRing::new(),
            dark_souls_2: DarkSouls2::new(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        UiState::apply(&mut self);
        THEME.set(RwLock::new(self.theme.palette())).unwrap();
        let rx = start_event_loop_thread();

        self.try_auto_attach();

        while self.running {
            terminal.draw(|frame| Self::draw(&mut self, frame))?;

            match rx.recv()? {
                Event::Key(key) => {
                    Self::handle_keys(&mut self, key)
                }
                Event::Info((text, info_type)) => {
                    self.info_message = text;
                    self.info_type = info_type;
                    self.show_info = true;
                }
                Event::BackgroundTick => {
                    if !is_attached() {
                        self.try_auto_attach();
                    } else {
                        if let Some(detached_game) = self.process_selector.manager.detach_if_invalid() {
                            send_event(Event::Detach(detached_game));
                            continue;
                        }
                        match self.game_screen {
                            Game::EldenRing => self.elden_ring.background_tick(),
                            Game::DarkSouls2 => self.dark_souls_2.background_tick(),
                        }
                    }
                }
                Event::RenderTick => {
                    if is_attached() {
                        match self.game_screen {
                            Game::EldenRing => self.elden_ring.render_tick(),
                            Game::DarkSouls2 => self.dark_souls_2.render_tick(),
                        }
                    }
                }
                Event::Attach => {
                    if let Some(game) = attached::game() {
                        self.game_screen = game;
                        let _ = UiState::update(|c| c.global.game_screen = game );

                        let time_to_wait = 5.0 - attached::uptime();

                        spawn_task! {
                            if time_to_wait > 0.1 {
                                tokio::time::sleep(Duration::from_secs_f64(time_to_wait)).await;
                            }
                            send_event(Event::ApplyAttach);
                        }
                    }
                }
                Event::ApplyAttach => {
                    match attached::game() {
                        Some(Game::EldenRing) => {
                            self.attach_options.manager.attach(Game::EldenRing).send_error();
                            self.elden_ring.on_attach()
                        }
                        Some(Game::DarkSouls2) => {
                            self.attach_options.manager.attach(Game::DarkSouls2).send_error();
                            self.dark_souls_2.on_attach()
                        }
                        None => Ok(()),
                    }
                    .send_error()
                }
                Event::Detach(game) => {
                    match game {
                        Game::EldenRing => self.elden_ring.on_unattach(),
                        Game::DarkSouls2 => self.dark_souls_2.on_unattach(),
                    }
                }
                Event::BlockInputs(state) => {
                    self.block_inputs = state;
                }
                Event::Input((prompt, sender, type_id)) => {
                    self.input.show(prompt, sender, type_id)
                }
                Event::Search((entries, sender)) => {
                    self.fuzzy_finder.show(entries, sender)
                }
                Event::MultiSearch((entries, sender)) => {
                    self.multi_fuzzy_finder.show(entries, sender)
                }
                Event::AppState(closure) => {
                    closure(&mut self)
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let background = Block::default().bg(theme().bg);
        frame.render_widget(background, frame.area());

        let constraints = if self.show_info {
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

        if self.show_info {
            let style = match self.info_type {
                InfoType::SysError => theme().error,
                InfoType::GameError => theme().warning,
                InfoType::Success => theme().success,
                InfoType::Warning => theme().warning,
            };
            let info_paragraph = Paragraph::new(self.info_message.to_string()).style(style);
            frame.render_widget(info_paragraph, layout[2]);
        }

        match self.game_screen {
            Game::EldenRing => self.elden_ring.draw(frame, layout[1]),
            Game::DarkSouls2 => self.dark_souls_2.draw(frame, layout[1]),
        }

        match self.current_screen {
            CurrentScreen::ProcessSelection => {
                self.process_selector.draw(frame)
            }
            CurrentScreen::ThemeSelection => {
                self.theme_selector.draw(frame, &self.theme)
            }
            CurrentScreen::GameScreenSelection => {
                self.game_screen_selector.draw(frame)
            }
            CurrentScreen::AttachOptions => {
                self.attach_options.draw(frame, &self.game_screen)
            }
            CurrentScreen::MemoryViewer => {
                self.memory_viewer_screen.draw(frame, layout[1]);
            },
            CurrentScreen::Help => {
                help::draw(frame)
            }
            CurrentScreen::Debug => {
                frame.render_widget(Clear, frame.area());
                frame.render_widget(self.dbg_paragraph(), frame.area());
            }
            _ => (),
        }
        self.input.draw_popup_checked(frame);
        self.fuzzy_finder.draw_checked(frame);
        self.multi_fuzzy_finder.draw_checked(frame);
    }

    fn handle_keys(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('c') &&
        key.modifiers == KeyModifiers::CONTROL {
            self.running = false
        }
        if self.show_info {
            self.show_info = false;
        }
        if self.input.show {
            self.input.handle_keys(key);
            return;
        }
        if self.fuzzy_finder.show {
            self.fuzzy_finder.handle_keys(key);
            return;
        }
        if self.multi_fuzzy_finder.show {
            self.multi_fuzzy_finder.handle_keys(key);
            return;
        }

        match self.current_screen {
            CurrentScreen::ProcessSelection => {
                self.process_selector.handle_keys(key, &mut self.current_screen)
            },
            CurrentScreen::GameScreenSelection => {
                self.game_screen_selector.handle_keys(key, &mut self.game_screen, &mut self.current_screen)
            },
            CurrentScreen::ThemeSelection => {
                self.theme_selector.handle_keys(key, &mut self.theme, &mut self.current_screen)
            },
            CurrentScreen::AttachOptions => {
                self.attach_options.handle_keys(key, &self.game_screen, &mut self.current_screen)
            },
            CurrentScreen::MemoryViewer => {
                self.memory_viewer_screen.handle_keys(key, &mut self.current_screen)
            },
            CurrentScreen::Help | CurrentScreen::Debug => {
                self.current_screen = CurrentScreen::Main
            },
            CurrentScreen::Main => {
                match self.game_screen {
                    Game::EldenRing => self.elden_ring.handle_keys(key, self.block_inputs),
                    Game::DarkSouls2 => self.dark_souls_2.handle_keys(key, self.block_inputs),
                }
                if self.block_inputs {
                    return;
                }
                match (key.code, key.modifiers) {
                    (KeyCode::Char('a'), _) => self.current_screen = CurrentScreen::AttachOptions,
                    (KeyCode::F(1), _) => self.current_screen = CurrentScreen::Help,
                    (KeyCode::Char('p'), _) => self.current_screen = {
                        self.process_selector.manager.refresh();
                        self.process_selector.table.select(Some(0));
                        CurrentScreen::ProcessSelection
                    },
                    (KeyCode::Char('o'), _) => self.current_screen = CurrentScreen::GameScreenSelection,
                    (KeyCode::F(12), KeyModifiers::CONTROL) => self.current_screen = CurrentScreen::MemoryViewer,
                    (KeyCode::F(12), _) => self.current_screen = CurrentScreen::ThemeSelection,
                    #[cfg(debug_assertions)]
                    (KeyCode::F(5), KeyModifiers::CONTROL) => self.current_screen = CurrentScreen::Debug,
                    _ => ()
                }
            }
        }
    }

    fn try_auto_attach(&mut self) {
        if let Some(result) = self.process_selector.manager.try_auto_attach() {
            result.send_error();
            send_event(Event::Attach);
        }
    }

    fn pid_paragraph(&self) -> Paragraph<'static> {
        if is_attached() {
            Paragraph::new(format!("Process ID: {}", attached::pid().unwrap()))
        } else {
            Paragraph::new("Scanning for game...")
        }.style(theme().fg)
    }
    fn version_paragraph(&self) -> Paragraph<'static> {
        if let Some(game_version)  = attached::game_version() {
            Paragraph::new(format!("{}", game_version))
        } else {
            Paragraph::new("")
        }.style(theme().fg)
            .alignment(Alignment::Right)
    }

    fn dbg_paragraph(&self) -> Paragraph<'static> {
        let mut debug_info = vec![
            format!("comm: {}", attached::comm()),
            format!("exe_path: {}", attached::path().display()),
            format!("module_base: {:#X}", attached::module_base()),
            format!("is 32 bit: {}", attached::is_32()),
            format!("process uptime: {:.1}", attached::uptime()),
            format!("\n"),
        ];

        match self.game_screen {
            Game::DarkSouls2 => debug_info.append(&mut darksouls2_screen::dbg_lines()),
            Game::EldenRing => debug_info.append(&mut eldenring_screen::dbg_lines()),
        }
        let lines: Vec<Line> = debug_info.iter().map(|f| Line::raw(f.to_string())).collect();
        Paragraph::new(Text::from(lines))
    }
}

#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    Help,
    ProcessSelection,
    ThemeSelection,
    GameScreenSelection,
    AttachOptions,
    MemoryViewer,
    Debug,
}