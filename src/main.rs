use bevy::prelude::*; //, utils::HashSet}
use bevy_prototype_lyon::prelude::*;
use rand::Rng; // prelude::SliceRandom,
               // use std::{
               //     env::VarError,
               //     io::{self, BufRead, BufReader},
               //     process::Stdio,
               // };

macro_rules! tuple_as {
    ($t: expr, $ty: ident) => {{
        let (a, b) = $t;
        let a = a as $ty;
        let b = b as $ty;
        (a, b)
    }};
    ($t: expr, ($ty: ident)) => {{
        let (a, b) = $t;
        let a = a as $ty;
        let b = b as $ty;
        (a, b)
    }};
    ($t: expr, ($($ty: ident),*)) => {{
        let ($($ty,)*) = $t;
        ($($ty as $ty,)*)
    }};
}

/// 0xF is the last register and is used as the carry flag
const STATUS_REGISTER: usize = 0xF;

#[derive(Debug, Clone, Copy)]
struct Cpu {
    /// programs should not use 0xF, the original last register, as a general purpose register
    /// STATUS_REGISTER is used as the carry flag and is the original last register, 0xF
    // TODO: usize but should be u8 compatible with the original chip8 implementation, shift, etc. overflow?
    registers: [usize; usize::BITS as usize],
    /// diverges from spec, originally 16, 256
    /// this allows larger programs to be run
    /// diverges from the spec, originally u16, usize allows for rust indexing
    stack: [usize; usize::BITS as usize * 4],
    /// diverges from the spec, originally 0x0-0x2 was reserved for system use, this is unneeded in this implementation, and so all memory is accessible.
    /// diverges from the spec, originally u8, usize allows for rust indexing
    /// this works because the spec only allows for 16 bit memory access
    /// but this can be handled by casting to u16 for opcode and
    // todo allow pass-in of memory size and use that instead?
    memory: [usize; 4096],
    /// diverges from the spec, originally u16, usize allows for rust indexing
    counter: usize,
    /// diverges from the spec, originally u8, usize allows for rust indexing
    pointer: usize,
    /// diverges from the spec, originally u16, usize allows for rust indexing
    i: usize,
    /// diverges from the spec, originally u8, usize allows for rust indexing
    delay: usize,
    /// diverges from the spec, originally u8, usize allows for rust indexing
    sound: usize,
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
 * This object implements CHIP>8 Virtual Machine wikipedia](https://e8.wikipedia.org/wiki/CHIP-8#Virtual_machine_description)
 *
 * ## Opcodes
 *
 * CHIP-8 has 35 opcodes, which are all two bytes long and stored Big-ending.
 *
 * ### Symbol Table
 *
 * The symbols in the Opcode column of the Opcode table can be interpreted as follows:
 *8 * | Symbol | Explanation |
 * -------- | -------------
 * | NNN | address |
 * | KK | 8-bit constant |
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
 * |🌻| 0000 | EXIT, Returns the program.
 * || 00E0 | CLS, Clears the screen.
 * || 00EE | RET, Returns from a subroutine.
 * |☠️| 0NNN | SYS, Calls RCA 1802 program at address NNN. Not necessary for most ROMs.
 * || 1NNN | JP, Jumps to address NNN.
 * || 2NNN | CALL, Calls subroutine at address NNN.
 * || 3XKK | SEKK, Skips the next instruction if VX equals KK.
 * || 4XKK | SNEKK, Skips the next instruction if VX doesn't equal KK.
 * || 5XY0 | SE Skips the next instruction if VX equals VY.
 * || 6XKK | LDKK, Sets VX to KK.
 * || 7XKK | ADDKK Adds KK to VX.
 * || 8XY0 | LD, Sets VX to the value of VY.
 * || 8XY1 | LDOR, Sets VX to VX or VY.
 * || 8XY2 | LDAND, Sets VX to VX and VY.
 * || 8XY3 | LDXOR, Sets VX to VX xor VY.
 * |🌻| 8XY4 | ADD, Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
 * |🌻| 8XY5 | SUB, VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
 * |🌱| 8XY6 | SHR, Shifts VX right by one. VF is set to the value of the least significant bit of VX before the shift.
 * || 8XY7 | SUBN, Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
 * |🌱| 8XYE | SHL, Shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift.|
 * || 9XY0 | SNE, Skips the next instruction if VX doesn't equal VY.
 * || ANNN | LDI, Sets I to the address NNN.
 * || BNNN | JP0, Jumps to the address NNN plus V0.
 * || CXKK | RNDKK, Sets VX to the result of a bitwise and operation on a random number and KK.
 * || DXYN | DRWN, Sprites stored in memory at location in index register (I), 8bits wide. Wraps around the screen. If when drawn, clears a pixel, register VF is set to 1 otherwise it is zero. All drawing is XOR drawing (i.e. it toggles the screen pixels). Sprites are drawn starting at position VX, VY. N is the number of 8bit rows that need to be drawn. If N is greater than 1, second line continues at position VX, VY+1, and so on.
 * || EX9E | SKP, Skips the next instruction if the key stored in VX is pressed.
 * || EXA1 | SKNP, Skips the next instruction if the key stored in VX isn't pressed.
 * || FX07 | LDDT, Sets VX to the value of the delay timer.
 * || FX0A | LDK, A key press is awaited, and then stored in VX.
 * || FX15 | LDDTX, Sets the delay timer to VX.
 * || FX18 | LDST, Sets the sound timer to VX.
 * || FX1E | ADDI, Adds VX to I. |
 * || FX29 | LDF, Sets I to the location (F) of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
 * || FX33 | LDB, Stores the Binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other |words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)
 * || FX55 | LDI, Stores V0 to VX in memory starting at address I.
 * || FX65 | LDXI, Fills V0 to VX with values from memory starting at address I.
 * |🌻| _ | PANIC,.
 *
 * https>//github.com/dezren39/chip](https://g8thub.com/dezren39/chip)
 */
impl Cpu {
    fn read_opcode(&self) -> (usize, usize) {
        (self.memory[self.counter], self.memory[self.counter + 1]) // combine the two bytes
    }
    fn run(&mut self) {
        println!(
            "\n\n\n   RUN\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tRUN",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        loop {
            let opcode_bytes = self.read_opcode();
            self.counter += 2;

            let opcode = (opcode_bytes.0 as u8 as u16) << 8 | opcode_bytes.1 as u8 as u16;
            println!(
                "\n    OP\t{:04X?}\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t{:04X?}\t{:04X?}\t{:04X?}\n",
                opcode, opcode, opcode, opcode
            );
            let c = (((opcode & 0xF000) >> 12) as u8) as usize;
            let x = (((opcode & 0x0F00) >> 8) as u8) as usize;
            let y = (((opcode & 0x00F0) >> 4) as u8) as usize;
            let n = ((opcode & 0x000F) as u8) as usize;
            let kk = y << 4 | n;
            let addr = (opcode & 0x0FFF) as usize;

            match (c, x, y, n) {
                (0, 0, 0, 0) => {
                    println!(
                        "   END\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tEND\n",
                        self.pointer,
                        self.i,
                        self.counter,
                        &self.registers[0..16],
                        &self.stack[0..16]
                    );
                    return;
                } // 0000 | Returns the program.
                (0x0, 0x0, 0xE, 0x0) => self._todo(), // 00E0 | Clears the screen.
                (0x0, 0x0, 0xE, 0xE) => self.ret(),   // 00EE | Returns from a subroutine.
                (0x0, _, _, _) => self._deprecated(), // 0NNN | Calls RCA 1802 program at address NNN. Not necessary for most ROMs.
                (0x1, _, _, _) => self.jump(addr),    // 1NNN | Jumps to address NNN.
                (0x2, _, _, _) => self.call(addr),    // 2NNN | Calls subroutine at address NNN.
                (0x3, _, _, _) => self.skip_if_equal(x, kk), // 3XKK | Skips the next instruction if VX equals KK.
                (0x4, _, _, _) => self.skip_if_not_equal(x, kk), // 4XKK | Skips the next instruction if VX doesn't equal KK.
                (0x5, _, _, 0x0) => self.skip_if_equal(x, y), // 5XY0 | Skips the next instruction if VX equals VY.
                (0x6, _, _, _) => self.ld_kk(x, kk),          // 6XKK | Sets VX to KK.
                (0x7, _, _, _) => self.add_kk(x, kk),         // 7XKK | Adds KK to VX.
                (0x8, _, _, 0x0) => self.ld_r(x, y),          // 8XY0 | Sets VX to the value of VY.
                (0x8, _, _, 0x1) => self.or(x, y),            // 8XY1 | Sets VX to VX or VY.
                (0x8, _, _, 0x2) => self.and(x, y),           // 8XY2 | Sets VX to VX and VY.
                (0x8, _, _, 0x3) => self.xor(x, y),           // 8XY3 | Sets VX to VX xor VY.
                (0x8, _, _, 0x4) => self.add(x, y), // 8XY4 | Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
                (0x8, _, _, 0x5) => self.sub(x, y), // 8XY5 | VY is subtracted from VX. VF is set to 0 whedin there's a borrow, and 1 when there isn't.
                (0x8, _, _, 0x6) => self.shift_right(x), // 8XY6 | Shifts VX right by one. VF is set to the value of the least significant bit of VX before the shift.,y
                (0x8, _, _, 0x7) => self.subn(x, y), // 8XY7 | Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                (0x8, _, _, 0xE) => self.shift_left(x), // 8XYE | Shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift
                (0x9, _, _, 0x0) => self.skip_if_not_equal(x, y), // 9XY0 | Skips the next instruction if VX doesn't equal VY.
                (0xA, _, _, _) => self.ld_i(addr), // ANNN | Sets I to the address NNN.
                (0xB, _, _, _) => self.jp_v0(addr), // BNNN | Jumps to the address NNN plus V0.
                (0xC, _, _, _) => self.rnd(x, kk), // CXKK | Sets VX to the result of a bitwise and operation on a random number and KK.
                (0xD, _, _, _) => self._todo(), // DXYN | Sprites stored in memory at location in index register (I), 8bits wide. Wraps around the screen. If when drawn, clears a pixel, register VF is set to 1 otherwise it is zero. All drawing is XOR drawing (i.e. it toggles the screen pixels). Sprites are drawn starting at position VX, VY. N is the number of 8bit rows that need to be drawn. If N is greater than 1, second line continues at position VX, VY+1, and so on.
                (0xE, _, 0x9, 0xE) => self._todo(), // EX9E | Skips the next instruction if the key stored in VX is pressed.
                (0xE, _, 0xA, 0x1) => self._todo(), // EXA1 | Skips the next instruction if the key stored in VX isn't pressed.
                (0xF, _, 0x0, 0x7) => self.ld_dt(x), // FX07 | Sets VX to the value of the delay timer.
                (0xF, _, 0x0, 0xA) => self._todo(), // FX0A | A key press is awaited, and then stored in VX.
                (0xF, _, 0x1, 0x5) => self.ld_dt_v(x), // FX15 | Sets the delay timer to VX.
                (0xF, _, 0x1, 0x8) => self.ld_st_v(x), // FX18 | Sets the sound timer to VX.
                (0xF, _, 0x1, 0xE) => self.ld_i_v(x), // FX1E | Adds VX to I.
                (0xF, _, 0x2, 0x9) => self._todo(), // FX29 | Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) ar
                (0xF, _, 0x3, 0x3) => self._todo(), // FX33 | Stores the Binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)
                (0xF, _, 0x5, 0x5) => self.ld_v_i(x), // FX55 | Stores V0 to VX in memory starting at address I.
                (0xF, _, 0x6, 0x5) => self.ld_i_v(x), // FX65 | Fills V0 to VX with values from memory starting at address I.
                _ => panic!("Unimplemented opcode: {:04X?}", opcode), // _ | Panic.
            }
            println!(
                "  LOOP\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}",
                self.pointer,
                self.i,
                self.counter,
                &self.registers[0..16],
                &self.stack[0..16]
            );
        }
    }
    fn _todo(&mut self) -> ! {
        panic!("[TODO] Unimplemented opcode: {:04X?}", self.read_opcode());
    }
    fn _deprecated(&mut self) -> ! {
        panic!(
            "[deprecated] Unimplemented opcode: {:04X?}",
            self.read_opcode()
        );
    }
    fn call(&mut self, addr: usize) {
        println!(
            "  CALL\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tCALL\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        if self.pointer >= self.stack.len() {
            panic!("Stack overflow")
        }
        self.stack[self.pointer] = self.counter;
        self.pointer += 1;
        self.counter = addr;
    }
    fn ret(&mut self) {
        println!(
            "   RET\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tRET\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        if self.pointer == 0 {
            panic!("Stack underflow")
        }
        self.pointer -= 1;
        self.counter = self.stack[self.pointer];
    }
    fn jump(&mut self, addr: usize) {
        println!(
            "  JUMP\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tJUMP\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        self.counter = addr;
    }

    fn skip_if_equal(&mut self, x: usize, kk: usize) {
        println!(
            "    SE\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSKIP\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        if self.registers[x] == kk {
            self.counter += 2;
        }
    }

    fn skip_if_not_equal(&mut self, x: usize, kk: usize) {
        println!(
            "   SNE\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSKIP\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        if self.registers[x] != kk {
            self.counter += 2;
        }
    }

    fn add(&mut self, x: usize, y: usize) {
        println!(
            "   ADD\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tADD\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        (self.registers[x], self.registers[STATUS_REGISTER]) = tuple_as!(
            (self.registers[x] as u8).overflowing_add(self.registers[y] as u8),
            usize
        );
    }

    fn add_kk(&mut self, x: usize, kk: usize) {
        println!(
            "   ADD\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tADD\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        (self.registers[x], self.registers[STATUS_REGISTER]) =
            tuple_as!((self.registers[x] as u8).overflowing_add(kk as u8), usize);
    }

    fn sub(&mut self, x: usize, y: usize) {
        println!(
            "   SUB\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSUB\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        (self.registers[x], self.registers[STATUS_REGISTER]) = tuple_as!(
            (self.registers[x] as u8).overflowing_sub(self.registers[y] as u8),
            usize
        );
    }

    fn subn(&mut self, x: usize, y: usize) {
        println!(
            "   SUB\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSUB\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        (self.registers[x], self.registers[STATUS_REGISTER]) = tuple_as!(
            (self.registers[y] as u8).overflowing_sub(self.registers[x] as u8),
            usize
        );
    }

    fn shift_left(&mut self, x: usize) {
        println!(
            "   SHL\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSHIFT\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        (self.registers[x], self.registers[STATUS_REGISTER]) = tuple_as!(
            (
                (self.registers[x] as u8) << 1,
                (self.registers[x] as u8) >> 7,
            ),
            usize
        );
    }

    fn shift_right(&mut self, x: usize) {
        println!(
            "   SHR\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSHIFT\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        (self.registers[x], self.registers[STATUS_REGISTER]) = tuple_as!(
            (
                (self.registers[x] as u8) >> 1,
                (self.registers[x] as u8) << 7,
            ),
            usize
        );
    }

    fn ld_kk(&mut self, x: usize, kk: usize) {
        println!(
            "   SET\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSET\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        self.registers[x] = kk;
    }

    fn ld_i(&mut self, addr: usize) {
        println!(
            "   SETI\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSET\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        self.i = addr;
    }

    fn ld_r(&mut self, x: usize, y: usize) {
        println!(
            "   LDR\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSET\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        self.registers[x] = self.registers[y];
    }

    fn ld_v(&mut self, x: usize, y: usize) {
        println!(
            "   LDR\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSET\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        self.registers[x] = self.stack[y];
    }

    fn ld_i_v(&mut self, x: usize) {
        println!(
            "   LDI\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSET\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        self.i = self.stack[x];
    }

    fn ld_v_i(&mut self, x: usize) {
        println!(
            "   LDI\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSET\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        self.stack[x] = self.i;
    }

    fn or(&mut self, x: usize, y: usize) {
        println!(
            "   OR\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tOR\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        self.registers[x] = self.registers[x] | self.registers[y];
    }

    fn xor(&mut self, x: usize, y: usize) {
        println!(
            "   XOR\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tXOR\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        self.registers[x] = self.registers[x] ^ self.registers[y];
    }

    fn and(&mut self, x: usize, y: usize) {
        println!(
            "   AND\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tAND\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        self.registers[x] = self.registers[x] & self.registers[y];
    }

    fn ld_dt(&mut self, x: usize) {
        println!(
            "   LDDT\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tLDDT\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        self.registers[x] = self.delay;
    }

    fn ld_dt_v(&mut self, x: usize) {
        println!(
            "   LDDTV\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tLDDT\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        self.delay = self.stack[x];
    }

    fn ld_st_v(&mut self, x: usize) {
        println!(
            "   LDST\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tLDST\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        self.sound = self.registers[x];
    }

    fn jp_v0(&mut self, x: usize) {
        println!(
            "   JPV0\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tJPV0\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        self.pointer = self.registers[x];
    }

    fn rnd(&mut self, x: usize, y: usize) {
        println!(
            "   RND\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tRND\n",
            self.pointer,
            self.i,
            self.counter,
            &self.registers[0..16],
            &self.stack[0..16]
        );
        self.registers[x] = rand::random::<usize>() & self.registers[y];
    }
}

fn main() {
    let cpu = &mut Cpu {
        registers: [0; usize::BITS as usize],
        stack: [0; usize::BITS as usize * 4],
        memory: [0; 4096],
        counter: 0,
        pointer: 0,
        i: 0,
        delay: 0,
        sound: 0,
    };

    cpu.registers[0] = 42;
    cpu.registers[1] = 5;

    let mem = &mut cpu.memory;
    mem[0x0] = 0x21; // call fn add twice // 2NNN | Calls subroutine at NNN.
    mem[0x2] = 0x21; // call fn sub twice // 2NNN | Calls subroutine at NNN.
    mem[0x3] = 0x06;
    mem[0x4] = 0x21; // call fn add twice then sub twice // 2NNN | Calls subroutine at NNN.
    mem[0x5] = 0x0C;
    // leave two bytes for the program return // 0000 | Returns the program.

    // fn add twice
    mem[0x100] = 0x80; // 8XY4 | Adds VY to VX. VF is set to 1 when there's a carry,
    mem[0x101] = 0x14;
    mem[0x102] = 0x80; // 8XY4 | Adds VY to VX. VF is set to 1 when there's a carry,
    mem[0x103] = 0x14;
    mem[0x105] = 0xEE; // leave two bytes for the subroutine return // 00EE | Returns from a subroutine.

    // fn sub twice
    mem[0x106] = 0x80; // 8XY5 | VY is subtracted from VX. VF is set to 0 whedin there's a borrow, and 1 when there isn't.
    mem[0x107] = 0x15;
    mem[0x108] = 0x80; // 8XY5 | VY is subtracted from VX. VF is set to 0 whedin there's a borrow, and 1 when there isn't.
    mem[0x109] = 0x15;
    mem[0x10B] = 0xEE; // leave two bytes for the subroutine return // 00EE | Returns from a subroutine.

    // fn add twice then call sub twice
    mem[0x10C] = 0x21; // call fn add twice // 2NNN | Calls subroutine at NNN.
    mem[0x10E] = 0x21; // call fn sub twice // 2NNN | Calls subroutine at NNN.
    mem[0x10F] = 0x06;
    // mem[0x10F] = 0x0C;
    mem[0x111] = 0xEE; // leave two bytes for the subroutine return // 00EE | Returns from a subroutine.

    println!("\n\n\n   MEM\t{:X?}", &mem[0..1024]);
    cpu.run();
    println!("\n\n\n   MEM\t{:X?}\n\n\n", &cpu.memory[0..1024]);

    println!("{}", cpu.registers[0]);
    assert_eq!(cpu.registers[0], 42);

    cpu.memory = [0; 4096];

    const SCREEN_X: usize = 64;
    const SCREEN_Y: usize = 32;
    #[derive(Debug, Component)]
    struct Screen;

    #[derive(Component)]
    struct Pixels([u8; SCREEN_X * SCREEN_Y]);

    fn add_screen(mut commands: Commands) {
        commands
            .spawn()
            .insert(Screen)
            .insert(Pixels([0; SCREEN_X * SCREEN_Y]));
    }
    fn hello_world() {
        println!(
            "hello world!" // {:?}",
                           // Screen {
                           //     px: [0; SCREEN_X * SCREEN_Y]
                           // }
                           // .px
                           // .len()
        );
    }
    App::new().add_system(hello_world).run();
    struct GreetTimer(Timer);

    fn greet_pixels(
        time: Res<Time>,
        mut timer: ResMut<GreetTimer>,
        query: Query<&Pixels, With<Screen>>,
    ) {
        // update our timer with the time elapsed since the last update
        // if that caused the timer to finish, we say hello to everyone
        if timer.0.tick(time.delta()).just_finished() {
            for px in query.iter() {
                for i in px.0.iter() {
                    println!("hello px:{}!", i);
                }
            }
        }
    }
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_startup_system(add_screen)
        .add_startup_system(setup_system)
        .add_system(greet_pixels)
        // .add_system(hello_world)
        .run();
}

fn setup_system(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 4,
        feature: shapes::RegularPolygonFeature::Radius(25.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::BLACK, 10.0),
        },
        Transform::default(),
    ));
}
fn setup_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load contributors from the git history log or use default values from
    // the constant array. Contributors must be unique, so they are stored in a HashSet
    let contribs = contributors().unwrap_or_else(|_| {
        CONTRIBUTORS_LIST
            .iter()
            .map(|name| name.to_string())
            .collect()
    });

    let texture_handle = asset_server.load("branding/icon.png");

    let mut contributor_selection = ContributorSelection {
        order: vec![],
        idx: 0,
    };

    let mut rnd = rand::thread_rng();

    for name in contribs {
        let pos = (rnd.gen_range(-400.0..400.0), rnd.gen_range(0.0..400.0));
        let dir = rnd.gen_range(-1.0..1.0);
        let velocity = Vec3::new(dir * 500.0, 0.0, 0.0);
        let hue = rnd.gen_range(0.0..=360.0);

        // some sprites should be flipped
        let flipped = rnd.gen_bool(0.5);

        let transform = Transform::from_xyz(pos.0, pos.1, 0.0);

        let entity = commands
            .spawn()
            .insert_bundle((
                Contributor { hue },
                Velocity {
                    translation: velocity,
                    rotation: -dir * 5.0,
                },
            ))
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0) * SPRITE_SIZE),
                    color: Color::hsla(hue, SATURATION_DESELECTED, LIGHTNESS_DESELECTED, ALPHA),
                    flip_x: flipped,
                    ..Default::default()
                },
                texture: texture_handle.clone(),
                transform,
                ..Default::default()
            })
            .id();

        contributor_selection.order.push((name, entity));
    }

    contributor_selection.order.shuffle(&mut rnd);

    commands.insert_resource(contributor_selection);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle((SelectTimer, Timer::from_seconds(SHOWCASE_TIMER_SECS, true)));

    commands
        .spawn()
        .insert(ContributorDisplay)
        .insert_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Contributor showcase".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        });
}
