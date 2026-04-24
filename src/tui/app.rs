use std::{
    time::Duration,
    sync::{RwLock, mpsc},
    fmt, thread,
};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, ListState, Paragraph, TableState, Tabs as RatatuiTabs, Wrap},
    DefaultTerminal, Frame, symbols,
};
use ratatui_themes::ThemeName;
use crate::{
    config::{Preferences, UiState},
    core::{
        attach::{self, ATTACHED_PROCESS, GameProcess},
        chr_ins::{ChrIns, ChrInsExt},
        game_state::GameStateHandler,
        player::{self, player_ins},
        target::{self, target_ins},
    },
    tui::{
        elden_beast_map::{self, EldenBeastMap},
        event::{Event, start_event_loop_thread},
        fuzzy_finder::{self, FuzzyFinder, Picker},
        input::Input,
        input_handler::{self, InputField},
        ResultExt, block, help, process_selection,
        tabs::{
            event_tab::{self, EventTab},
            items_tab::{self, ItemTab},
            player_tab::{self, PlayerTab},
            target_tab::{self, TargetTab},
            travel_tab::{self, TravelTab},
            utility_tab::{self, UtilityTab}
        },
        theme::{self, THEME, theme},
    }
};

pub struct App {
    running: bool,
    current_tab: Tabs,
    pub current_screen: CurrentScreen,
    pub attached: bool,

    pub player_ins: ChrIns,
    pub target_ins: ChrIns,

    pub theme: ThemeName,
    pub sender: mpsc::Sender<Event>,
    pub input: Input,
    pub input_field: InputField,
    pub fuzzy_finder: FuzzyFinder,
    pub game_state: GameStateHandler,

    show_help: bool,
    show_dbg: bool,
    show_err: bool,
    err_message: String,

    pub player: PlayerTab,
    pub target: TargetTab,
    pub item: ItemTab,
    pub utility: UtilityTab,
    pub travel: TravelTab,
    pub event: EventTab,

    pub elden_beast_map: EldenBeastMap,

    pub theme_list: ListState,
    pub processes_state: TableState,
    pub available_processes: Vec<GameProcess>,
}

impl App {
    pub fn new() -> App {
        App {
            running: true,
            current_tab: Tabs::Player,
            current_screen: CurrentScreen::Tab,
            player_ins: player_ins(),
            target_ins: target_ins(),
            sender: { let (tx, _rx) = mpsc::channel(); tx },
            input: Input::default(),
            input_field: InputField::GiveRunes,
            fuzzy_finder: FuzzyFinder::default(),
            game_state: GameStateHandler::new(),
            theme: ThemeName::default(),

            show_help: false,
            show_dbg: false,
            show_err: false,
            err_message: "".to_string(),
            attached: false,

            player: PlayerTab::new(),
            target: TargetTab::new(),
            travel: TravelTab::new(),
            item: ItemTab::new(),
            utility: UtilityTab::new(),
            event: EventTab::new(),

            elden_beast_map: EldenBeastMap::default(),
            theme_list: ListState::default().with_selected(Some(0)),
            processes_state: TableState::default().with_selected(0),
            available_processes: Vec::new(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        UiState::apply(&mut self);
        THEME.set(RwLock::new(self.theme.palette())).unwrap();
        let (tx, rx) = start_event_loop_thread();
        self.sender = tx;

        self.try_attach(None, false).send_error(self.sender.clone());

        while self.running {
            terminal.draw(|frame| Self::draw(&mut self, frame))?;

            match rx.recv()? {
                Event::Key(key) => {
                    Self::handle_keys(&mut self, key);
                }
                Event::Error(err) => {
                    self.err_message = err;
                    self.show_err = true;
                }
                Event::BackgroundTick => {
                    if !self.attached {
                        self.try_attach(None, true).send_error(self.sender.clone());
                    } else {
                        if !attach::is_pid_valid() {
                            unsafe {
                                ATTACHED_PROCESS = GameProcess::detached()
                            }
                            self.game_state = GameStateHandler::new();
                            self.attached = false;
                        }
                        self.game_state.poll().ok();
                        self.player_ins = player_ins();

                        if self.current_screen == CurrentScreen::ProcessSelection {
                            self.available_processes = attach::get_processes();
                        }
                    }
                }
                Event::RenderTick => {
                    if self.attached {
                        self.target_ins = target_ins();
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let background = Block::default().bg(theme().bg);
        frame.render_widget(background, frame.area());

        if self.show_dbg {
            frame.render_widget(dbg_paragraph(self), frame.area());
            return
        }
        if self.current_screen == CurrentScreen::EldenBeast {
            elden_beast_map::draw(self, frame);
            return
        }
        let constraints = if self.show_err || self.current_screen == CurrentScreen::Input {
            vec![
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(1),
            ]
        } else {
            vec![
                Constraint::Length(1),
                Constraint::Length(3),
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

        let pid_paragraph = if self.attached {
            Paragraph::new(format!("Process ID: {}", unsafe { ATTACHED_PROCESS.pid }))
        } else {
            Paragraph::new("Scanning for game...")
        }.style(theme().fg);

        let version_paragraph = if self.attached {
            Paragraph::new(format!("Version: {}", unsafe { ATTACHED_PROCESS.version }))
        } else {
            Paragraph::new("")
        }
        .style(theme().fg)
        .alignment(Alignment::Right);

        frame.render_widget(pid_paragraph, pid_area);
        frame.render_widget(version_paragraph, version_area);
        frame.render_widget(self.tabs_widget(), layout[1]);

        if self.show_err {
            let err_paragraph = Paragraph::new(self.err_message.to_string()).style(theme().error);
            frame.render_widget(err_paragraph, layout[3]);
        } else if self.current_screen == CurrentScreen::Input {
            let input = Paragraph::new(self.input.to_string()).style(theme().fg);
            self.input.set_cursor(frame, layout[3]);
            frame.render_widget(input, layout[3]);
        }

        match self.current_tab {
            Tabs::Player => player_tab::draw(frame, self, layout[2]),
            Tabs::Target => target_tab::draw(frame, self, layout[2]),
            Tabs::Utility => utility_tab::draw(frame, self, layout[2]),
            Tabs::Items => items_tab::draw(frame, self, layout[2]),
            Tabs::Travel => travel_tab::draw(frame, self, layout[2]),
            Tabs::Event => event_tab::draw(frame, self, layout[2]),
        }

        match self.current_screen {
            CurrentScreen::Search => {
                fuzzy_finder::draw(frame, &mut self.fuzzy_finder)
            }
            CurrentScreen::ProcessSelection => {
                process_selection::draw(frame, self)
            }
            CurrentScreen::ThemeSelection => theme::draw(frame, self),
            _ => (),
        }
        if self.show_help {
            help::draw(frame);
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
        if self.show_dbg {
            self.show_dbg = false;
        }
        if self.show_help {
            self.show_help = false;
        }
        match self.current_screen {
            CurrentScreen::Search => fuzzy_finder::handle_keys(self, key),
            CurrentScreen::Input => input_handler::handle_keys(self, key),
            CurrentScreen::ProcessSelection => process_selection::handle_keys(self, key),
            CurrentScreen::ThemeSelection => theme::handle_keys(self, key),
            CurrentScreen::EldenBeast => elden_beast_map::handle_keys(self, key),
            CurrentScreen::Tab => {
                match (key.code, key.modifiers) {
                    (KeyCode::BackTab, _) => {
                        let tabs_len = Tabs::ARRAY.len() as i64;
                        let val = ((self.current_tab.clone() as i64) + tabs_len - 1) % tabs_len;
                        self.current_tab = Tabs::from(val);
                    }
                    (KeyCode::Tab, _) => {
                        let tabs_len = Tabs::ARRAY.len() as i64;
                        let val = ((self.current_tab.clone() as i64) + tabs_len + 1) % tabs_len;
                        self.current_tab = Tabs::from(val);
                    }
                    (KeyCode::Char('1'), _) => self.current_tab = Tabs::Player,
                    (KeyCode::Char('2'), _) => self.current_tab = Tabs::Target,
                    (KeyCode::Char('3'), _) => self.current_tab = Tabs::Utility,
                    (KeyCode::Char('4'), _) => self.current_tab = Tabs::Items,
                    (KeyCode::Char('5'), _) => self.current_tab = Tabs::Travel,
                    (KeyCode::Char('6'), _) => self.current_tab = Tabs::Event,
                    (KeyCode::F(1), _) => self.show_help = true,
                    (KeyCode::F(2), _) => self.current_screen = CurrentScreen::ProcessSelection,
                    /*
                    (KeyCode::F(11), _) => {
                        self.current_screen = CurrentScreen::EldenBeast;
                        self.elden_beast_map = EldenBeastMap::default()
                    }
                    (KeyCode::F(12), KeyModifiers::CONTROL) => self.show_dbg = true,
                    */
                    (KeyCode::F(12), _) => self.current_screen = CurrentScreen::ThemeSelection,
                    _ => ()
                }
                match self.current_tab {
                    Tabs::Player => player_tab::handle_keys(self, key),
                    Tabs::Target => target_tab::handle_keys(self, key),
                    Tabs::Utility => utility_tab::handle_keys(self, key),
                    Tabs::Items => items_tab::handle_keys(self, key),
                    Tabs::Travel => travel_tab::handle_keys(self, key),
                    Tabs::Event => event_tab::handle_keys(self, key),
                }
            }
        }
    }

    pub fn open_input(&mut self, input_field: InputField) {
        self.input_field = input_field;
        self.current_screen = CurrentScreen::Input;
    }

    pub fn jump_to_entry(&mut self) {
        if let Some(idx) = self.fuzzy_finder.selected_idx()
            && let Some(picker) = self.fuzzy_finder.picker.take()
        {
            picker.jump(idx, self);
        }
    }

    fn tabs_widget(&self) -> RatatuiTabs<'static> {
        let titles: Vec<String> = Tabs::ARRAY.iter().map(|t| t.to_string()).collect();
        RatatuiTabs::new(titles)
            .block(block(None, None))
            .highlight_style(Style::from(theme().accent).bold())
            .select(self.current_tab.clone() as usize)
            .divider(symbols::line::VERTICAL)
    }

    pub fn set_fuzzy_finder(&mut self, picker: Box<dyn Picker>) {
        self.fuzzy_finder.set_picker(picker);
        self.fuzzy_finder.update_matches();
        self.current_screen = CurrentScreen::Search;
    }

    pub fn try_attach(&mut self, process: Option<GameProcess>, wait: bool) -> anyhow::Result<()> {
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
        if let Err(err) = Preferences::apply()
            && result.is_ok() {
                result = Err(err);
        }
        target::install_target_hook().ok();
        self.attached = true;
        self.game_state = GameStateHandler::new();
        result
    }
}

#[derive(PartialEq)]
pub enum CurrentScreen {
    Tab,
    Search,
    Input,
    ProcessSelection,
    ThemeSelection,
    EldenBeast,
}

#[derive(Clone)]
#[derive(PartialEq)]
pub enum Tabs {
    Player,
    Target,
    Utility,
    Items,
    Travel,
    Event,
}

impl Tabs {
    pub const ARRAY: [Tabs; 6] = [
        Self::Player,
        Self::Target,
        Self::Utility,
        Self::Items,
        Self::Travel,
        Self::Event,
    ];
}

impl From<i64> for Tabs {
    fn from(v: i64) -> Self {
        match v {
            0 => Tabs::Player,
            1 => Tabs::Target,
            2 => Tabs::Utility,
            3 => Tabs::Items,
            4 => Tabs::Travel,
            5 => Tabs::Event,
            _ => Tabs::Player,
        }
    }
}

impl fmt::Display for Tabs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Self::Player => "Player",
            Self::Target => "Target",
            Self::Utility => "Utility",
            Self::Items => "Items",
            Self::Travel => "Travel",
            Self::Event => "Events",
        };
        write!(f, "{}", name)
    }
}

fn dbg_paragraph(app: &App) -> Paragraph<'static> {
    let lcoords = app.player_ins.local_coords().unwrap_or_default();
    let mcoords = app.player_ins.map_coords().unwrap_or_default();

    let debug_info = [
        format!("Attached: {}", app.attached),
        format!("Module Handle: {:#X}", unsafe { ATTACHED_PROCESS.module_handle }),
        format!("DLC: {}", app.game_state.dlc),
        format!("Loaded: {}", app.game_state.loaded),
        format!("Player Handle: {:#X}", app.player_ins.handle().unwrap_or_default()),
        format!("Player Map ID: {}", app.player_ins.block_id().unwrap_or_default()),
        format!("Player Local Coords: x:{:.2}, y:{:.2}, z:{:.2}, angle:{:.2}",
            lcoords[0],
            lcoords[1],
            lcoords[2],
            player::map_angle().unwrap_or_default()
        ),
        format!("Player Map Coords: x:{:.2}, y:{:.2}, z:{:.2}",
            mcoords[0],
            mcoords[1],
            mcoords[2],
        ),
        format!("{:?}", app.target_ins.get_lua_timers().unwrap_or_default()),
        format!("Target Entity ID {}", app.target_ins.entity_id().unwrap_or_default()),
    ];

    let lines: Vec<Line> = debug_info.iter().map(|f| Line::raw(f.to_string())).collect();
    Paragraph::new(Text::from(lines))
        .wrap(Wrap { trim:true })
}
