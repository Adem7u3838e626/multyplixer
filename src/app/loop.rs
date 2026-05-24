use crate::pty;

use std::thread;

pub fn run() {
    let (mut child, reader, writer) = pty::create_shell();

    let (tx, rx) = std::sync::mpsc::channel::<String>();

    start_output_thread(reader);
    let input_thread = start_input_handler(rx, writer);

    read_user_input(tx);

    input_thread.join().unwrap();

    println!("waiting for shell to exit...");

    let status = child.wait().unwrap();

    println!("shell exited: {:?}", status);
}

fn start_output_thread(mut reader: Box<dyn std::io::Read + Send>) {
    thread::spawn(move || {
        let mut buffer = [0u8; 1024];

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => {
                    println!("PTY closed");
                    break;
                }

                Ok(n) => {
                    let output = String::from_utf8_lossy(&buffer[..n]);
                    print!("{}", output);
                }

                Err(e) => {
                    eprintln!("read error: {}", e);
                    break;
                }
            }
        }
    });
}

fn start_input_handler(
    rx: std::sync::mpsc::Receiver<String>,
    mut writer: Box<dyn std::io::Write + Send>,
) -> std::thread::JoinHandle<()> {
    thread::spawn(move || {
        for input in rx.iter() {
            if writer.write_all(input.as_bytes()).is_err() {
                eprintln!("write error");
                break;
            }
        }
    })
}

fn read_user_input(tx: std::sync::mpsc::Sender<String>) {
    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        if tx.send(input).is_err() {
            break;
        }
    }
}
