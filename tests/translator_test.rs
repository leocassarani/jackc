use jackc::asm;
use jackc::hack::Emulator;
use jackc::vm::{Command, Segment, Translator};

#[test]
fn simple_add_test() {
    let mut translator = Translator::new(&[
        Command::Push(Segment::Constant, 7),
        Command::Push(Segment::Constant, 8),
        Command::Add,
    ]);

    let rom = asm::assemble(&translator.translate()[..]);
    let mut emulator = Emulator::new(&rom);
    emulator.ram.init(&[(0, 256)]);
    emulator.run(60);

    assert_eq!(emulator.ram.get(0), 257);
    assert_eq!(emulator.ram.get(256), 15);
}

#[test]
fn stack_test() {
    let mut translator = Translator::new(&[
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
    ]);

    let rom = asm::assemble(&translator.translate()[..]);
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
    let mut translator = Translator::new(&[
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
    ]);

    let rom = asm::assemble(&translator.translate()[..]);
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
    let mut translator = Translator::new(&[
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
    ]);

    let rom = asm::assemble(&translator.translate()[..]);
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
fn basic_loop() {
    let cmds = vec![
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
    ];

    let mut translator = Translator::new(&cmds);
    let rom = asm::assemble(&translator.translate()[..]);

    let mut emulator = Emulator::new(&rom);
    emulator.ram.init(&[(0, 256), (1, 300), (2, 400), (400, 3)]);
    emulator.run(600);

    assert_eq!(emulator.ram.get(0), 257);
    assert_eq!(emulator.ram.get(256), 6);
}
