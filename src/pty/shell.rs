use portable_pty::{
    CommandBuilder,
    NativePtySystem,
    PtySize,
    PtySystem,
};

use std::io::{Read, Write};

   use std::io::{Read, Write};

pub fn create_shell() -> (
    Box<dyn portable_pty::Child + Send>,
    Box<dyn Read + Send>,
    Box<dyn Write + Send>,
) { let pty_system = NativePtySystem::default();

    let pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .unwrap();

    let cmd = CommandBuilder::new("zsh");

    let child = pair.slave.spawn_command(cmd).unwrap();

    drop(pair.slave);

    let reader = pair.master.try_clone_reader().unwrap();
    let writer = pair.master.take_writer().unwrap();

    (child, reader, writer)
}
