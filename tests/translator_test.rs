use jackc::asm;
use jackc::hack::Emulator;
use jackc::vm::{Command, Segment, Translator};

fn translate_and_assemble(cmds: &[Command], init: Option<String>) -> Vec<u16> {
    let mut translator = Translator::new(cmds).init(init);
    asm::assemble(translator.translate())
}

#[test]
fn simple_add_test() {
    let rom = translate_and_assemble(
        &[
            Command::Push(Segment::Constant, 7),
            Command::Push(Segment::Constant, 8),
            Command::Add,
        ],
        None,
    );

    let mut emulator = Emulator::new(&rom);
    emulator.ram.init(&[(0, 256)]);
    emulator.run(60);

    assert_eq!(emulator.ram.get(0), 257);
    assert_eq!(emulator.ram.get(256), 15);
}

#[test]
fn consts_test() {
    let rom = translate_and_assemble(
        &[
            Command::Push(Segment::Constant, 32768), // 2^15
            Command::Push(Segment::Constant, 16384), // 2^14
            Command::Add,
            Command::Push(Segment::Constant, 65535), // 2^16 - 1 = 0xffff
            Command::Push(Segment::Constant, 32767), // 2^15 - 1 = 0x7fff
            Command::And,
        ],
        None,
    );

    let mut emulator = Emulator::new(&rom);
    emulator.ram.init(&[(0, 256)]);
    emulator.run(60);

    assert_eq!(emulator.ram.get(0), 258);
    assert_eq!(emulator.ram.get(256), 49152);
    assert_eq!(emulator.ram.get(257), 32767);
}

#[test]
fn stack_test() {
    let rom = translate_and_assemble(
        &[
            Command::Push(Segment::Constant, 17),
            Command::Push(Segment::Constant, 17),
            Command::Eq,
            Command::Push(Segment::Constant, 17),
            Command::Push(Segment::Constant, 16),
            Command::Eq,
            Command::Push(Segment::Constant, 16),
            Command::Push(Segment::Constant, 17),
            Command::Eq,
            Command::Push(Segment::Constant, 892),
            Command::Push(Segment::Constant, 891),
            Command::Lt,
            Command::Push(Segment::Constant, 891),
            Command::Push(Segment::Constant, 892),
            Command::Lt,
            Command::Push(Segment::Constant, 891),
            Command::Push(Segment::Constant, 891),
            Command::Lt,
            Command::Push(Segment::Constant, 32767),
            Command::Push(Segment::Constant, 32766),
            Command::Gt,
            Command::Push(Segment::Constant, 32766),
            Command::Push(Segment::Constant, 32767),
            Command::Gt,
            Command::Push(Segment::Constant, 32766),
            Command::Push(Segment::Constant, 32766),
            Command::Gt,
            Command::Push(Segment::Constant, 57),
            Command::Push(Segment::Constant, 31),
            Command::Push(Segment::Constant, 53),
            Command::Add,
            Command::Push(Segment::Constant, 112),
            Command::Sub,
            Command::Neg,
            Command::And,
            Command::Push(Segment::Constant, 82),
            Command::Or,
            Command::Not,
        ],
        None,
    );

    let mut emulator = Emulator::new(&rom);
    emulator.ram.init(&[(0, 256)]);
    emulator.run(1000);

    assert_eq!(emulator.ram.get(0), 266);
    assert_eq!(emulator.ram.get(256), -1i16 as u16);
    assert_eq!(emulator.ram.get(257), 0);
    assert_eq!(emulator.ram.get(258), 0);
    assert_eq!(emulator.ram.get(259), 0);
    assert_eq!(emulator.ram.get(260), -1i16 as u16);
    assert_eq!(emulator.ram.get(261), 0);
    assert_eq!(emulator.ram.get(262), -1i16 as u16);
    assert_eq!(emulator.ram.get(263), 0);
    assert_eq!(emulator.ram.get(264), 0);
    assert_eq!(emulator.ram.get(265), -91i16 as u16);
}

#[test]
fn basic_test() {
    let rom = translate_and_assemble(
        &[
            Command::Push(Segment::Constant, 10),
            Command::Pop(Segment::Local, 0),
            Command::Push(Segment::Constant, 21),
            Command::Push(Segment::Constant, 22),
            Command::Pop(Segment::Argument, 2),
            Command::Pop(Segment::Argument, 1),
            Command::Push(Segment::Constant, 36),
            Command::Pop(Segment::This, 6),
            Command::Push(Segment::Constant, 42),
            Command::Push(Segment::Constant, 45),
            Command::Pop(Segment::That, 5),
            Command::Pop(Segment::That, 2),
            Command::Push(Segment::Constant, 510),
            Command::Pop(Segment::Temp, 6),
            Command::Push(Segment::Local, 0),
            Command::Push(Segment::That, 5),
            Command::Add,
            Command::Push(Segment::Argument, 1),
            Command::Sub,
            Command::Push(Segment::This, 6),
            Command::Push(Segment::This, 6),
            Command::Add,
            Command::Sub,
            Command::Push(Segment::Temp, 6),
            Command::Add,
        ],
        None,
    );

    let mut emulator = Emulator::new(&rom);
    emulator
        .ram
        .init(&[(0, 256), (1, 300), (2, 400), (3, 3000), (4, 3010)]);
    emulator.run(600);

    assert_eq!(emulator.ram.get(0), 257);
    assert_eq!(emulator.ram.get(256), 472);
    assert_eq!(emulator.ram.get(11), 510);
    assert_eq!(emulator.ram.get(256), 472);
    assert_eq!(emulator.ram.get(300), 10);
    assert_eq!(emulator.ram.get(401), 21);
    assert_eq!(emulator.ram.get(402), 22);
    assert_eq!(emulator.ram.get(3006), 36);
    assert_eq!(emulator.ram.get(3012), 42);
    assert_eq!(emulator.ram.get(3015), 45);
}

#[test]
fn pointer_test() {
    let rom = translate_and_assemble(
        &[
            Command::Push(Segment::Constant, 3030),
            Command::Pop(Segment::Pointer, 0),
            Command::Push(Segment::Constant, 3040),
            Command::Pop(Segment::Pointer, 1),
            Command::Push(Segment::Constant, 32),
            Command::Pop(Segment::This, 2),
            Command::Push(Segment::Constant, 46),
            Command::Pop(Segment::That, 6),
            Command::Push(Segment::Pointer, 0),
            Command::Push(Segment::Pointer, 1),
            Command::Add,
            Command::Push(Segment::This, 2),
            Command::Sub,
            Command::Push(Segment::That, 6),
            Command::Add,
        ],
        None,
    );

    let mut emulator = Emulator::new(&rom);
    emulator.ram.init(&[(0, 256)]);
    emulator.run(450);

    assert_eq!(emulator.ram.get(3), 3030);
    assert_eq!(emulator.ram.get(4), 3040);
    assert_eq!(emulator.ram.get(256), 6084);
    assert_eq!(emulator.ram.get(3032), 32);
    assert_eq!(emulator.ram.get(3046), 46);
}

#[test]
fn static_test() {
    let rom = translate_and_assemble(
        &[
            Command::Push(Segment::Constant, 111),
            Command::Push(Segment::Constant, 333),
            Command::Push(Segment::Constant, 888),
            Command::Pop(Segment::Static, 8),
            Command::Pop(Segment::Static, 3),
            Command::Pop(Segment::Static, 1),
            Command::Push(Segment::Static, 3),
            Command::Push(Segment::Static, 1),
            Command::Sub,
            Command::Push(Segment::Static, 8),
            Command::Add,
        ],
        None,
    );

    let mut emulator = Emulator::new(&rom);
    emulator.ram.init(&[(0, 256)]);
    emulator.run(200);

    assert_eq!(emulator.ram.get(256), 1110);
}

#[test]
fn basic_loop() {
    let rom = translate_and_assemble(
        &[
            Command::Push(Segment::Constant, 0),
            Command::Pop(Segment::Local, 0),
            Command::Label("LOOP_START".into()),
            Command::Push(Segment::Argument, 0),
            Command::Push(Segment::Local, 0),
            Command::Add,
            Command::Pop(Segment::Local, 0),
            Command::Push(Segment::Argument, 0),
            Command::Push(Segment::Constant, 1),
            Command::Sub,
            Command::Pop(Segment::Argument, 0),
            Command::Push(Segment::Argument, 0),
            Command::IfGoto("LOOP_START".into()),
            Command::Push(Segment::Local, 0),
        ],
        None,
    );

    let mut emulator = Emulator::new(&rom);
    emulator.ram.init(&[(0, 256), (1, 300), (2, 400), (400, 3)]);
    emulator.run(600);

    assert_eq!(emulator.ram.get(0), 257);
    assert_eq!(emulator.ram.get(256), 6);
}

#[test]
fn fibonacci_series() {
    let rom = translate_and_assemble(
        &[
            Command::Push(Segment::Argument, 1),
            Command::Pop(Segment::Pointer, 1),
            Command::Push(Segment::Constant, 0),
            Command::Pop(Segment::That, 0),
            Command::Push(Segment::Constant, 1),
            Command::Pop(Segment::That, 1),
            Command::Push(Segment::Argument, 0),
            Command::Push(Segment::Constant, 2),
            Command::Sub,
            Command::Pop(Segment::Argument, 0),
            Command::Label("MAIN_LOOP_START".into()),
            Command::Push(Segment::Argument, 0),
            Command::IfGoto("COMPUTE_ELEMENT".into()),
            Command::Goto("END_PROGRAM".into()),
            Command::Label("COMPUTE_ELEMENT".into()),
            Command::Push(Segment::That, 0),
            Command::Push(Segment::That, 1),
            Command::Add,
            Command::Pop(Segment::That, 2),
            Command::Push(Segment::Pointer, 1),
            Command::Push(Segment::Constant, 1),
            Command::Add,
            Command::Pop(Segment::Pointer, 1),
            Command::Push(Segment::Argument, 0),
            Command::Push(Segment::Constant, 1),
            Command::Sub,
            Command::Pop(Segment::Argument, 0),
            Command::Goto("MAIN_LOOP_START".into()),
            Command::Label("END_PROGRAM".into()),
        ],
        None,
    );

    let mut emulator = Emulator::new(&rom);
    emulator
        .ram
        .init(&[(0, 256), (1, 300), (2, 400), (400, 6), (401, 3000)]);
    emulator.run(1100);

    assert_eq!(emulator.ram.get(3000), 0);
    assert_eq!(emulator.ram.get(3001), 1);
    assert_eq!(emulator.ram.get(3002), 1);
    assert_eq!(emulator.ram.get(3003), 2);
    assert_eq!(emulator.ram.get(3004), 3);
    assert_eq!(emulator.ram.get(3005), 5);
}

#[test]
fn simple_function() {
    let rom = translate_and_assemble(
        &[
            Command::Function("SimpleFunction.test".into(), 2),
            Command::Push(Segment::Local, 0),
            Command::Push(Segment::Local, 1),
            Command::Add,
            Command::Not,
            Command::Push(Segment::Argument, 0),
            Command::Add,
            Command::Push(Segment::Argument, 1),
            Command::Sub,
            Command::Return,
        ],
        None,
    );

    let mut emulator = Emulator::new(&rom);
    emulator.ram.init(&[
        (0, 317),
        (1, 317),
        (2, 310),
        (3, 3000),
        (4, 4000),
        (310, 1234),
        (311, 37),
        (312, 1000),
        (313, 305),
        (314, 300),
        (315, 3010),
        (316, 4010),
    ]);
    emulator.run(300);

    assert_eq!(emulator.ram.get(0), 311);
    assert_eq!(emulator.ram.get(1), 305);
    assert_eq!(emulator.ram.get(2), 300);
    assert_eq!(emulator.ram.get(3), 3010);
    assert_eq!(emulator.ram.get(4), 4010);
    assert_eq!(emulator.ram.get(310), 1196);
}

#[test]
fn nested_call() {
    let rom = translate_and_assemble(
        &[
            Command::Function("Sys.init".into(), 0),
            Command::Call("Sys.main".into(), 0),
            Command::Pop(Segment::Temp, 1),
            Command::Label("LOOP".into()),
            Command::Goto("LOOP".into()),
            Command::Function("Sys.main".into(), 0),
            Command::Push(Segment::Constant, 123),
            Command::Call("Sys.add12".into(), 1),
            Command::Pop(Segment::Temp, 0),
            Command::Push(Segment::Constant, 246),
            Command::Return,
            Command::Function("Sys.add12".into(), 3),
            Command::Push(Segment::Argument, 0),
            Command::Push(Segment::Constant, 12),
            Command::Add,
            Command::Return,
        ],
        Some("Sys.init".into()),
    );

    let mut emulator = Emulator::new(&rom);
    emulator.ram.init(&[
        (0, 261),
        (1, 261),
        (2, 256),
        (3, -1i16 as u16),
        (4, -1i16 as u16),
        (256, 1234),
        (257, -1i16 as u16),
        (258, -1i16 as u16),
        (259, -1i16 as u16),
        (260, -1i16 as u16),
    ]);
    emulator.run(1000);

    assert_eq!(emulator.ram.get(0), 261);
    assert_eq!(emulator.ram.get(1), 261);
    assert_eq!(emulator.ram.get(2), 256);
    assert_eq!(emulator.ram.get(5), 135);
    assert_eq!(emulator.ram.get(6), 246);
}
