#![allow(dead_code)]
#![allow(clippy::approx_constant)]
#![allow(clippy::single_match)]
#![allow(clippy::type_complexity)]

mod cli;
mod config;
mod core;
mod offsets;
mod resources;
mod tui;

fn main() -> anyhow::Result<()> {
    cli::run()
}
