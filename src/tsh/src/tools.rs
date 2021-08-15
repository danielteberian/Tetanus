use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::io::IntoRawFd;
use std::path::{Path, PathBuf};

use chrono::prelude::{Local, Datelike, Timelike};
use libc;
use regex::Regex;

use crate::execute;
use crate::libraries::re::re_cont;
use crate::parse;
use crate::shell;

macro_rules! println_stderr
{
		($fmt:expr) => (match writeln!(&mut ::std::io::stderr(), $fmt)
		{
			Ok(_) => {}
			Err(e) => println!("[ERR] FAILED TO WRITE TO STDERR: {:?}", e)
		}
	);
		($fmt:expr, $($arg:tt)*) => (match writeln!(&mut ::std::io::stderr(), $fmt, $($arg)*)
		{
			Ok(_) => {}
			Err(e) => println!("[ERR] FAILED TO WRITE TO STDERR: {:?}", e)
		}
	);
}
