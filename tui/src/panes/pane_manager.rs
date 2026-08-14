use {
    crate::{event::KeyContext, panes::Pane},
    crossterm::event::{KeyCode, KeyModifiers},
    ratatui::{Frame, layout::Rect},
};

pub struct PaneManager {
    panes:        Vec<Box<dyn Pane>>,
    current_pane: usize,
}

impl PaneManager {
    pub fn new(panes: Vec<Box<dyn Pane>>) -> Self {
        Self {
            panes,
            current_pane: 0,
        }
    }

    pub fn get_list_selected(&self, list_idx: usize) -> Option<usize> {
        self.panes[list_idx].selected()
    }

    pub fn set_current_list_idx(&mut self, idx: usize) {
        self.panes[self.current_pane].select(idx)
    }

    pub fn current_command(&self) -> Option<&shared::command::Command> {
        self.panes[self.current_pane].current_command()
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: &[Rect]) {
        for (i, rect) in layout.iter().enumerate() {
            if i == self.current_pane {
                self.panes[i].draw_active(frame, *rect);
            } else {
                self.panes[i].draw_inactive(frame, *rect);
            };
        }
    }

    pub fn handle_keys(&mut self, ctx: &mut KeyContext) {
        let layout: &[Neighbours] = match self.panes.len() {
            2 => &TWO_PANES,
            3 => &THREE_PANES,
            4 => &FOUR_PANES,
            _ => unreachable!("invalid amount of panes"),
        };

        if (ctx.key_with_modifiers(KeyCode::Char('h'), KeyModifiers::CONTROL)
            || ctx.key_with_modifiers(KeyCode::Left, KeyModifiers::CONTROL))
            && let Some(new_pane) = layout[self.current_pane].left
        {
            self.current_pane = new_pane;
        }

        if (ctx.key_with_modifiers(KeyCode::Char('l'), KeyModifiers::CONTROL)
            || ctx.key_with_modifiers(KeyCode::Right, KeyModifiers::CONTROL))
            && let Some(new_pane) = layout[self.current_pane].right
        {
            self.current_pane = new_pane;
        }

        if (ctx.key_with_modifiers(KeyCode::Char('j'), KeyModifiers::CONTROL)
            || ctx.key_with_modifiers(KeyCode::Down, KeyModifiers::CONTROL))
            && let Some(new_pane) = layout[self.current_pane].down
        {
            self.current_pane = new_pane;
        }

        if (ctx.key_with_modifiers(KeyCode::Char('k'), KeyModifiers::CONTROL)
            || ctx.key_with_modifiers(KeyCode::Up, KeyModifiers::CONTROL))
            && let Some(new_pane) = layout[self.current_pane].up
        {
            self.current_pane = new_pane;
        }

        self.panes[self.current_pane].handle_keys(ctx);
    }
}

struct Neighbours {
    left:  Option<usize>,
    right: Option<usize>,
    up:    Option<usize>,
    down:  Option<usize>,
}

const TWO_PANES: [Neighbours; 2] = [
    Neighbours {
        left:  None,
        right: Some(1),
        up:    None,
        down:  None,
    },
    Neighbours {
        left:  Some(0),
        right: None,
        up:    None,
        down:  None,
    },
];

const THREE_PANES: [Neighbours; 3] = [
    Neighbours {
        left:  None,
        right: Some(1),
        up:    None,
        down:  None,
    },
    Neighbours {
        left:  Some(0),
        right: None,
        up:    None,
        down:  Some(2),
    },
    Neighbours {
        left:  Some(0),
        right: None,
        up:    Some(1),
        down:  None,
    },
];

const FOUR_PANES: [Neighbours; 4] = [
    Neighbours {
        left:  None,
        right: Some(1),
        up:    None,
        down:  Some(4),
    },
    Neighbours {
        left:  Some(0),
        right: None,
        up:    None,
        down:  Some(2),
    },
    Neighbours {
        left:  Some(4),
        right: None,
        up:    Some(1),
        down:  None,
    },
    Neighbours {
        left:  None,
        right: Some(3),
        up:    Some(0),
        down:  None,
    },
];
