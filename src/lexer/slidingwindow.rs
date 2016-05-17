struct SlidingWindow {
    characters: Vec<char>,
    current_pos: usize,
    offset: usize,
    file_len: usize
}

impl SlidingWindow {
    pub fn new(&self, source_code: &str) -> SlidingWindow {
     //   let chopped_source_code = source_code.chars().collect();
        SlidingWindow {
            characters: source_code.chars().collect(),
            current_pos: 0,
            offset: 0,
            file_len: source_code.len()
        }
    }

    fn peek(&mut self) -> char {
        self.offset += 1;
        self.characters[self.current_pos + self.offset].to_owned()
    }

    fn advance_char(&mut self) {
        self.offset = 0;
        self.current_pos += 1;
    }
    
    fn is_eof(&mut self) -> bool {
        self.current_pos == self.file_len
    }

}