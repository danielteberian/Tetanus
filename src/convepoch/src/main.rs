use std::process::exit;
use structopt::StructOpt;
use chrono::prelude::{NaiveDateTime, DateTime};
use chrono::{Utc};

#[derive(StructOpt)]
#[structopt(name = "convepoch")]
struct Opt
{
	value: String,
	#[structopt(short = "d", long = "date")]
	isdate: bool,
	#[structopt(short = "f", long = "format", default_value = "%Y-%m-%d %H:%M:%S")]
	format: String
}

fn convepoch(value: String, format: String) -> Result<String, String>
{
	let valconv = match value.parse::<i64>()
	{
		Ok(value) => value,
		Err(_) => return Err("ERROR: EPOCH CANNOT BE CONVERTED TO DATE/TIME.".to_string()),
	};
	let naivedate = NaiveDateTime::from_timestamp(valconv, 0);
	let datetz: DateTime<Utc> = DateTime::from_utc(naivedate, Utc);
	let datestr = datetz.format(&format).to_string();
	Ok(datestr)
}

fn convdatetime(value: String, format: String) -> Result<String, String>
{
	let naivedate = match NaiveDateTime::parse_from_str(&value, &format)
	{
		Ok(value) => value,
		Err(_) => return Err("ERROR: DATETIME CANNOT BE CONVERTED TO EPOCH.".to_string()),
	};

	let datetz: DateTime<Utc> = DateTime::from_utc(naivedate, Utc);
	let dttmstamp: i64 = datetz.timestamp();
	Ok(dttmstamp.to_string())
}

fn main()
{
	let opt = Opt::from_args();
	let resobj: Result<String, String>;
	if opt.isdate
	{
		resobj = convdatetime(opt.value, opt.format);
	}
	else
	{
		resobj = convepoch(opt.value, opt.format);
	}

	let res = match resobj
	{
		Ok(value) => value,
		Err(err) =>
		{
			println!("{}", err);
			exit(1);
		},
	};

	println!{"{}", res};
}
