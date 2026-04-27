use anyhow::anyhow;
use ratatui::{
    Frame,
    style::Stylize,
    symbols::Marker,
    widgets::{
        Paragraph, Widget,
        canvas::{Canvas, Circle},
    },
};

use crate::{
    er::{
        chr_ins::{ChrIns, ChrInsExt, chr_ins_from_entity_id},
        player,
    },
    tui::{er::ErInfo, theme::theme},
};

const ELDEN_BEAST_ENTITY_ID: u32 = 19000800;
const ELDEN_BEAST_MAP_ID: u32 = 318767104;
pub struct EldenBeastMap {
    map_valid: bool,
    chr_ins: ChrIns,
}

impl Default for EldenBeastMap {
    fn default() -> Self {
        Self {
            map_valid: false,
            chr_ins: Err(anyhow!("")),
        }
    }
}
impl EldenBeastMap {
    fn draw(&mut self, frame: &mut Frame, er: &ErInfo) {
        let map = er.player_ins.block_id().unwrap_or_default();
        let correct_map = map == ELDEN_BEAST_MAP_ID;
        if self.map_valid && !correct_map {
            self.map_valid = false
        }
        if !self.map_valid && correct_map {
            self.chr_ins = chr_ins_from_entity_id(ELDEN_BEAST_ENTITY_ID);
            self.map_valid = true
        }
        if self.map_valid {
            frame.render_widget(self.stars_cooldown(), frame.area());
            frame.render_widget(Self::arena(&self.chr_ins), frame.area());
        } else {
            frame.render_widget(Self::not_loaded_paragraph(), frame.area());
        }
    }

    fn stars_cooldown(&self) -> impl Widget {
        let cooldown = self.chr_ins.get_lua_timers().unwrap_or_default()[2];
        Paragraph::new(format!("Elden Stars Cooldown: {}", cooldown as i32))
            .fg(theme().fg)
    }

    fn not_loaded_paragraph() -> impl Widget {
        Paragraph::new("Waiting for player to enter the Stone Platform...")
            .fg(theme().fg)
    }

    fn arena(eb_chr_ins: &ChrIns) -> impl Widget {
        let player_coords = player::map_coords().unwrap_or_default();
        let eb_coords = eb_chr_ins.map_coords().unwrap_or_default();
        Canvas::default()
            .background_color(theme().bg)
            .x_bounds([31.0, 371.0])
            .y_bounds([-800.0, -460.0])
            .marker(Marker::Braille)
            .paint(move |ctx| {
                ctx.draw(&Circle {
                    x: 201.555,
                    y: -630.225,
                    radius: 149.395,
                    color: theme().fg,
                });
                ctx.layer();
                ctx.draw(&Circle {
                    x: eb_coords[0].into(),
                    y: eb_coords[2].into(),
                    radius: 20.0,
                    color: theme().warning,
                });
                ctx.draw(&Circle {
                    x: player_coords[0].into(),
                    y: player_coords[2].into(),
                    radius: 5.0,
                    color: theme().error,
                });
            })
    }
}
