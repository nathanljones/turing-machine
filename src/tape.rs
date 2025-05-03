pub struct Tape {
    cells: Vec<char>,
    position: u64,
}

impl Tape {
    pub fn new() -> Tape {
        // start the tape with one cell which is blank
        Tape {
            cells: vec![' '],
            position: 0,
        }
    }
    fn add_cell_left(&mut self) {
        self.cells.insert(0, ' ');
    }
    fn add_cell_right(&mut self) {
        self.cells.push(' ');
    }
    pub fn move_left(&mut self) {
        // decrement the position counter, but if hits zero then
        // add an element onto the beginning of the tape and don't decrement
        // this will extend the tape "indefinitely" to the left
        if self.position == 0 {
            self.add_cell_left();
        } else {
            self.position -= 1;
        }
    }
    pub fn move_right(&mut self) {
        // increment the position counter, but if we go above the length of the vector
        // then add an element to the end, then increment the position counter
        // this will extend the tape "indefinitely" to the right
        if self.position == (self.cells.len() - 1) as u64 {
            self.add_cell_right();
        }
        self.position += 1;
    }
    // the read and write bits are effectively the tape head
    // not sure if this needs to be spilt out
    pub fn read(&self) -> char {
        self.cells[self.position as usize]
    }
    pub fn write(&mut self, c: char) {
        self.cells[self.position as usize] = c;
    }
}
mod tests {
    use super::*;
    #[test]
    fn test_tape_add_cell_left() {
        let mut tape = Tape::new();
        tape.move_left();
        assert_eq!(tape.read(), ' ');
        tape.move_left();
        assert_eq!(tape.read(), ' ');
    }
    #[test]
    fn test_tape_add_cell_right() {
        let mut tape = Tape::new();
        tape.move_right();
        assert_eq!(tape.read(), ' ');
        tape.move_right();
        assert_eq!(tape.read(), ' ');
    }
    #[test]
    fn test_tape_read_and_write() {
        let mut tape = Tape::new();
        tape.write('a');
        assert_eq!(tape.read(), 'a');
    }
    #[test]
    fn test_tape_read_and_write_move_left() {
        let mut tape = Tape::new();
        tape.write('a');
        assert_eq!(tape.read(), 'a');
        tape.move_left();
        tape.write('b');
        assert_eq!(tape.read(), 'b');
    }
    #[test]
    fn test_tape_read_and_write_move_right() {
        let mut tape = Tape::new();
        tape.write('a');
        assert_eq!(tape.read(), 'a');
        tape.move_right();
        tape.write('b');
        assert_eq!(tape.read(), 'b');
    }
}
