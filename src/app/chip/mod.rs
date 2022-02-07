use bevy::{
    core::{Time, Timer},
    prelude::{Commands, Component, Query, Res, ResMut},
};

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
const STATUS_REGISTER: usize = 0xF;

pub(crate) struct CpuTimer(pub Timer);
pub(crate) struct AppTimer(pub Timer);
pub(crate) fn cpu_cycle(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<CpuTimer>,
    mut query: Query<(&mut Cpu, &mut super::screen::Screen)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let (mut cpu, mut screen) = query.single_mut();
        cpu.run(&mut commands, &mut screen);
    }
}
#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Cpu {
    pub(crate) registers: [usize; usize::BITS as usize],
    pub(crate) stack: [usize; usize::BITS as usize * 4],
    pub(crate) memory: [usize; 4096],
    pub(crate) counter: usize,
    pub(crate) pointer: usize,
    pub(crate) i: usize,
    pub(crate) delay: usize,
    pub(crate) sound: usize,
    pub(crate) keys: [bool; 16],
}
impl Cpu {
    fn read_opcode(&self) -> (usize, usize) {
        (self.memory[self.counter], self.memory[self.counter + 1])
    }
    fn on(&mut self, commands: &mut Commands, mut pixel: &mut super::screen::Pixel) {
        commands.entity(pixel.0).remove::<super::screen::Off>();
        commands.entity(pixel.0).remove::<super::screen::Disabled>();
        pixel.1 = true;
        // println!("{:?}", pixel);
    }
    fn off(&mut self, commands: &mut Commands, mut pixel: &mut super::screen::Pixel) {
        commands.entity(pixel.0).insert(super::screen::Disabled());
        pixel.1 = false;
        // println!("{:?}", pixel);
    }
    fn set(&mut self, commands: &mut Commands, pixel: &mut super::screen::Pixel, bit: bool) {
        if bit {
            self.on(commands, pixel);
        } else {
            self.off(commands, pixel);
        }
    }
    fn flp(&mut self, commands: &mut Commands, pixel: &mut super::screen::Pixel) {
        self.set(commands, pixel, !pixel.1);
    }
    fn clr(&mut self, mut commands: &mut Commands, screen: &mut super::screen::Screen) {
        for mut pixel in screen.0.as_mut() {
            self.off(&mut commands, &mut pixel);
        }
    }
    /// DXYN | Sprites stored in memory at location in index register (I), 8bits wide. Wraps around the screen. If when drawn, clears a pixel, register VF is set to 1 otherwise it is zero. All drawing is XOR drawing (i.e. it toggles the screen pixels). Sprites are drawn starting at position VX, VY. N is the number of 8bit rows that need to be drawn. If N is greater than 1, second line continues at position VX, VY+1, and so on.
    fn drw(
        &mut self,
        mut commands: &mut Commands,
        screen: &mut super::screen::Screen,
        x: usize,
        y: usize,
        n: usize,
    ) {
        let mut vf = false;
        for i in 0..n {
            let sprite_byte = self.memory[self.i + i];
            for j in 0..8 {
                let bit = (sprite_byte >> (7 - j)) & 1;
                let x = (x + j) % super::screen::SCREEN_X;
                let y = (y + i) % super::screen::SCREEN_Y;
                let pixel = screen.0.get_mut(x + y * super::screen::SCREEN_X).unwrap();
                if bit == pixel.1 as usize {
                    vf = true
                }
                self.flp(&mut commands, pixel);
            }
        }
        self.registers[STATUS_REGISTER] = vf as usize;
    }
    fn run(&mut self, commands: &mut Commands, screen: &mut super::screen::Screen) {
        if self.counter == 0 {
            // println!(
            //     "\n\n\n CYCLE\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tRUN",
            //     self.pointer,
            //     self.i,
            //     self.counter,
            //     &self.registers[0..16],
            //     &self.stack[0..16]
            // );
        }
        let opcode_bytes = self.read_opcode();
        self.counter += 2;

        self.flp(commands, &mut screen.0[1]);
        // println!("f1{:?}", screen.0[1]);

        let opcode = (opcode_bytes.0 as u8 as u16) << 8 | opcode_bytes.1 as u8 as u16;
        // println!(
        //     "\n    OP\t{:04X?}\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t{:04X?}\t{:04X?}\t{:04X?}\n",
        //     opcode, opcode, opcode, opcode
        // );
        let c = (((opcode & 0xF000) >> 12) as u8) as usize;
        let x = (((opcode & 0x0F00) >> 8) as u8) as usize;
        let y = (((opcode & 0x00F0) >> 4) as u8) as usize;
        let n = ((opcode & 0x000F) as u8) as usize;
        let kk = y << 4 | n;
        let addr = (opcode & 0x0FFF) as usize;

        match (c, x, y, n) {
            // when in doubt, trust the cowgod
            (0, 0, 0, 0) => {
                let hold = screen.0[1].1;
                // println!("h{:?}", hold);
                self.clr(commands, screen);
                // println!("c{:?}", screen.0[1]);
                self.set(commands, &mut screen.0[1], hold);
                // println!("s{:?}", screen.0[1]);

                // println!(
                //     "   END\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tEND\n",
                //     self.pointer,
                //     self.i,
                //     self.counter,
                //     &self.registers[0..16],
                //     &self.stack[0..16]
                // );
                // return;
            } // 0000 | Returns the program.
            (0x0, 0x0, 0xE, 0x0) => self.clr(commands, screen), // 00E0 | Clears the screen.
            (0x0, 0x0, 0xE, 0xE) => self.ret(),                 // 00EE | Returns from a subroutine.
            (0x0, _, _, _) => self._depr(), // 0NNN | Calls RCA 1802 program at address NNN. Not necessary for most ROMs.
            (0x1, _, _, _) => self.jp(addr), // 1NNN | Jumps to address NNN.
            (0x2, _, _, _) => self.call(addr), // 2NNN | Calls subroutine at address NNN.
            (0x3, _, _, _) => self.skp(self.registers[x] == kk), // 3XKK | Skips the next instruction if VX equals KK.
            (0x4, _, _, _) => self.skp(self.registers[x] != kk), // 4XKK | Skips the next instruction if VX doesn't equal KK.
            (0x5, _, _, 0x0) => self.skp(self.registers[x] == self.registers[y]), // 5XY0 | Skips the next instruction if VX equals VY.
            (0x6, _, _, _) => self.ldk(x, kk), // 6XKK | Sets VX to KK.
            (0x7, _, _, _) => self.addk(x, kk), // 7XKK | Adds KK to VX.
            (0x8, _, _, 0x0) => self.ld(x, y), // 8XY0 | Sets VX to the value of VY.
            (0x8, _, _, 0x1) => self.or(x, y), // 8XY1 | Sets VX to VX or VY.
            (0x8, _, _, 0x2) => self.and(x, y), // 8XY2 | Sets VX to VX and VY.
            (0x8, _, _, 0x3) => self.xor(x, y), // 8XY3 | Sets VX to VX xor VY.
            (0x8, _, _, 0x4) => self.add(x, y), // 8XY4 | Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
            (0x8, _, _, 0x5) => self.sub(x, y), // 8XY5 | VY is subtracted from VX. VF is set to 0 whedin there's a borrow, and 1 when there isn't.
            (0x8, _, _, 0x6) => self.shr(x), // 8XY6 | Shifts VX right by one. VF is set to the value of the least significant bit of VX before the shift.,y
            (0x8, _, _, 0x7) => self.subn(x, y), // 8XY7 | Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
            (0x8, _, _, 0xE) => self.shl(x), // 8XYE | Shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift
            (0x9, _, _, 0x0) => self.skp(self.registers[x] != self.registers[y]), // 9XY0 | Skips the next instruction if VX doesn't equal VY.
            (0xA, _, _, _) => self.ldik(addr), // ANNN | Sets I to the address NNN.
            (0xB, _, _, _) => self.jp0(addr),  // BNNN | Jumps to the address NNN plus V0.
            (0xC, _, _, _) => self.rnd(x, kk), // CXKK | Sets VX to the result of a bitwise and operation on a random number and KK.
            (0xD, _, _, _) => self.drw(commands, screen, x, y, n), // DXYN | Sprites stored in memory at location in index register (I), 8bits wide. Wraps around the screen. If when drawn, clears a pixel, register VF is set to 1 otherwise it is zero. All drawing is XOR drawing (i.e. it toggles the screen pixels). Sprites are drawn starting at position VX, VY. N is the number of 8bit rows that need to be drawn. If N is greater than 1, second line continues at position VX, VY+1, and so on.
            (0xE, _, 0x9, 0xE) => self.skp(self.keys[self.registers[x]]), // EX9E | Skips the next instruction if the key stored in VX is pressed.
            (0xE, _, 0xA, 0x1) => self.skp(!self.keys[self.registers[x]]), // EXA1 | Skips the next instruction if the key stored in VX isn't pressed.
            (0xF, _, 0x0, 0x7) => self.ldd(x), // FX07 | Sets VX to the value of the delay timer.
            // (0xF, _, 0x0, 0xA) => self.ldkb(x), // FX0A | A key press is awaited, and then stored in VX.
            (0xF, _, 0x1, 0x5) => self.lddv(x), // FX15 | Sets the delay timer to VX.
            (0xF, _, 0x1, 0x8) => self.ldsv(x), // FX18 | Sets the sound timer to VX.
            (0xF, _, 0x1, 0xE) => self.addiv(x), // FX1E | Adds VX to I.
            // (0xF, _, 0x2, 0x9) => self.ldiv(x), // FX29 | Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) ar
            // (0xF, _, 0x3, 0x3) => self.ldb(x), // FX33 | Stores the Binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at locaself.registers[x]tion I+2.)
            // (0xF, _, 0x5, 0x5) => self.ldir(x), // FX55 | Stores V0 to VX in memory starting at address I.
            // (0xF, _, 0x6, 0x5) => self.ldri(x), // FX65 | Fills V0 to VX with values from memory starting at address I.
            _ => panic!("Unimplemented opcode: {:04X?}", opcode), // _ | Panic.
                                                                  // moo.
        }
        // println!(
        //     "  LOOP\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
    }
    fn _todo(&mut self) -> ! {
        todo!("[TODO] Unimplemented opcode: {:04X?}", self.read_opcode());
    }
    fn _depr(&mut self) -> ! {
        panic!(
            "[deprecated] Unimplemented opcode: {:04X?}",
            self.read_opcode()
        );
    }
    fn skp(&mut self, bool: bool) {
        if bool {
            self.counter += 2;
        }
    }
    fn call(&mut self, addr: usize) {
        // println!(
        //     "  CALL\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tCALL\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        if self.pointer >= self.stack.len() {
            panic!("Stack overflow")
        }
        self.stack[self.pointer] = self.counter;
        self.pointer += 1;
        self.counter = addr;
    }
    fn ret(&mut self) {
        // println!(
        //     "   RET\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tRET\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        if self.pointer == 0 {
            // panic!("Stack underflow")
            self.counter = self.stack[self.pointer];
            return;
        }
        self.pointer -= 1;
        self.counter = self.stack[self.pointer];
    }
    fn jp(&mut self, addr: usize) {
        // println!(
        //     "  JUMP\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tJUMP\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        self.counter = addr;
    }
    fn add(&mut self, x: usize, y: usize) {
        // println!(
        //     "   ADD\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tADD\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        (self.registers[x], self.registers[STATUS_REGISTER]) = tuple_as!(
            (self.registers[x] as u8).overflowing_add(self.registers[y] as u8),
            usize
        );
    }
    fn addk(&mut self, x: usize, kk: usize) {
        // println!(
        //     "   ADD\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tADD\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        (self.registers[x], self.registers[STATUS_REGISTER]) =
            tuple_as!((self.registers[x] as u8).overflowing_add(kk as u8), usize);
    }
    fn sub(&mut self, x: usize, y: usize) {
        // println!(
        //     "   SUB\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSUB\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        (self.registers[x], self.registers[STATUS_REGISTER]) = tuple_as!(
            (self.registers[x] as u8).overflowing_sub(self.registers[y] as u8),
            usize
        );
    }
    fn subn(&mut self, x: usize, y: usize) {
        // println!(
        //     "   SUB\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSUB\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        (self.registers[x], self.registers[STATUS_REGISTER]) = tuple_as!(
            (self.registers[y] as u8).overflowing_sub(self.registers[x] as u8),
            usize
        );
    }
    fn shl(&mut self, x: usize) {
        // println!(
        //     "   SHL\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSHIFT\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        (self.registers[x], self.registers[STATUS_REGISTER]) = tuple_as!(
            (
                (self.registers[x] as u8) << 1,
                (self.registers[x] as u8) >> 7,
            ),
            usize
        );
    }
    fn shr(&mut self, x: usize) {
        // println!(
        //     "   SHR\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSHIFT\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        (self.registers[x], self.registers[STATUS_REGISTER]) = tuple_as!(
            (
                (self.registers[x] as u8) >> 1,
                (self.registers[x] as u8) << 7,
            ),
            usize
        );
    }
    fn ldk(&mut self, x: usize, kk: usize) {
        // println!(
        //     "   SET\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSET\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        self.registers[x] = kk;
    }
    fn ldik(&mut self, addr: usize) {
        // println!(
        //     "   SETI\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSET\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        self.i = addr;
    }
    fn ld(&mut self, x: usize, y: usize) {
        // println!(
        //     "   LDR\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSET\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        self.registers[x] = self.registers[y];
    }
    fn addiv(&mut self, x: usize) {
        // println!(
        //     "   LDI\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSET\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        self.i = self.stack[x];
    }
    // fn ldir(&mut self, x: usize) {
    //     // println!(
    //     //     "   LDI\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tSET\n",
    //     //     self.pointer,
    //     //     self.i,
    //     //     self.counter,
    //     //     &self.registers[0..16],
    //     //     &self.stack[0..16]
    //     // );
    //     self.stack[x] = self.i;
    // }
    fn or(&mut self, x: usize, y: usize) {
        // println!(
        //     "   OR\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tOR\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        self.registers[x] = self.registers[x] | self.registers[y];
    }
    fn xor(&mut self, x: usize, y: usize) {
        // println!(
        //     "   XOR\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tXOR\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        self.registers[x] = self.registers[x] ^ self.registers[y];
    }
    fn and(&mut self, x: usize, y: usize) {
        // println!(
        //     "   AND\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tAND\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        self.registers[x] = self.registers[x] & self.registers[y];
    }
    fn ldd(&mut self, x: usize) {
        // println!(
        //     "   LDDT\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tLDDT\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        self.registers[x] = self.delay;
    }
    fn lddv(&mut self, x: usize) {
        // println!(
        //     "   LDDTV\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tLDDT\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        self.delay = self.stack[x];
    }
    fn ldsv(&mut self, x: usize) {
        // println!(
        //     "   LDST\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tLDST\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        self.sound = self.registers[x];
    }
    fn jp0(&mut self, x: usize) {
        // println!(
        //     "   JPV0\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tJPV0\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        self.pointer = self.registers[x];
    }
    fn rnd(&mut self, x: usize, y: usize) {
        // println!(
        //     "   RND\tp:{:?}\ti:{:?}\tc:{:04X?}\tr:{:?}\ts:{:X?}\tRND\n",
        //     self.pointer,
        //     self.i,
        //     self.counter,
        //     &self.registers[0..16],
        //     &self.stack[0..16]
        // );
        self.registers[x] = rand::random::<usize>() & self.registers[y];
    }
}
pub(crate) fn default_memory() -> [usize; 4096] {
    let mut memory = [0; 4096];
    memory[0x0] = 0x21; // call fn add twice // 2NNN | Calls subroutine at NNN.
    memory[0x2] = 0x21; // call fn sub twice // 2NNN | Calls subroutine at NNN.
    memory[0x3] = 0x06;
    memory[0x4] = 0x21; // call fn add twice then sub twice // 2NNN | Calls subroutine at NNN.
    memory[0x5] = 0x0C;
    // leave two bytes for the program return // 0000 | Returns the program.

    // fn add twice
    memory[0x100] = 0x80; // 8XY4 | Adds VY to VX. VF is set to 1 when there's a carry,
    memory[0x101] = 0x14;
    memory[0x102] = 0x80; // 8XY4 | Adds VY to VX. VF is set to 1 when there's a carry,
    memory[0x103] = 0x14;
    memory[0x105] = 0xEE; // leave two bytes for the subroutine return // 00EE | Returns from a subroutine.

    // fn sub twice
    memory[0x106] = 0x80; // 8XY5 | VY is subtracted from VX. VF is set to 0 whedin there's a borrow, and 1 when there isn't.
    memory[0x107] = 0x15;
    memory[0x108] = 0x80; // 8XY5 | VY is subtracted from VX. VF is set to 0 whedin there's a borrow, and 1 when there isn't.
    memory[0x109] = 0x15;
    memory[0x10B] = 0xEE; // leave two bytes for the subroutine return // 00EE | Returns from a subroutine.

    // fn add twice then call sub twice
    memory[0x10C] = 0x21; // call fn add twice // 2NNN | Calls subroutine at NNN.
    memory[0x10E] = 0x21; // call fn sub twice // 2NNN | Calls subroutine at NNN.
    memory[0x10F] = 0x06;
    // mem[0x10F] = 0x0C;
    memory[0x111] = 0xEE; // leave two bytes for the subroutine return // 00EE | Returns from a subroutine.
    memory
}
pub(crate) fn default_registers() -> [usize; usize::BITS as usize] {
    let mut registers = [0; usize::BITS as usize];
    registers[0] = 42;
    registers[1] = 5;
    registers
}
