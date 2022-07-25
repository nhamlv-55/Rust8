struct Chip8{
    opcode: i16,
    memory: [i8; 4096],
    registers: [i8; 16],
    index_reg: i16,
    pc_reg: i16,
    screen: [bool; 64*32],
    delay_timer: char,
    sound_timer: char,
    stack: [i16; 16],
    sp: i16,
    key: [i16; 16]
}

impl Chip8{
    fn new() ->Chip8{
        Chip8{
            opcode: 0,
            memory: [0; 4096],
            registers: [0; 16],
            index_reg: 0,
            pc_reg: 0,
            screen: [false; 64*32],
            delay_timer: 'a',
            sound_timer: 'a',
            stack: [0; 16],
            sp: 0,
            key: [0; 16]
        }
    }

    fn draw(self){
        for row in (0..31){
            for column in (0..63){
                if self.screen[64*row+column]{
                    print!(" ");
                }else{
                    print!("\u{25A0}");
                }
            }
            print!("\n");
        }
    }
}



fn main() {
    let chip8 = Chip8::new();
    chip8.draw();
}
