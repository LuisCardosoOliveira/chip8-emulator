struct Cpu {
    registers: [u8; 16],
    position_in_memory: usize,
    // represents 4096 bytes of RAM
    memory: [u8; 4096],
    // stack's max height is 16. After 16 nested function calls, the program
    // encounters a stack overflow.
    stack: [u16; 16],
    stack_pointer: usize,
}

impl Cpu {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        // This creates a u16 opcode combining two values
        // from `memory` with the logical OR operation.
        // These need to be cast as u16 to start with
        // otherwise, the left shift sets all of the bits to 0.
        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            // continues execution beyond processing a single execution
            let opcode = self.read_opcode();
            // increment `position_in_memory` to point to next instruction
            self.position_in_memory += 2;

            // opcode decoding
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = (opcode & 0x000F) as u8;

            let nnn = opcode & 0x0FFF;

            match (c, x, y, d) {
                // terminate execution when opcode === 0x0000
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!")
        }

        // Adds the current `position_in_memory` to the stack.
        stack[sp] = self.position_in_memory as u16;

        // Increments `self.stack_pointer` to prevent `self.position_in_memory`
        // from being overwritten until it needs to be accessed again in a
        // subsequent return
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;

        // Jumps to position in memory where an earlier call was made
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

fn main() {
    // Initializes CPU
    let mut cpu = Cpu {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    // Sets opcode to 0x2100: CALL the function at 0x100
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;
    // Sets opcode to 0x2100: CALL the function at 0x100
    mem[0x002] = 0x21;
    mem[0x003] = 0x00;
    // Sets opcode to 0x0000: HALT
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;

    // Sets opcode to 0x8014: ADD register 1's value to register °
    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    // Sets opcode to 0x8014: ADD register 1's value to register °
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;

    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
