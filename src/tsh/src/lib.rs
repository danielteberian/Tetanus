#![allow(dead_code)]
#![allow(unknown_lints)]
#![feature(tool_lints)]

extern crate errno;
extern crate exec;
extern crate glob;
extern crate libc;
extern crate linefeed;
extern crate nix;
extern crate regex;
extern crate rusqlite;
extern crate chrono;

#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod tp;
#[macro_use]
mod tools;

mod bltin;
mod calc;
mod core;
mod execute;
mod hist;
mod jobc;
mod libraries;
mod parser;
mod rcfile;
mod script;
mod shell;
mod sig;

pub use crate::types::CommandResult;

pub fn cmd_to_tokens(cmd: &str) -> Vec<(String, String)>
{
	return parsers::parser_line::cmd_to_tokens(cmd);
}

pub fn run(line: &str) -> CommandResult
{
	execute::run(line)
}
