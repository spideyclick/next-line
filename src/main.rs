use lazy_regex::{lazy_regex, Lazy, Regex};
use std::io::{stdin, IsTerminal};

static RE: Lazy<Regex> = lazy_regex!(r"(?m)^(\s+)?(- |\d+\.)?(\[[ \*xX]\] )?.*$");

fn main() {
	let input = stdin();
	if input.is_terminal() {
		println!("No input detected from STDIN");
		std::process::exit(1);
	}
	let mut input_string = String::new();
	if input.read_line(&mut input_string).is_err() {
		println!("Could not read from STDIN!");
		std::process::exit(1);
	}
	let trimmed_string = input_string.trim_end();
	let output = generate_next_line(trimmed_string);
	println!("{output}");
	std::process::exit(0)
}

fn generate_next_line(input: &str) -> String {
	RE.replace_all(input, "$1$2").into_owned()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn normal_line() {
		let result = generate_next_line("Abc");
		assert_eq!(result, "");
	}
	#[test]
	fn indented_line() {
		let result = generate_next_line("    Abc\tDef");
		assert_eq!(result, "    ");
	}
	#[test]
	fn markdown_list() {
		let result = generate_next_line("\t\t- Abc");
		assert_eq!(result, "\t\t- ");
	}
	// TODO: Implement Number Increment
	#[ignore]
	#[test]
	fn markdown_number() {
		let result = generate_next_line("   9. 123Abc");
		assert_eq!(result, "   10. ");
	}
}
