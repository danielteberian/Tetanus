mod canv;

use std::fs;
use std::env;
use regex::Regex;

use cursive::views::{Button, Dialog, LinearLayout, Panel, EditView, ListView, TextView, SliderView, ViewRef};
use cursive::theme::{BorderStyle, Color, Theme, Palette, PaletteColor::*};
use cursive::traits::*;
use cursive::event::Event;
use cursive::traits::Identifiable;
use cursive::Cursive;
use cursive::Vec2;
use cursive::backends;
use cursive_buffered_backend::BufferedBackend;

#[cfg(target_os = "windows)]
fn bkend() -> Box<BufferedBackend>
{
	let cterm_bkend = backends::crossterm::Backend::init().unwrap();
	let buf_bkend = cursive_buffered_backend::BufferedBackend::new(crossterm_backend);
	Box::new(buf_bkend)
}
