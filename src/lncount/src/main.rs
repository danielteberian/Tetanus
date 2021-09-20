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

fn main()
{
	let matches = App::new("lncount")
		.global_settings(&[AppSettings::ColoredHelp])
		.version(crate_version!())
		.author("Daniel P. Teberian")
		.about("A utility to count lines in selected files. This is a part of Tetanus.")
		.arg(Arg::with_name("exclude")
			.required(false)
			.multiple(true)
			.long("exclude")
			.value_name("REGEX")
			.takes_value(true)
			.help("I don't know what to write here."))
		.arg(Arg::with_name("include")
			.required(false)
			.multiple(true)
			.long("include")
			.value_name("REGEX")
			.takes_value(true)
			.help("Regex files to be included."))
		.arg(Arg::with_name("files")
			.required(false)
			.long("files")
			.takes_value(false)
			.help("Display statistics about individual files."))
		.arg(Arg::with_name("sort")
			.required(false)
			.long("sort")
			.value_name("COLUMN")
			.takes_value(true)
			.help("Column used to sort"))
		.arg(Arg::with_name("unrestricted")
			.required(false)
			.multiple(true)
			.long("unrestricted")
			.short("u")
			.takes_value(false)
			.help("TODO"))
		.arg(Arg::with_name("target")
			.multiple(true)
			.help("The file(s)/directory/directories to include in the counting process. Multiple arguments can be provided."))
		.get_matches();
	
		let tgt = match matches.values_of("target")
		{
			Some(tgt) => tgt.collect(),
			None => vec!["."]
		};

		let sort: Sort = match matches.value_of("sort")
		{
			Some(string) => match Sort::from_str(string)
			{
				Ok(sort) => sort,
				Err(err) =>
				{
					if let Some(suggestion) = err
					{
						println!("[ERR] INVALID ARGUMENT: '{}'",
							string, suggestion);
					}
					else
					{
						println!("[ERR] INVALID ARGUMENT: '{}'", string);
					}
					return
				},
			},
			None => Sort::Code,
		};

		let by_file: bool = matches.is_present("files");
		if by_file && (sort == Sort::Language || sort == Sort::Files)
		{
			println!("[ERR] CANNOT SORT BY LANGUAGE IF THE --FILES ARGUMENT HAS BEEN INVOKED");
			return
		}

		let (use_ignore, ignore_hidden) = match matches.occurrences_of("unrestricted")
		{
			0 => (true, true),
			1 => (false, true),
			2 => (false, false),
			_ => (false, false),
		};

		let exclude_regex = match matches.values_of("exclude")
		{
			Some(regex_strs) =>
			{
				let combined_regex = regex_strs.map(|r| format!("({})", r)).collect::<Vec<String>>().join("|");
				match Regex::new(&combined_regex)
				{
					Ok(r) => Some(r),
					Err(e) ->
					{
						println!("[ERR] COULD NOT PROCESS EXCLUDE REGEX: {}", e);
						std::process::exit(1);
					}
				}
			}
			None => None,
		};
		let include_regex = match matches.values_of("include")
		{
			Some(regex_strs) =>
			{
				let combined_regex = regex_strs.map(|r| format!("({})", r)).collect::<Vec<String>>().join("|");
				match Regex::new(&combined_regex)
				{
					Ok(r) => Some(r),
					Err(e) =>
					{
						println!("[ERR] COULD NOT PROCESS INCLUDE REGEX: {}", e);
						std::process::exit(1);
					}
				}
			}
			None => None,
		};

		let threads = num_cpus::get();
		let mut workers = vec![];
		let (workq, stealer) = deque::new();
		for _ in 0..threads
		{
			let worker = Worker
			{
				chan: stealer.clone()
			};
			workers.push(thread::spawn(|| worker.run()));
		}
		for target in tgt
		{
			let walker = WalkBuilder::new(target).ignore(use_ignore)
								.git_ignore(use_ignore)
								.git_exclude(use_ignore)
								.hidden(ignore_hidden)
								.build();
			let files = walker
				.filter_map(Result::ok)
				.filter(|entry| entry.file_type().expect("NO FILETYPE").is_file())
				.map(|entry| String::from(entry.path().to_str().unwrap()))
				.filter(|path| match include_regex
				{
					None => true,
					Some(ref include) => include.is_match(path),
				})
				.filter(|path| match exclude_regex
				{
					None => true,
					Some(ref exclude) => !exclude.is_match(path),
				});
			for path in files
			{
				workq.push(Work::File(path));
			}
		}

		for _ in 0..workers.len()
		{
			workq.push(Work::Quit);
		}

		let mut filecnt: Vec<FileCount> = Vec::new();
		for worker in workers
		{
			filecnt.extend(worker.join().unwrap().iter().cloned())
		}

		let mut by_lang: HashMap<Lang, Vec<FileCount>> = HashMap::new();
		for fc in filecnt
		{
			match by_lang.entry(fc.lang)
			{
				Entry::Occupied(mut elem) => elem.get_mut().push(fc),
				Entry::Vacant(elem) =>
				{
					elem.insert(vec![fc]);
				}
			};
		}
