use linefeed::Interface;
use linefeed::terminal::DefaultTerminal;
use rusqlite::Connection as Connec;
use rusqlite::Error::SqliteFailure;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::shell;
use crate::tools::{self, clog};

fn initdb(hfile: &str, htable: &str)
{
	let path = Path::new(hfile);
	if !path.exists()
	{
		let _parent;
		match path.parent()
		{
			Some(x) => _parent = x,
			None =>
			{
				println_stderr!("[ERR] HISTINIT - NO PARENT");
				return;
			}
		}
		let parent;
		match _parent.to_str()
		{
			Some(x) => parent = x,
			None =>
			{
				println_stderr!("[ERR] PARENT TO_STR IS NONE");
				return;
			}
		}
		match fs::create_dir_all(parent)
		{
			Ok(_) => {}
			Err(e) =>
			{
				println_stderr!("[ERR] HISTDIR CREATION ERROR: {}", e);
				return;
			}
		}
