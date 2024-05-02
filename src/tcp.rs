use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::time::Duration;

pub fn start(port: i32) -> TcpStream {
    TcpStream::connect(format!("127.0.0.1: {port}")).unwrap()
}

pub struct ClientHandler {
    pub clients: Vec<TcpStream>,
    messages: HashMap<SocketAddr, String>,
    user_names: HashMap<SocketAddr, String>
}

impl ClientHandler {
    pub fn new() -> Self {
        Self {
            clients: vec![],
            messages: HashMap::new(),
            user_names: HashMap::new()
        }
    }

    pub fn update_clients(&mut self) {
        let mut sent: Vec<(SocketAddr, String)> = vec![];
        let mut to_remove: Vec<usize> = vec![];

        for (i, client) in self.clients.iter_mut().enumerate() {
            let _ = client.set_read_timeout(Some(Duration::from_millis(1)));

            let mut buf = [0u8; 255];
            //let _ = client.read(&mut buf);

            if client.read(&mut buf).unwrap_or(1) == 0usize {
                to_remove.push(i);
                client.shutdown(Shutdown::Both);
            }
            let str = String::from_utf8(buf.to_vec()).unwrap_or("".to_string());

            if str.is_empty() { continue }

            if str.contains("\n") {
                let mut split = str.split("\n");

                let mut old = self.messages.get_mut(&client.peer_addr().unwrap()).unwrap().clone();
                old.push_str(split.next().unwrap());
                self.messages.insert(client.peer_addr().unwrap(), old.clone());

                sent.push((client.peer_addr().unwrap(), old.clone()));

                self.messages.insert(client.peer_addr().unwrap(), split.next().unwrap().to_string());
            } else {
                if !self.messages.contains_key(&client.peer_addr().unwrap()) {
                    self.messages.insert(client.peer_addr().unwrap(), "".to_string());
                }
                let mut old = self.messages.get_mut(&client.peer_addr().unwrap()).unwrap().clone();
                old.push_str(str.as_str());
                self.messages.insert(client.peer_addr().unwrap(), old.clone());
            }
        }

        for idx in to_remove {
            self.clients.remove(idx);
        }

        for (addr, msg) in sent {
            self.process_message(addr, msg);
        }
    }

    pub fn process_message(&mut self, sender: SocketAddr, message: String) {
        for mut client in &self.clients {
            if client.peer_addr().unwrap().eq(&sender) { continue }
            println!("Client: {}", client.peer_addr().unwrap());
            let str = format!("{} wrote:  {message}\n\r", sender);
            let _ = client.write(str.as_bytes());
        }
    }
}