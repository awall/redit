use geometry::*;

pub struct Line {
	pub text: String,
}
	
pub struct Buffer {
	pub show_line_numbers: bool,
	pub cursor: Pos,
	pub lines: Vec<Line>,
}

pub fn append_char(ch: char, buffer: &mut Buffer) {	
	let lines = &mut buffer.lines;	
	let end = lines.len() - 1;
	buffer.cursor.x += 1;
	lines[end].text.push(ch);
}

pub fn append_newline(buffer: &mut Buffer) {
	append_char('\n', buffer);
	buffer.lines.push(Line { text: String::new() });

	buffer.cursor.x  = 0;
	buffer.cursor.y += 1;
}

pub fn append_backspace(buffer: &mut Buffer) {
    let lines = &mut buffer.lines;	
	let end = lines.len() - 1;

	if lines[end].text.is_empty() {		
		if end != 0 {
			lines.pop();
			buffer.cursor.x = lines[end - 1].text.len() as int;
			buffer.cursor.y -= 1;
		}
	} else {
		lines[end].text.pop();
		buffer.cursor.x -= 1;
	}
}

pub fn append_left(buffer: &mut Buffer) {
}

pub fn append_right(buffer: &mut Buffer) {
}

pub fn append_down(buffer: &mut Buffer) {
}

pub fn append_up(buffer: &mut Buffer) {
}