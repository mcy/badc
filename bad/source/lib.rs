use std::fs::File;
use std::io::Stdin;
use std::path::PathBuf;

pub mod ast;

pub enum ProgramSource {
	Path(PathBuf),
	File(File),
	Stdin(Stdin),
}

pub struct CompilationConfiguration {
	pub sources: Vec<ProgramSource>,
}

#[derive(Debug)]
pub enum Token {
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	Semicolon,
	SingleQuote,
	Codepoint(char),
	Identifier(String),
}

#[derive(Debug)]
pub struct TokenList {
	pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct SyntaxTree {}

pub fn lex(_config: &CompilationConfiguration) -> TokenList {
	let list: TokenList = TokenList { tokens: Vec::new() };
	list
}

pub fn parse(
	_token_stream: TokenList,
	_config: &CompilationConfiguration,
) -> SyntaxTree {
	SyntaxTree {}
}

pub fn compile(config: &CompilationConfiguration) -> SyntaxTree {
	let lex: TokenList = lex(config);
	let tree: SyntaxTree = parse(lex, config);
	tree
}
