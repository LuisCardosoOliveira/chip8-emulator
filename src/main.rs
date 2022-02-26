struct Cpu {
    current_operation: u16,
    registers: [u8; 2],
}

impl Cpu {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }

    fn run(&mut self) {
        let opcode = self.read_opcode();

        // opcode decoding
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = (opcode & 0x000F) as u8;

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }
    }
}

fn main() {
    // Initializes CPU
    let mut cpu = Cpu {
        current_operation: 0,
        registers: [0; 2],
    };

    // opcode that the CPU will interpret and it can be read like that
    // 8 -> signifies that the operation involves two registers
    // 0 -> maps to `cpu.registers[0]`
    // 1 -> maps to `cpu.registers[1]`
    // 4 -> indicates addition
    cpu.current_operation = 0x8014;

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 15);

    println!("5 + 10 = {}", cpu.registers[0]);
}
