#[macro_use]
extern crate clap;

use anyhow::{anyhow, Context, Error as AnyhowError};
use atty::Stream;
use clap::{App, AppSettings, Arg};
use hexview::{BorderStyle, Input, Printer};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, prelude::*, SeekFrom};
use thiserror::Error as ThisError;



fn run() -> Result<(), AnyhowError>
{
	let app = App::new(crate_name!())
		.setting(AppSettings::ColorAuto)
		.setting(AppSettings::ColoredHelp)
		.setting(AppSettings::DeriveDisplayOrder)
		.setting(AppSettings::UnifiedHelpMessage)
		.max_term_width(90)
		.version(crate_version!())
		.about(crate_description!())
		.arg(
			Arg::with_name("FILE")
				.help("FILE TO BE DISPLAYED. IF NO FILE IS SPECIFIED, STDIN IS USED."),
		)
		.arg(
			Arg::with_name("LENGTH")
				.short("n")
				.long("length")
				.takes_value(true)
				.value_name("N")
				.help("SPECIFY THE NUMBER OF BYTES TO BE READ FROM INPUT."),
		)
		.arg(
			Arg::with_name("BYTES")
				.short("c")
				.long("bytes")
				.takes_value(true)
				.value_name("N")
				.conflicts_with("LENGTH")
				.help("ALIAS FOR -n/--LENGTH"),
		)
		.arg(
			Arg::with_name("COUNT")
				.short("l")
				.takes_value(true)
				.value_name("N")
				.hidden(true)
				.help("ALIAS FOR -n/--LENGTH"),
		)
		.arg(
			Arg::with_name("SKIP")
				.short("s")
				.long("skip")
				.takes_value(true)
				.value_name("N")
				.help("SKIP X NUMBER OF BYTES FROM THE START OF INPUT"),
		)
		.arg(
			Arg::with_name("BLOCK_SIZE")
				.long("block-size")	
				.takes_value(true)
				.value_name("SIZE")
				.help("SET THE BLOCK SIZE"),
		)
		.arg(
			Arg::with_name("NOSQUEEZE")
				.short("v")
				.long("no-squeeze")
				.help("DISPLAY ALL INPUT DATA."),
		)
		.arg(
			Arg::with_name("BORDER")
				.long("border")
				.takes_value(true)
				.value_name("STYLE")
				.possible_values(&["unicode", "ascii", "none"])
				.default_value("unicode")
				.help("WHAT THE BORDER IS DRAWN WITH. DEFAULTS TO UNICODE"),
		)
		.arg(
			Arg::with_name("DISPLAY_OFFSET")
				.short("o")
				.long("display-offset")
				.takes_value(true)
				.value_name("N")
				.help("ADD X BYTES TO THE DISPLAYED FILE POSITION."),
		);

	let matches = app.get_matches_safe()?;
	let stdin = io::stdin();
	let mut reader = match matches.value_of("FILE")
	{
		Some(filename) => Input::File(File::open(filename)?),
		None => Input::Stdin(stdin.lock()),
	};

	let blksize = matches
		.value_of("BLOCK_SIZE")
		.map(|bs|
		{
			bs.parse::<i64>().map_err(|e| anyhow!(e)).and_then(|x|
			{
				PositiveI64::new(x).ok_or_else(|| anyhow!("BLOCKSIZE ARG MUST BE POSITIVE."))
			})
		})
		.transpose()?
		.unwrap_or_else(|| PositiveI64::new(512).unwrap());

	let skiparg = matches
		.value_of("SKIP")
		.map(|s|
		{
			parse_byte_offset(s, blksize).context(anyhow!("COULD NOT PARSE ARG AS BYTE COUNT: {:?}", s))
		})
		.transpose()?;

