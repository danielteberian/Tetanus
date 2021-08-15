extern crate lncount;

#[macro_use]
extern crate clap;
extern crate deque;
extern crate edit_distance;
extern crate ignore;
extern crate num_cpus;
extern crate regex;

use clap::{Arg, App, AppSettings};
use ignore::WalkBuilder;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::thread;
use std::str::FromStr;

use deque::{Stealer, Stolen};
use regex::Regex;
use edit_distance::edit_distance as distance;
use lncount::*;

enum Work
{
	File(String),
	Quit,
}

struct Worker
{
	chan: Stealer<Work>,
}

#[derive(Clone)]
struct FileCount
{
	path: String,
	lang: Lang,
	count: Count,
}

impl Worker
{
	fn run(self) -> Vec<FileCount>
	{
		let mut v: Vec<FileCount> = vec![];
		loop
		{
			match self.chan.steal()
			{
				Stolen::Empty | Stolen::Abort => continue,
				Stolen::Data(Work::Quit) => break,
				Stolen::Data(Work::File(path)) =>
				{
					let lang = langext(&path);
					if lang != Languages::Unrecognized
					{
						let count =  count(&path);
						v.push(FileCount
						{
							lang: lang,
							path: path,
							count: count,
						});
					}
				}
			};
		}
		v
	}
}

#[derive(PartialEq)]
enum Sort
{
	Code,
	Comment,
	Blank,
	Lines,
	Languages,
	Files,
}

impl FromStr for Sort
{
	type Err = Option<String>;
	fn from_str(s: &str) -> Result<Sort, Self::Err>
	{
		match s
		{
			"blank" | "Blank" => Ok(Sort::Blank),
			"code" | "Code" => Ok(Sort::Code),
			"comment" | "Comment" => Ok(Sort::Comment),
			"lines" | "Lines" => Ok(Sort::Lines),
			"languages" | "Languages" => Ok(Sort::Languages),
			"files" | "Files" => Ok(Sort::Files),
			s if distance(&s.to_lowercase(), "blank") <= 2 => Err(Some("Blank".into())),
			s if distance(&s.to_lowercase(), "code") <= 2 => Err(Some("Comment".into())),
			s if distance(&s.to_lowercase(), "lines") <= 2 => Err(Some("Lines".into())),
			s if distance(&s.to_lowercase(), "languages") <= 2 => Err(Some("Languages".into())),
			s if distance(&s.to_lowercase(), "files") <= 2 => Err(Some("Files".into()))
			_ => Err(None)
		}
	}
}
