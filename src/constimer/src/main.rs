extern crate chrono;
extern crate eventual;
extern crate ncurses;
extern crate regex;

use chrono::prelude::*;
use eventual::Timer;
use ncurses::*;
use regex::Regex;
use std::env;
use std::str::FromStr;


fn main()
{
	let args : Vec<String> = env::args().collect();
	if(args.len() < 2)
	{
		printusg();
		return;
	}
	run(&args);
}

fn run(args: &Vec<String>)
{
	let mut col = 0;
	let mut row = 0;
	let secornone = getsec(&args[1]);
	if(secornone.is_none())
	{
		printusg();
		return;
	}

	initscr();
	curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
	getmaxyx(stdscr(), &mut row, &mut col);

	let timer = Timer::new();
	let tick = timer.interval_ms(1000).iter();
	print_centmsg(&row,&col,&"INIT");

	let seccnt = secornone.unwrap();
	let mut i = seccnt;
	for _ in tick
	{
		if(i < 0)
		{
		break;
		}
		print_centmsg(&row,&col,&format!("{}s", i));
		i -= 1;
	}

	let fintm = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
	print_centmsg(&row, &col, &format!("TIMER FINISHED, {}s at {}", seccnt, fintm));

	getch();
	endwin();
}

fn print_centmsg(row: &i32, col: &i32, msg: &str)
{
	clear();
	let posX : i32 = col/2 - ((msg.len()/2) as i32);
	mvprintw(row/2, posX, &msg);
	refresh();
}

fn getsec(time_string: &str) -> Option<i32>
{
	let numre = Regex::new(r"^(\d+)$").unwrap();
	let minsre = Regex::new(r"^(\d+)m$").unwrap();

	if(numre.is_match(time_string))
	{
		let c = numre.captures(time_string).unwrap();
		let number = i32::from_str(&c[1]).unwrap();
		return Some(number);
	}

	if(minsre.is_match(time_string))
	{
		let c = minsre.captures(time_string).unwrap();
		let seconds = i32::from_str(&c[1]).unwrap();
		return Some(seconds*60);
	}

	return None;
}






fn printusg()
{
	println!("USAGE: CONSTIMER [TIME IN SEC]");
}
