struct Cpu {
    registers: [u8; 16],
    memory: [u8; 4096],
    position_in_memory: usize,
    stack: [u16; 16],
    stack_pointer: usize,
}

/**
 * This object represents the CPU.
 * It contains all the registers and memory.
 * It also contains the stack and the stack pointer.
 * It also contains the current position in memory.
 * It also contains the current opcode.
 * It also contains the current instruction.
 * It also contains the current instruction's parameters.
 * It also contains the current instruction's parameter types.
 * It also contains the current instruction's parameter count.
 *
 * The CPU is the heart of the emulator.
 * It is the main object that is responsible for executing instructions.
 *
 * The CPU is responsible for:
 * - Reading the opcode from memory.
 * - Reading the parameters from memory.
 * - Executing the instruction.
 * - Writing the result to memory.
 * - Writing the result to the screen.
 * - Writing the result to the stack.
 * - Writing the result to the registers.
 * - Writing the result to the keyboard.
 * - Writing the result to the sound.
 * - Writing the result to the display.
 * - Writing the result to the screen.
 * - Writing the result to the screen.
 * - Writing the result to the screen.
 *
 * This object implements [CHIP-8 Virtual Machine [wikipedia]](https://en.wikipedia.org/wiki/CHIP-8#Virtual_machine_description)
 *
 * ## Opcodes
 *
 * CHIP-8 has 35 opcodes, which are all two bytes long and stored Big-ending.
 *
 * ### Symbol Table
 *
 * The symbols in the Opcode column of the Opcode table can be interpreted as follows:
 *
 * | Symbol | Explanation |
 * -------- | -------------
 * | NNN | address |
 * | NN | 8-bit constant |
 * | N | 4-bit constant |
 * | X | 4-bit register identifier |
 * | Y | 4-bit register identifier |
 * | PC | Program Counter |
 * | I | 16bit register (For memory address) (Similar to void pointer) |
 * | VN | One of the 16 available variables. N may be 0 to F (hexadecimal) |
 *
 * ### Opcode Table
 *
 * The table below lists the 35 CHIP-8 opcodes in hexadecimal.
 *
 * Of these opcodes, **1/35** are currently implemented in chip
 * (Emoji denote implementation status.)
 *
 * | Implemented | Opcode | Explanation |
 * -------- | -------------
 * || 0NNN | Calls RCA 1802 program at address NNN. Not necessary for most ROMs. |
 * || 00E0 | Clears the screen. |
 * || 00EE | Returns from a subroutine. |
 * || 1NNN | Jumps to address NNN. |
 * || 2NNN | Calls subroutine at NNN. |
 * || 3XNN | Skips the next instruction if VX equals NN. |
 * || 4XNN | Skips the next instruction if VX doesn't equal NN. |
 * || 5XY0 | Skips the next instruction if VX equals VY. |
 * || 6XNN | Sets VX to NN. |
 * || 7XNN | Adds NN to VX. |
 * || 8XY0 | Sets VX to the value of VY. |
 * || 8XY1 | Sets VX to VX or VY. |
 * || 8XY2 | Sets VX to VX and VY. |
 * || 8XY3 | Sets VX to VX xor VY. |
 * |🌱| 8XY4 | Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't. |
 * || 8XY5 | VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't. |
 * || 8XY6 | Shifts VX right by one. VF is set to the value of the least significant bit of VX before the shift. |
 * || 8XY7 | Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't. |
 * || 8XYE | Shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift.|
 * || 9XY0 | Skips the next instruction if VX doesn't equal VY. |
 * || ANNN | Sets I to the address NNN. |
 * || BNNN | Jumps to the address NNN plus V0. |
 * || CXNN | Sets VX to the result of a bitwise and operation on a random number and NN. |
 * || DXYN | Sprites stored in memory at location in index register (I), 8bits wide. Wraps around the screen. If when drawn, clears a pixel, register VF is set to 1 otherwise it is zero. All drawing is XOR drawing (i.e. |it toggles the screen pixels). Sprites are drawn starting at position VX, VY. N is the number of 8bit rows that need to be drawn. If N is greater than 1, second line continues at position VX, VY+1, and so on. |
 * || EX9E | Skips the next instruction if the key stored in VX is pressed. |
 * || EXA1 | Skips the next instruction if the key stored in VX isn't pressed. |
 * || FX07 | Sets VX to the value of the delay timer. |
 * || FX0A | A key press is awaited, and then stored in VX. |
 * || FX15 | Sets the delay timer to VX. |
 * || FX18 | Sets the sound timer to VX. |
 * || FX1E | Adds VX to I.</sup> |
 * || FX29 | Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font. |
 * || FX33 | Stores the Binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other |words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.) |
 * || FX55 | Stores V0 to VX in memory starting at address I.</sup> |
 * || FX65 | Fills V0 to VX with values from memory starting at address I.</sup> |
 *
 * [https://github.com/dezren39/chip](https://github.com/dezren39/chip)
 */
impl Cpu {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2 // combine the two bytes
    }
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0x0, _, _, _) => self.todo(),
                (0x0, 0x0, 0xE, 0x0) => self.todo(),
                (0x0, 0x0, 0xE, 0xE) => self.todo(),
                (0x1, _, _, _) => self.todo(),
                (0x2, _, _, _) => self.todo(),
                (0x3, _, _, _) => self.todo(),
                (0x4, _, _, _) => self.todo(),
                (0x5, _, _, 0x0) => self.todo(),
                (0x6, _, _, _) => self.todo(),
                (0x7, _, _, _) => self.todo(),
                (0x8, _, _, 0x0) => self.todo(),
                (0x8, _, _, 0x1) => self.todo(),
                (0x8, _, _, 0x2) => self.todo(),
                (0x8, _, _, 0x3) => self.todo(),
                (0x8, _, _, 0x5) => self.todo(),
                (0x8, _, _, 0x6) => self.todo(),
                (0x8, _, _, 0x7) => self.todo(),
                (0x8, _, _, 0xE) => self.todo(),
                (0x9, _, _, 0x0) => self.todo(),
                (0xA, _, _, _) => self.todo(),
                (0xB, _, _, _) => self.todo(),
                (0xC, _, _, _) => self.todo(),
                (0xD, _, _, _) => self.todo(),
                (0xE, _, 0x9, 0xE) => self.todo(),
                (0xE, _, 0xA, 0x1) => self.todo(),
                (0xF, _, 0x0, 0x7) => self.todo(),
                (0xF, _, 0x0, 0xA) => self.todo(),
                (0xF, _, 0x1, 0x5) => self.todo(),
                (0xF, _, 0x1, 0x8) => self.todo(),
                (0xF, _, 0x1, 0xE) => self.todo(),
                (0xF, _, 0x2, 0x9) => self.todo(),
                (0xF, _, 0x3, 0x3) => self.todo(),
                (0xF, _, 0x5, 0x5) => self.todo(),
                (0xF, _, 0x6, 0x5) => self.todo(),
            }
        }
    }

    fn todo(&mut self) -> ! {
        panic!("TODO")
    }
}

fn main() {
    let _cpu = &mut Cpu {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };
    _cpu.registers[0] = 1;
    _cpu.memory[0] = 0x01;
}
