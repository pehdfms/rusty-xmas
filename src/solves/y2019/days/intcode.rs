#[derive(Clone, Debug)]
enum Mode {
    Position = 0,
    Immediate = 1,
}

pub struct Computer {
    memory: Vec<i64>,
    pointer: usize,
    finished: bool,
    blocked: bool,
    modes: Vec<Mode>,
    inputs: Vec<i64>,
    input_pointer: usize,
    outputs: Vec<i64>,
}

impl Computer {
    pub fn from_vec(memory: Vec<i64>) -> Self {
        Self {
            pointer: 0,
            finished: memory.is_empty(),
            blocked: false,
            memory,
            modes: Vec::default(),
            inputs: Vec::default(),
            input_pointer: 0,
            outputs: Vec::default(),
        }
    }

    pub fn from_string(memory: &str) -> Self {
        Self::from_vec(Self::parse(memory))
    }

    fn get_input(&mut self) -> Option<i64> {
        let result = self.inputs.get(self.input_pointer)?;
        self.input_pointer += 1;

        Some(*result)
    }

    fn convert_usize(n: i64) -> usize {
        usize::try_from(n).expect("Function expected i64 to be unsigned, got negative!")
    }

    pub fn add_input(&mut self, input: i64) {
        self.inputs.push(input);
        self.blocked = false;
    }

    pub const fn read_outputs(&self) -> &Vec<i64> {
        &self.outputs
    }

    pub fn parse(memory: &str) -> Vec<i64> {
        memory
            .trim()
            .replace(' ', "")
            .replace('\n', "")
            .split(',')
            .map(|n| {
                n.parse()
                    .expect("All memory values should parse as a number")
            })
            .collect()
    }

    pub fn read(&self, position: usize) -> i64 {
        let memory_length = self.memory.len();
        *self.memory.get(position).unwrap_or_else(|| {
            panic!(
                "Tried to index {position} in memory but actual size of memory is {memory_length}"
            )
        })
    }

    pub fn replace(&mut self, position: usize, value: i64) {
        let memory_length = self.memory.len();
        *self.memory.get_mut(position).unwrap_or_else(||
            panic!(
                "Tried to replace position {position} in memory with {value} but actual size of memory is {memory_length}"
            )
        ) = value;
    }

    fn pop(&mut self) -> i64 {
        let value = self.read(self.pointer);
        self.pointer += 1;

        value
    }

    fn pop_arg(&mut self) -> i64 {
        let mode = self
            .modes
            .pop()
            .expect("Mode count should always match argument count");

        match mode {
            Mode::Position => {
                let arg = Self::convert_usize(self.pop());
                self.read(arg)
            }
            Mode::Immediate => self.pop(),
        }
    }

    fn unwind(&mut self, count: usize) {
        self.pointer -= count;
    }

    fn unary_operation<F>(&mut self, write: bool, f: F)
    where
        F: FnOnce(&mut Self, i64),
    {
        self.pad_modes(1);
        let arg = if write { self.pop() } else { self.pop_arg() };

        f(self, arg);
    }

    fn binary_operation<F>(&mut self, write: bool, f: F)
    where
        F: FnOnce(&mut Self, i64, i64),
    {
        self.pad_modes(2);

        let lhs = self.pop_arg();
        let rhs = if write { self.pop() } else { self.pop_arg() };

        f(self, lhs, rhs);
    }

    fn ternary_operation<F>(&mut self, write: bool, f: F)
    where
        F: FnOnce(&mut Self, i64, i64, i64),
    {
        self.pad_modes(3);

        let lhs = self.pop_arg();
        let mhs = self.pop_arg();
        let rhs = if write { self.pop() } else { self.pop_arg() };

        f(self, lhs, mhs, rhs);
    }

    fn pad_modes(&mut self, count: usize) {
        let length = self.modes.len();
        let padding: Vec<Mode> = (length..count).map(|_| Mode::Position).collect();

        self.modes.splice(0..0, padding.iter().cloned());
    }

    fn parse_opcode(&mut self) -> i64 {
        let instruction = self.pop().to_string();
        let instruction_length = instruction.len();

        if instruction_length <= 2 {
            return instruction
                .parse()
                .expect("Instructions with only one or two characters should always be opcodes.");
        }

        let modes = &instruction[0..instruction_length - 2];
        let opcode = &instruction[instruction_length - 2..];

        self.modes = modes
            .chars()
            .map(|mode| match mode {
                '0' => Mode::Position,
                '1' => Mode::Immediate,
                _ => panic!("Unexpected mode found!"),
            })
            .collect();

        opcode.parse().expect("Opcode should parse as i64")
    }

    pub fn step(&mut self) -> bool {
        if self.finished || self.blocked {
            return false;
        }

        let opcode = self.parse_opcode();

        match opcode {
            // Addition
            1 => self.ternary_operation(true, |this, lhs, rhs, out| {
                this.replace(Self::convert_usize(out), lhs + rhs);
            }),
            // Multiplication
            2 => self.ternary_operation(true, |this, lhs, rhs, out| {
                this.replace(Self::convert_usize(out), lhs * rhs);
            }),
            // Input
            3 => self.unary_operation(true, |this, arg| {
                let input = this.get_input();
                if let Some(raw_input) = input {
                    this.replace(Self::convert_usize(arg), raw_input);
                } else {
                    this.blocked = true;
                    this.unwind(2);
                }
            }),
            // Output
            4 => self.unary_operation(false, |this, arg| this.outputs.push(arg)),
            // Jump if not zero
            5 => self.binary_operation(false, |this, lhs, rhs| {
                this.pointer = if lhs == 0 {
                    this.pointer
                } else {
                    Self::convert_usize(rhs)
                };
            }),
            // Jump if zero
            6 => self.binary_operation(false, |this, lhs, rhs| {
                this.pointer = if lhs == 0 {
                    Self::convert_usize(rhs)
                } else {
                    this.pointer
                };
            }),
            // Less than
            7 => self.ternary_operation(true, |this, lhs, rhs, out| {
                this.replace(Self::convert_usize(out), i64::from(lhs < rhs));
            }),
            // Equal to
            8 => self.ternary_operation(true, |this, lhs, rhs, out| {
                this.replace(Self::convert_usize(out), i64::from(lhs == rhs));
            }),
            // Halt
            99 => self.finished = true,
            op => panic!("Found unexpected opcode: {op}!"),
        };

        true
    }

    pub fn run(&mut self) {
        while self.step() {}
    }

    #[cfg(test)]
    const fn read_memory(&self) -> &Vec<i64> {
        &self.memory
    }

    pub const fn finished(&self) -> bool {
        self.finished
    }
}

#[test]
fn should_not_modify_memory_on_creation() {
    let computer = Computer::from_vec(vec![1, 2, 3, 4, 99]);
    let memory = computer.read_memory();

    assert_eq!(&vec![1, 2, 3, 4, 99], memory);
}

#[test]
#[should_panic(expected = "unexpected opcode")]
fn should_panic_on_unexpected_opcode() {
    let mut computer = Computer::from_vec(vec![31, 2, 1, 4, 99]);
    computer.step();
}

#[test]
fn should_generate_from_string() {
    let vec_computer = Computer::from_vec(vec![1, 0, 0, 0, 99]);
    let string_computer = Computer::from_string("1, 0, 0, 0, 99");

    assert_eq!(vec_computer.read_memory(), string_computer.read_memory());
}

#[test]
fn should_add() {
    let mut computer = Computer::from_vec(vec![1, 0, 0, 0, 99]);
    computer.step();

    // Comparing the entire memory to make sure we're not corrupting memory.
    assert_eq!(computer.read_memory(), &vec![2, 0, 0, 0, 99]);

    // Making sure read matches read_memory.
    assert_eq!(computer.read(0), 2);
}

#[test]
fn should_multiply() {
    let mut computer = Computer::from_vec(vec![2, 3, 0, 3, 99]);
    computer.step();

    // Comparing the entire memory to make sure we're not corrupting memory.
    assert_eq!(computer.read_memory(), &vec![2, 3, 0, 6, 99]);

    // Making sure read matches read_memory.
    assert_eq!(computer.read(3), 6);
}

#[test]
fn should_handle_multiple_ops() {
    let mut computer = Computer::from_vec(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);

    computer.run();

    assert_eq!(computer.read_memory(), &vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
}

#[test]
fn should_handle_long_source() {
    let mut computer = Computer::from_vec(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

    computer.run();

    assert_eq!(
        computer.read_memory(),
        &vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
}

#[test]
#[should_panic]
fn should_panic_on_unexpected_mode() {
    let mut computer = Computer::from_vec(vec![9002, 4, 3, 4, 33]);

    computer.run();
}

#[test]
fn should_handle_immediate_mode() {
    let mut computer = Computer::from_vec(vec![1002, 4, 3, 4, 33]);

    computer.run();

    assert_eq!(computer.read_memory().last().unwrap(), &99);
}

#[test]
fn should_handle_negatives() {
    let mut computer = Computer::from_vec(vec![1101, 100, -1, 4, 0]);

    computer.run();

    assert_eq!(computer.read_memory().last().unwrap(), &99);
}

#[test]
fn should_handle_io() {
    let mut computer = Computer::from_vec(vec![3, 0, 4, 0, 99]);

    computer.add_input(10);
    computer.run();

    assert_eq!(computer.outputs[0], 10);
}

#[test]
fn should_handle_io_blocking() {
    let mut computer = Computer::from_vec(vec![3, 0, 3, 1, 4, 1, 99]);

    computer.run();
    assert!(computer.read_outputs().is_empty());
    assert!(computer.blocked);

    computer.add_input(10);
    computer.run();
    assert!(computer.read_outputs().is_empty());
    assert!(computer.blocked);

    computer.add_input(10);
    computer.run();
    assert!(computer.finished());
}

#[test]
fn should_handle_diagnostics() {
    // Very long input, it's the diagnostic input for d5p1
    let mut computer = Computer::from_vec(vec![
        3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 91, 92, 225, 1102, 85, 13, 225, 1,
        47, 17, 224, 101, -176, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 7, 224, 1, 223,
        224, 223, 1102, 79, 43, 225, 1102, 91, 79, 225, 1101, 94, 61, 225, 1002, 99, 42, 224, 1001,
        224, -1890, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 6, 224, 1, 224, 223, 223, 102, 77,
        52, 224, 1001, 224, -4697, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 7, 224, 1, 224, 223,
        223, 1101, 45, 47, 225, 1001, 43, 93, 224, 1001, 224, -172, 224, 4, 224, 102, 8, 223, 223,
        1001, 224, 1, 224, 1, 224, 223, 223, 1102, 53, 88, 225, 1101, 64, 75, 225, 2, 14, 129, 224,
        101, -5888, 224, 224, 4, 224, 102, 8, 223, 223, 101, 6, 224, 224, 1, 223, 224, 223, 101,
        60, 126, 224, 101, -148, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 2, 224, 1, 224,
        223, 223, 1102, 82, 56, 224, 1001, 224, -4592, 224, 4, 224, 1002, 223, 8, 223, 101, 4, 224,
        224, 1, 224, 223, 223, 1101, 22, 82, 224, 1001, 224, -104, 224, 4, 224, 1002, 223, 8, 223,
        101, 4, 224, 224, 1, 223, 224, 223, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1,
        99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274,
        1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0,
        1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0,
        1105, 1, 99999, 8, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 329, 1001, 223, 1, 223,
        1007, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 344, 101, 1, 223, 223, 108, 226, 226,
        224, 1002, 223, 2, 223, 1006, 224, 359, 1001, 223, 1, 223, 107, 226, 677, 224, 102, 2, 223,
        223, 1006, 224, 374, 101, 1, 223, 223, 8, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 389,
        1001, 223, 1, 223, 1008, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 404, 101, 1, 223,
        223, 7, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 419, 101, 1, 223, 223, 1108, 226, 677,
        224, 1002, 223, 2, 223, 1005, 224, 434, 101, 1, 223, 223, 1108, 226, 226, 224, 102, 2, 223,
        223, 1005, 224, 449, 1001, 223, 1, 223, 107, 226, 226, 224, 102, 2, 223, 223, 1005, 224,
        464, 101, 1, 223, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 479, 101, 1, 223,
        223, 1007, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 494, 1001, 223, 1, 223, 1008, 226,
        226, 224, 1002, 223, 2, 223, 1005, 224, 509, 1001, 223, 1, 223, 1108, 677, 226, 224, 1002,
        223, 2, 223, 1006, 224, 524, 1001, 223, 1, 223, 108, 677, 677, 224, 1002, 223, 2, 223,
        1005, 224, 539, 101, 1, 223, 223, 108, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 554,
        101, 1, 223, 223, 1008, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 569, 1001, 223, 1,
        223, 1107, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 584, 1001, 223, 1, 223, 7, 677, 226,
        224, 102, 2, 223, 223, 1005, 224, 599, 1001, 223, 1, 223, 8, 677, 226, 224, 1002, 223, 2,
        223, 1005, 224, 614, 1001, 223, 1, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1006, 224,
        629, 101, 1, 223, 223, 1107, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 644, 1001, 223,
        1, 223, 1107, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 659, 1001, 223, 1, 223, 107, 677,
        677, 224, 1002, 223, 2, 223, 1005, 224, 674, 101, 1, 223, 223, 4, 223, 99, 226,
    ]);

    computer.add_input(1);
    computer.run();

    let outputs = computer.read_outputs();

    assert!(outputs[0..outputs.len() - 2].iter().all(|n| n == &0));
}

#[test]
fn should_handle_equal_to_in_position_mode() {
    let mut computer = Computer::from_vec(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);

    computer.add_input(8);
    computer.run();

    assert_eq!(computer.read_outputs()[0], 1);
}

#[test]
fn should_handle_less_than_in_position_mode() {
    let mut computer = Computer::from_vec(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);

    computer.add_input(8);
    computer.run();

    assert_eq!(computer.read_outputs()[0], 0);
}

#[test]
fn should_handle_equal_to_in_immediate_mode() {
    let mut a = Computer::from_vec(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
    let mut b = Computer::from_vec(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);

    a.add_input(8);
    a.run();

    b.add_input(9);
    b.run();

    assert_eq!(a.read_outputs()[0], 1);
    assert_eq!(b.read_outputs()[0], 0);
}

#[test]
fn should_handle_less_than_in_immediate_mode() {
    let mut a = Computer::from_vec(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
    let mut b = Computer::from_vec(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);

    a.add_input(8);
    a.run();

    b.add_input(7);
    b.run();

    assert_eq!(a.read_outputs()[0], 0);
    assert_eq!(b.read_outputs()[0], 1);
}

#[test]
fn should_handle_jump_in_position_mode() {
    let mut a = Computer::from_vec(vec![
        3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
    ]);

    let mut b = Computer::from_vec(vec![
        3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
    ]);

    a.add_input(-20);
    a.run();

    b.add_input(0);
    b.run();

    assert_eq!(a.read_outputs()[0], 1);
    assert_eq!(b.read_outputs()[0], 0);
}

#[test]
fn should_handle_jump_in_immediate_mode() {
    let mut a = Computer::from_vec(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
    let mut b = Computer::from_vec(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);

    a.add_input(-20);
    a.run();

    b.add_input(0);
    b.run();

    assert_eq!(a.read_outputs()[0], 1);
    assert_eq!(b.read_outputs()[0], 0);
}

#[test]
fn should_halt() {
    let mut computer = Computer::from_vec(vec![99]);

    assert!(computer.step()); // First run doesn't halt
    assert!(!computer.step()); // Second run halts
}

#[test]
fn should_replace() {
    let mut computer = Computer::from_vec(vec![23, 0, 99]);

    computer.replace(0, 4);
    computer.run();

    assert_eq!(computer.read_outputs()[0], 4);
}

#[test]
#[should_panic(expected = "Tried to index")]
fn should_panic_on_lack_of_halt() {
    let mut computer = Computer::from_vec(vec![3, 0, 4, 0]);

    computer.add_input(0);
    computer.run();
}
