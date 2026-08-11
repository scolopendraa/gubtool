use crate::{
    event::KeyContext,
    panes::{TableController, TablePane, TableView},
    popup::{Popup, PopupState, centered_popup},
    screen::Screen,
    ui_state::UiState,
};
use config::Config;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{BorderType, Row},
};
use ratatui_themes::{Color, ThemeName, ThemePalette};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::IntoDeserializer};
use std::{iter, sync::RwLock};

pub const HIGHLIGHT_SYMBOL: &'static str = "> ";
pub const BORDER_TYPE: BorderType = BorderType::Rounded;
const TERMINAL_PALETTE: ThemePalette = ThemePalette {
    accent: Color::Blue,
    secondary: Color::Magenta,
    bg: Color::Reset,
    fg: Color::Reset,
    muted: Color::DarkGray,
    selection: Color::Cyan,
    error: Color::Red,
    warning: Color::Yellow,
    success: Color::Green,
    info: Color::Cyan,
};

static GLOBAL_THEME: RwLock<GlobalTheme> = RwLock::new(GlobalTheme::new());

#[derive(Clone, Copy)]
pub enum ThemeChoice {
    Terminal,
    Preset(ThemeName),
}


impl ThemeChoice {
    #[cfg(windows)]
    const PLATFORM_DEFAULT: Self = Self::Preset(ThemeName::TokyoNight);

    #[cfg(unix)]
    const PLATFORM_DEFAULT: Self = Self::Terminal;

    pub const fn palette(self) -> ThemePalette {
        match self {
            Self::Terminal => TERMINAL_PALETTE,
            Self::Preset(theme) => theme.palette(),
        }
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Terminal => "Terminal",
            Self::Preset(theme) => theme.display_name(),
        }
    }

    fn all() -> impl Iterator<Item = Self> {
        iter::once(ThemeChoice::Terminal)
            .chain(ThemeName::all().iter().copied().map(ThemeChoice::Preset))
    }

    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Terminal),
            _ => ThemeName::all().get(index - 1).copied().map(Self::Preset),
        }
    }
}

impl Default for ThemeChoice {
    fn default() -> Self {
        Self::PLATFORM_DEFAULT
    }
}

impl<'de> Deserialize<'de> for ThemeChoice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        if value == "terminal" {
            return Ok(Self::Terminal);
        }

        ThemeName::deserialize(value.into_deserializer()).map(Self::Preset)
    }
}

impl Serialize for ThemeChoice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Terminal => serializer.serialize_str("terminal"),
            Self::Preset(theme) => theme.serialize(serializer),
        }
    }
}

struct GlobalTheme {
    palette: ThemePalette,
    version: usize,
}

impl GlobalTheme {
    const fn new() -> Self {
        Self { palette: ThemeChoice::PLATFORM_DEFAULT.palette(), version: 0 }
    }
}

pub fn theme() -> ThemePalette {
    GLOBAL_THEME.read().unwrap().palette
}

pub fn set_theme(theme: ThemeChoice) {
    let mut global_theme = GLOBAL_THEME.write().unwrap();
    global_theme.palette = theme.palette();
    global_theme.version = global_theme.version.wrapping_add(1);
}

pub fn get_theme_version() -> usize {
    GLOBAL_THEME.read().unwrap().version
}

pub struct ThemeSelector {
    list: TablePane,
    popup: PopupState,
}

impl Popup for ThemeSelector {
    fn screen(&mut self) -> &mut dyn Screen {
        &mut self.list
    }
    fn popup_state(&mut self) -> &mut PopupState {
        &mut self.popup
    }
    fn popup_rect(&self, frame: &mut Frame) -> Rect {
        centered_popup(60, 60, frame.area())
    }
}

struct ThemeList;
impl TableController for ThemeList {
    fn make_table_view(&self) -> TableView {
        let selected_theme = theme();
        let rows = ThemeChoice::all()
            .map(|theme| {
                let name = if selected_theme == theme.palette() {
                    format!("*{}", theme.display_name())
                } else {
                    format!(" {}", theme.display_name())
                };
                Row::new([name])
            })
            .collect();
        TableView::new(rows)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter()
            && let Some(theme) = ThemeChoice::from_index(selected)
        {
            UiState::update(|c| { c.global.theme = theme; }).ok();
            set_theme(theme);
        }
    }
}

impl ThemeSelector {
    pub fn new() -> Self {
        Self {
            list: TablePane::new_static(&ThemeList)
                .with_title("Themes"),
            popup: PopupState::default(),
        }
    }
}
