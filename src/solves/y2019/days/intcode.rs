pub struct IntcodeComputer {
    memory: Vec<i32>,
    pointer: usize,
    finished: bool,
}

impl IntcodeComputer {
    pub fn new(memory: Vec<i32>) -> IntcodeComputer {
        IntcodeComputer {
            pointer: 0,
            finished: memory.is_empty(),
            memory,
        }
    }

    pub fn read(&self, position: usize) -> i32 {
        self.memory[position]
    }

    pub fn replace(&mut self, position: usize, value: i32) {
        self.memory[position] = value;
    }

    fn pop(&mut self) -> i32 {
        let value = self.read(self.pointer);
        self.pointer += 1;

        value
    }

    fn binary_operation<F>(&mut self, f: F)
    where
        F: FnOnce(i32, i32) -> i32,
    {
        let lhs = self.pop() as usize;
        let rhs = self.pop() as usize;
        let out = self.pop() as usize;

        self.replace(out, f(self.read(lhs), self.read(rhs)));
    }

    pub fn step(&mut self) -> bool {
        if self.finished {
            return false;
        }

        let opcode = self.pop();

        match opcode {
            1 => self.binary_operation(|lhs, rhs| lhs + rhs),
            2 => self.binary_operation(|lhs, rhs| lhs * rhs),
            99 => self.finished = true,
            op => panic!("Found unexpected opcode: {op}!"),
        };

        true
    }

    pub fn run(&mut self) {
        while self.step() {}
    }

    #[cfg(test)]
    fn read_memory(&self) -> &Vec<i32> {
        &self.memory
    }
}

#[test]
fn should_not_modify_memory_on_creation() {
    let computer = IntcodeComputer::new(vec![1, 2, 3, 4, 99]);
    let memory = computer.read_memory();

    assert_eq!(&vec![1, 2, 3, 4, 99], memory);
}

#[test]
#[should_panic(expected = "unexpected opcode")]
fn should_panic_on_unexpected_opcode() {
    let mut computer = IntcodeComputer::new(vec![231, 2, 1, 4, 99]);
    computer.step();
}

#[test]
fn should_add() {
    let mut computer = IntcodeComputer::new(vec![1, 0, 0, 0, 99]);
    computer.step();

    // Comparing the entire memory to make sure we're not corrupting memory.
    assert_eq!(computer.read_memory(), &vec![2, 0, 0, 0, 99]);

    // Making sure read matches read_memory.
    assert_eq!(computer.read(0), 2);
}

#[test]
fn should_multiply() {
    let mut computer = IntcodeComputer::new(vec![2, 3, 0, 3, 99]);
    computer.step();

    // Comparing the entire memory to make sure we're not corrupting memory.
    assert_eq!(computer.read_memory(), &vec![2, 3, 0, 6, 99]);

    // Making sure read matches read_memory.
    assert_eq!(computer.read(3), 6);
}

#[test]
fn should_handle_multiple_ops() {
    let mut computer = IntcodeComputer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);

    computer.run();

    assert_eq!(computer.read_memory(), &vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
}

#[test]
fn should_handle_long_source() {
    let mut computer = IntcodeComputer::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

    computer.run();

    assert_eq!(
        computer.read_memory(),
        &vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
}

#[test]
fn should_halt() {
    let mut computer = IntcodeComputer::new(vec![99]);

    assert!(computer.step()); // First run doesn't halt
    assert!(!computer.step()); // Second run halts
}
