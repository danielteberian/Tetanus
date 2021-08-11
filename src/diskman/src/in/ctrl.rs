use ::tui::backend::Backend;
use crossterm::event::Event;
use crossterm::event::KeyModifiers;
use crossterm::event::{read, KeyCode, KeyEvent};

use crate::state::FileToDelete;
use crate::App;

#[derive(Clone)]
pub struct TerminalEvents;

impl Iterator for TerminalEvents
{
	type Item = Event;
	fn next(&mut self) -> Option<Event>
	{
		Some(read().unwrap())
	}
}

macro_rules! key
{
	(char $x:expr) =>
	{
		Event::Key(KeyEvent
		{
			code: KeyCode::Char($x),
			modifiers: KeyModifiers::NONE,
		})
	};

	(shift $x:expr) =>
	{
		Event::Key(KeyEvent
		{
			code: KeyCode::Char($x),
			modifiers: KeyModifiers::SHIFT,
		})
	};
