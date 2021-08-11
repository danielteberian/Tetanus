#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate derive_more;
use reqwest;
use std::process::exit;
use structopt::StructOpt;

#[derive(From, Display, Debug)]
enum convbtc_err
{
	ApiError,
	Reqwest(reqwest::Error),
}

const APIURL: &str = "https://apiv2.bitcoinaverage.com/convert/global";

#[derive(Debug, StructOpt)]
#[structopt(name = "convbtc", about = "Convert BTC to various currencies.")]
struct Opt
{
	#[structopt(defval = "1")]
	amount: f64,

	#[structopt(short = "f", long = "from", defval = "BTC")]
	from: String,

	#[structopt(short = "t", long = "to", defval = "USD")]
	to: String,

	#[structopt(short = "s", long = "silent")]
	silent: bool,

	#[structopt(short = "v", long = "verbose")]
	verbose: bool,
}

#[derive(Deserialize, Debug)]
struct BTC_Resp
{
	time: String,
	success: bool,
	price: f64,
}

fn conv_btc(amount: f64, from: &str, to: &str) -> Result<BTC_Resp, convbtc_err>
{
	use convbtc_err::*;
	let client = reqwest::Client::new();
	let request =
		client
			.get(APIURL)
			.query(&[("from", from), ("to", to), ("amount", &amount.to_string())]);
	let respres: BtcResponse = request.send()?.json()?;

	if !respres.success
	{
		return Err(ApiError);
	}

	Ok(respres)
}

fn main()
{
	let opt = Opt::from_args();
	let resp = match conv_btc(opt.amount, &opt.from, &opt.to)
	{
		Ok(value) => value,
		Err(e) =>
		{
			println!("ERROR: Could not communicate with API.");
			if opt.verbose
			{
				println!("MESSAGE: {} - DETAILS: {:?}", e, e);
			}
			exit(1);
		}
	};

	if opt.silent
	{
		println!("{}", response.price);
	}
	else
	{
		println!("{} {}", response.price, &opt.to);
	}
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn test_conv_success()
	{
		match conv_btc(1.8, "BTC", "USE")
		{
			Ok(_) => assert!(true),
			Err(_) => assert!(false),
		}
	}

	#[test]
	fn test_conv_err_wrong_from()
	{
		match conv_btc(1.8, "wrongvalue", "USD")
		{
			Ok(_) => assert!(false),
			Err(_) => assert!(true),
		}
	}

	#[test]
	fn test_conv_err_wrong_to()
	{
		match conv_btc(1.8, "USD", "wrongvalue")
		{
			Ok(_) => assert!(false),
			Err(_) => assert!(true),
		}
	}
}
