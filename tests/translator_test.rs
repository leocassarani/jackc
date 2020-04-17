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
