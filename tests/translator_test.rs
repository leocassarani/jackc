use jackc::hack::Emulator;
use jackc::vm::{Command, Segment, Translator};

#[test]
fn simple_add_test() {
    let translator = Translator::new(&[
        Command::Push(Segment::Constant, 7),
        Command::Push(Segment::Constant, 8),
        Command::Add,
    ]);

    let rom = translator.translate();
    let mut emulator = Emulator::new(&rom);
    emulator.ram.init(&[(0, 256)]);
    emulator.run(60);

    assert_eq!(emulator.ram.get(0), 257);
    assert_eq!(emulator.ram.get(256), 15);
}

#[test]
fn basic_test() {
    let translator = Translator::new(&[
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

    let rom = translator.translate();
    let mut emulator = Emulator::new(&rom);
    emulator
        .ram
        .init(&[(0, 256), (1, 300), (2, 400), (3, 3000), (4, 3010)]);
    emulator.run(600);

    assert_eq!(emulator.ram.get(0), 257);
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
    let translator = Translator::new(&[
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

    let rom = translator.translate();
    let mut emulator = Emulator::new(&rom);
    emulator.ram.init(&[(0, 256)]);
    emulator.run(450);

    assert_eq!(emulator.ram.get(3), 3030);
    assert_eq!(emulator.ram.get(4), 3040);
    assert_eq!(emulator.ram.get(256), 6084);
    assert_eq!(emulator.ram.get(3032), 32);
    assert_eq!(emulator.ram.get(3046), 46);
}
