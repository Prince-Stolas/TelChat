use std::future::Future;
use std::io;
use std::io::{Error, LineWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;
use mvsync::{MVSync, MVSyncSpecs};
use mvutils::utils::{PClamp, Recover};
use crate::tcp::ClientHandler;
use crate::ui::draw_input_box;

mod ui;
mod tcp;

fn main() -> io::Result<()> {
    let sync = MVSync::new(MVSyncSpecs {
        thread_count: 8,
        workers_per_thread: 10,
    });

    let listener = TcpListener::bind("127.0.0.1:23")?;

    let mut client_handler = Arc::new(RwLock::new(ClientHandler::new()));
    let binding = client_handler.clone();

    let (task, handle) = sync.create_async_task(move || async move {
        for mut stream in listener.incoming().filter_map(Result::ok) {
            println!("Client: {}", stream.peer_addr().unwrap());

            stream.write("welcome to the chatroom\r\n".to_string().as_bytes());
            stream.write("Please enter your name: ".to_string().as_bytes());

            let mut guard = binding.write().recover();
            guard.clients.push(stream);
        }
    });

    sync.get_queue().submit(task);

    loop {
        let mut guard = client_handler.write().recover();
        guard.update_clients();
        drop(guard);
        sleep(Duration::from_millis(10));
    }

    Ok(())
}
