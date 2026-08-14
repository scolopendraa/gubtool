use {
    crate::theme::theme,
    eldenring::{chr_ins::ChrIns, player},
    ratatui::{
        Frame,
        style::Stylize,
        symbols::Marker,
        widgets::{
            Paragraph,
            Widget,
            canvas::{Canvas, Circle},
        },
    },
};

const ELDEN_BEAST_ENTITY_ID: u32 = 19000800;
const ELDEN_BEAST_MAP_ID: u32 = 318767104;

#[derive(Default)]
pub(super) struct EldenBeastMap {
    map_valid: bool,
    chr_ins:   Option<ChrIns>,
}

impl EldenBeastMap {
    fn draw(&mut self, frame: &mut Frame) {
        let map = player::player()
            .chr_ins()
            .and_then(|chr| chr.block_id())
            .unwrap_or_default();

        let correct_map = map == ELDEN_BEAST_MAP_ID;
        if self.map_valid && !correct_map {
            self.map_valid = false;
            self.chr_ins = None;
        }
        if !self.map_valid && correct_map {
            self.chr_ins = Some(ChrIns::from_entity_id(ELDEN_BEAST_ENTITY_ID).unwrap());
            self.map_valid = true
        }
        if self.map_valid {
            frame.render_widget(self.stars_cooldown(), frame.area());
            frame.render_widget(Self::arena(self), frame.area());
        } else {
            frame.render_widget(Self::not_loaded_paragraph(), frame.area());
        }
    }

    fn stars_cooldown(&mut self) -> impl Widget {
        let cooldown = if let Some(chr_ins) = self.chr_ins.as_mut() {
            chr_ins.get_lua_timers().unwrap_or_default()[2]
        } else {
            0.0
        };
        Paragraph::new(format!("Elden Stars Cooldown: {}", cooldown as i32)).fg(theme().fg)
    }

    fn not_loaded_paragraph() -> impl Widget {
        Paragraph::new("Waiting for player to enter the Stone Platform...").fg(theme().fg)
    }

    fn arena(&mut self) -> impl Widget {
        let player_coords = player::map_coords().unwrap_or_default();
        let eb_coords = if let Some(chr_ins) = self.chr_ins.as_mut() {
            chr_ins.map_coords().unwrap_or_default()
        } else {
            Default::default()
        };
        Canvas::default()
            .background_color(theme().bg)
            .x_bounds([31.0, 371.0])
            .y_bounds([-800.0, -460.0])
            .marker(Marker::Braille)
            .paint(move |ctx| {
                ctx.draw(&Circle {
                    x:      201.555,
                    y:      -630.225,
                    radius: 149.395,
                    color:  theme().fg,
                });
                ctx.layer();
                ctx.draw(&Circle {
                    x:      eb_coords[0].into(),
                    y:      eb_coords[2].into(),
                    radius: 20.0,
                    color:  theme().warning,
                });
                ctx.draw(&Circle {
                    x:      player_coords[0].into(),
                    y:      player_coords[2].into(),
                    radius: 5.0,
                    color:  theme().error,
                });
            })
    }
}
