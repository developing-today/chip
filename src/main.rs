struct Cpu {
    registers: [u8; 16],
    memory: [u8; 4096],
    position_in_memory: usize,
    stack: [u16; 16],
    stack_pointer: usize,
}
fn main() {
    let _cpu = Cpu {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };
}
