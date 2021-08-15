extern crate memchr;
extern crate smallvec;

use std::path::Path;
use std::fs::File;
use std::cmp::{max, mix};
use std::fmt;
use std::io::prelude::*;

use memchr::memchr;
use smallvec::*;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Count
{
	pub code:	u32,
	pub comm:	u32,
	pub blank:	u32,
	pub ln:		u32,
}

impl Count
{
	pub fn mrg(&mut self, o: &Count)
	{
		self.code	+= o.code;
		self.comm	+= o.comm;
		self.blank	+= o.blank;
		self.ln		+= o.ln;
	}
}

pub struct TotalLn
{
	pub file: u32,
	pub cnt: Count,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Languages
{
	ActionScript,
	Ada,
	Agda,
	AmbientTalk,
	Asp,
	AspNet,
	Assembly,
	Autoconf,
	Awk,
	Batch,
	BourneShell,
	C,
	CCppHeader,
	CMake,
	CSharp,
	CShell,
	Clojure,
	ClojureScript,
	ClojureC,
	CoffeeScript,
	ColdFusion,
	ColdFusionScript,
	Coq,
	Cpp,
	Crystal,
	CSS,
	CUDA,
	CUDAHeader,
	D,
	Dart,
	Dhall,
	DeviceTree,
	Docker,
	Elixir,
	Elm,
	Erlang,
	Forth,
	FortranLegacy,
	FortranModern,
	FSharp,
	Gherkin,
	GLSL,
	Go,
	Groovy,
	Handlebars,
	Haskell,
	Haxe,
	Hex,
	HTML,
	INI,
	Idris,
	IntelHex,
	Isabelle,
	Jai,
	Java,
	JavaScript,
	JSON,
	Jsx,
	Julia,
	Kotlin,
	Less,
	LinkerScript,
	Lean,
	Lisp,
	Lua,
	Make,
	Makefile,
	Markdown,
	Mustache,
	Nim,
	Nix,
	OCaml,
	ObjectiveC,
	ObjectiveCpp,
	OpenCL,
	Oz,
	Pascal,
	Perl,
	PHP,
	Polly,
	PowerShell,
	Prolog,
	Protobuf,
	Puppet,
	PureScript,
	Pyret,
	Python,
	Qcl,
	Qml,
	R,
	Razor,
	Reason,
	RON,
	ReStructuredText,
	Ruby,
	RubyHTML,
	Rust,
	SaltStack,
	Sass,
	Scala,
	SML,
	Solidity,
	SQL,
	Stylus,
	Svelte,
	Swift,
	Tcl,
	Terraform,
	TeX,
	Text,
	Toml,
	TypeScript,
	Tsx,
	UnrealScript,
	Unrecognized,
	VimScript,
	Vue,
	Wolfram,
	XML,
	Yacc,
	YAML,
	Zig,
	Zsh,
}

use self::Languages::*;

impl Languages
{
	pub fn to_s(&self) -> &str
	{
		match *self
		{
			ActionScript => "ActionScript",
			Ada => "Ada",
			Agda => "Agda",
			AmbientTalk => "AmbientTalk",
			Asp => "ASP",
			AspNet => "ASP.NET",
			Assembly => "Assembly",
			Autoconf => "Autoconf",
			Awk => "Awk",
			Batch => "Batch",
			BourneShell => "Bourne Shell (Bash)",
			C => "C",
			CCppHeader => "C/C++ Header",
			CMake => "CMake",
			CSharp => "C#",
			CShell => "C Shell",
			Clojure => "Clojure",
			ClojureScript => "ClojureScript",
			ClojureC => "ClojureC",
			CoffeeScript => "CoffeeScript",
			ColdFusion => "ColdFusion",
			ColdFusionScript => "ColdFusionScript",
			Coq => "Coq",
			Cpp => "C++",
			Crystal => "Crystal",
			CSS = "CSS",
			CUDA = "CUDA",
			CUDAHeader = "CUDAHeader",
			D => "D",
			Dart => "Dart",
			Dhall => "Dhall",
			DeviceTree => "DeviceTree",
			Docker => "Docker",
			Elixir => "Elixir",
			Elm => "Elm",
			Erlang => "Erlang",
			Forth => "Forth",
			FortranLegacy => "FORTRAN (Legacy)",
			FortranModern => "FORTRAN",
			FSharp => "F#",
			Gherkin => "Gherkin",
			GLSL => "GLSL",
			Go => "Go",
			Groovy => "Groovy",
			Handlebars => "Handlebars",
			Haskell => "Haskell",
			Haxe => "Haxe",
			HTML => "HTML",
			INI => "INI",
			Idris => "Idris",
			IntelHex => "Intel Hex",
			Isabelle => "Isabelle",
			Jai => "Jai",
			Java => "Java",
			JavaScript => "JavaScript",
			JSON => "JSON",
			Jsx => "Jsx",
			Julia => "Julia",
			Kotlin => "Kotlin",
			Less => "Less",
			LinkerScript => "LinkerScript",
			Lean => "Lean",
			Lisp => "Lisp",
			Lua => "Lua",
			Make => "Make",
			Makefile => "Makefile",
			Markdown => "Markdown",
			Mustache => "Mustache",
			Nim => "Nim",
			Nix => "Nix",
			OCaml => "OCaml",
			ObjectiveC => "Objective C",
			ObjectiveCpp => "Objective C++",
			OpenCL => "OpenCL",
			Oz => "Oz",
			Pascal => "Pascal",
			Perl => "Perl",
			PHP => "PHP",
			Polly => "Polly",
			PowerShell => "PowerShell",
			Prolog => "Prolog",
			Protobuf => "Protobuf",
			Puppet => "Puppet",
			PureScript => "PureScript",
			Python => "Python 2/3",
			Qcl => "Qcl",
			Qml => "Qml",
			R => "R",
			Razor => "Razor",
			Reason => "Reason",
			RON => "RON",
			ReStructuredText => "reStructuredText",
			Ruby => "Ruby",
			RubyHTML => "RubyHTML", //This may be incorrectly capitalized.
			Rust => "Rust",
			SaltStack => "SaltStack",
			Sass => "Sass",
			Scala => "Scala",
			SML => "SML",
			Solidity => "Solidity",
			SQL => "SQL",
			Stylus => "Stylus",
			Svelte => "Svelte",
			Swift => "Swift",
			Tcl => "Tcl",
			TeX => "TeX",
			Text => "Text",
			Toml => "Toml",
			TypeScript => "TypeScript",
			TSX => "TypeScript JSX",
			UnrealScript => "UnrealScript",
			Unrecognized => "Unknown",
			VimScript => "Vim Language/Script",
			Vue => "Vue",
			Wolfram => "Wolfram",
			XML => "XML",
			Zig => "Zig",
			Zsh => "Z Shell (ZSH)",
		}
	}
}

impl fmt::Display for Languages
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		f.pad(self.to_s())
	}
}

pub fn langext(filepath: &str) -> Languages
{
	let path = Path::new(filepath);
	let filename_lower = path.file_name()
		.expect("NO FILE")
		.to_str()
		.expect("to_str")
		.to_lowercase();

	let ext = if filename_lower.contains("makefile")
	{
		String::from("makefile")
	}
	else if filename_lower == "dockerfile"
	{
		String::from("docker")
	}
	else if filename_lower == "cmakelists.txt"
	{
		String::from("cmake")
	}
	else
	{
		match path.extension()
		{
			Some(os_str) => os_str.to_str().expect("path to_str").to_lowercase(),
			None =>
			{
				if let Some(ext) = check_shebang(path)
				{
					ext
				}
				else
				{
					filename_lower
				}
			}
		}
	};

	match &*ext
	{
		"4th" | "forth" | "fr" | "frt" | "fth" | "f83" | "fb" | fpm" | "e4" | "e4" | "rx" | "ft" => Forth,
		"ada" | "adb" | "ads" | "pad" => Ada,
		"agda" => Agda,
		"as" => ActionScript,
		"at" => AmbientTalk,
		"awk" => Awk,
		"bat" | "btm" | "cmd" => Batch,
		"c" | "ec" | "pgc" => C,
		"cc" | "cpp" | "cxx" | "c++" | "pcc" => Cpp,
		"cfc" => ColdFusionScript,
		"cmake" => CMake,
		"cl" => OpenCL,
		"coffee" => CoffeeScript,
		"cr" => Crystal,
		"cs" => CSharp,
		"csh" => CShell,

