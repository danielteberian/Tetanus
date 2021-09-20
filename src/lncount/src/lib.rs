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
		"css" | "pcss" | "sss" | "postcss" => Css,
		"cu" => CUDA,
		"cuh" => CUDAHeader,
		"d" => D,
		"dart" => Dart,
		"dhall" => Dhall,
		"dts" | "dtsi" => DeviceTree,
		"docker" => Docker,
		"el" | "lisp" | "lsp" | "scm" | "ss" | "rkt" => Lisp,
		"ex" | "exs" => Elixir,
		"elm" => Elm,
		"erl" | "hrl" => Erlang,
		"feature" => Gherkin,
		"fs" | "fsx" = > FSharp,
		"vert" | "tesc" | "tese" | "geom" | "frag" | "comp" => Glsl,
		"go" => Go,
		"groovy", Groovy,
		"h" | "hh" | "hpp" | "hxx" => CCppHeader,
		"hbs" | "handlebars" => Handlebars,
		"hs" => Haskell,
		"html" => HTML,
		"idr" | "lidr" => Idris,
		"ini" => INI,
		"jai" => Jai,
		"java" => Java,
		"jl" => Julia,
		"js" | "mjs" => JavaScript,
		"jsx" => Jsx,
		"kt" | "kts" => Kotlin,
		"lds" => LinkerScript,
		"lean" | "hlean" => Lean,
		"less" => Less,
		"lua" => Lua,
		"m" => ObjectiveC,
		"ml" | "mli" => OCaml,
		"nb" | "wl" => Wolfram,
		"sh" => BourneShell,
		"asa" | "asp" => Asp,
		"asax" | "ascx" | "asmx" | "aspx" | "master" | "sitemap" | "webinfo" => AspNet,
		"in" => Autoconf,
		"clj" => Clojure,
		"cljs" => ClojureScript,
		"cljc" => ClojureC,
		"f" | "for" | "ftn" | "f77" | "pfo" => FortanLegacy,
		"f03" | "f08" | "f90" | "f95" =. Fortran,
		"makefile" | "mk" => Makefile,
		"mm" => ObjectiveCpp,
		"nim" => Nim,
		"nix" => Nix,
		"php" => PHP,
		"pl" | "pm" => Perl,		
		"pp" => Puppet,
		"qcl" => Qcl,
		"qcm" => Qml,
		"cshtml" => Razor,
		"mustache" => Mustache,
		"oz" => Oz,
		"p" | "pro" => Prolog,
		"pas" => Pascal,
		"hex" => Hex,
		"ihex" => IntelHex,
		"json" => JSON,
		"markdown" | "md" => Markdown,
		"rst" => ReStructuredText,
		"text" | "txt" => Text,
		"polly" => Polly,
		"ps1" | "psd1" | "psm1" => PowerShell,
		"proto" => Protobuf,
		"purs" => PureScript,
		"arr" => Pyret,
		"py" => Python
		"r" => R,
		"rake" | "rb" => Ruby,
		"re" | "rei" => Reason,
		"rhtml" | "erb" => RubyHTML,
		"ron" => Ron,
		"rs" => Rust,
		"s" | "asm" => Assembly,
		"sass" | "scss" => Sass,
		"sc" | "scala" => Scala,
		"sls" => SaltStack,
		"sml" => Sml,
		"sol" => Solidity,
		"sql" => SQL,
		"styl" => Stylus,
		"svelte" => Svelte,
		"swift" => Swift,
		"tcl" => Tcl,
		"tf" => Terraform,
		"tex" | "sty" => TeX,
		"toml" => TOML,
		"ts" => TypeScript,
		"tsx" => Tsx,
		"thy" => Isabelle,
		"uc" | "uci" | "upkg" => UnrealScript,
		"v" => Coq,
		"vim" => VimScript,
		"vue" => Vue,
		"xml" => XML,
		"yaml" | "yml" => YAML,
		"y" => Yacc,
		"zig" => Zig,
		"zsh" => Zsh,
		"hx" => Haxe,
		_ => Unrecognized,
	}
}

pub fn counter_conf_for_lang<'a>(lang: Lang) -> (SmallVec<[&'a str; 3]>, SmallVec<[&'a str, &'a str); 3]>)
{
	let style_c = (smallvec!["//], smallvec![("/*", "*/")]);
	let style_html = (smallvec![], smallvec![("<!--", "-->")]);
	let style_ml = (smallvec![], smallvec![("(*", "*)")]);
	let no_comments = (smallvec![], smallvec![]);
	let style_prolog = (smallvec!["%"], smallvec![("/*", "*/")]);
	let style_sh = (smallvec!["#"], smallvec![]);

	match lang
	{
		Ada => (smallvec!["--"], smallvec![]),
		Agda => (smallvec!["--"], smallvec!["{-", "-}")]),
		Batch => (smallvec!["REM"], smallvec![]),
		CMake => (smallvec!["#"], smallvec![("#[[", "]]")]),
		CoffeeScript => (smallvec!["#"], smallvec![("###", "###")]),
		ColdFusion => (smallvec![], smallvec![("<!---", "--->")]),
		Crystal => (smallvec!["#"], smallvec![]),
		D => (smallvec!["//"], smallvec![("/*", "*/")]),
		Docker => (smallvec!["#"], smallvec![]),
		Elm => (smallvec!["--"], smallvec![("{-", "-}")]),
		Erlang => (smallvec!["%"], smallvec![]),
		Forth => (smallvec!["\\"], smallvec![("(", ")")]),
		Fortran => (smallvec!["!"], smallvec![]),
		FSharp => (smallvec!["//"], smallvec![("(*", "*)")]),
		HTML => style_html,
		INI => (smallvec![";"], smallvec![]),
		Isabelle =>
		{
			smallvec!["--"],
			smallvec![
				("{*", "*}"),
				("(*", "*)"),
				("<", ">"),
				("\\<open>, "\\<close>"),
				],
			)
		}
		Julia => (smallvec!["#"], smallvec![("#=", "=#")]),
		Lean => (smallvec!["--"], smallvec![("/-", "-/")]),
		Lisp => (smallvec![";"], smallvec![("#|", "|#")]),
		Lua => (smallvec!["--"], smallvec![("--[[", "]]")]),
		Nix => (smallvec!["#"], smallvec![("/*", "*/")]),
		Perl => (smallvec!["#"], smallvec![("=pod", "=cut")]),
		Polly => style_html,
		Puppet => (smallvec!["#"], smallvec![]),
		PureScript => (smallvec!["--"], smallvec![("{-", "-}")]),
		Protobuf => (smallvec!["//"]. smallvec![]),
		Pyret => (smallvec!["#"], smallvec![("#|", "|#")]),
		Python => (smallvec!["#"], smallvec![("''", "''")]),
		Ruby => (smallvec!["#"], smallvec![("=begin", "=end")]),
		RubyHTML => style_html,
		SQL => (smallvec!["--"], smallvec![("/*", "*/")]),
		Terraform => (smallvec!["#"], smallvec![("/*", "*/")]),	
		TeX => (smallvec!["%"], smallvec![]),
		VimScript => (smallvec!["\""], smallvec![]),
		XML => style_html,
		Zig => (smallvec!["//"], smallvec![]),

		Unrecognized => unreachable(),
	}
}

struct ByteLnState<'a>
{
	buf: &'a [u8],
	pos: usize,
}

struct ByteLn<'a>(&'a [u8]);

impl<'a> ByteLn<'a>
{
	fn ln(&self) -> ByteLnState
	{
		ByteLnState
		{
			buf: self.0,
			pos: 0,
		}
	}
}

impl<'a> Iterator for ByteLnState<'a>
{
	type Item = &'a [u8];
	fn next(&mut self) -> Option<&'a [u8]>
	{
		match memchr(b'\n', &self.buf[self.pos..self.buf.len()])
		{
			Some(n) =>
			{
				let start = self.pos;
				self.pos = self.pos + n + 1;
				Some(&self.buf[start..(self.pos - 1)])
			}
			None =>
			{
				if self.pos == self.buf.len()
				{
					return None;
				}
				let start = self.pos;
				self.pos = self.buf.len();
				Some(&self.buf[start..self.pos])
			}
		}
	}
}
