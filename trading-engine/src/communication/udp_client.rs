use std::net::SocketAddr;
use std::net::UdpSocket;

pub struct UdpClient {
    socket: UdpSocket,
    client_address: SocketAddr,
}

impl UdpClient {
    pub fn new(client_address: SocketAddr) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(client_address)?;
        #[cfg(debug_assertions)]
        {
            println!(
                "A new UDP client is initialized with its address: {}",
                client_address
            );
        }
        Ok(Self {
            socket,
            client_address,
        })
    }

    pub fn send_message(&self, server_address: SocketAddr, message: &str) -> std::io::Result<()> {
        self.socket.send_to(message.as_bytes(), server_address)?;
        Ok(())
    }

    pub fn receive_response(&self) -> std::io::Result<String> {
        let mut buffer = [0; 1024];
        let (bytes_read, _) = self.socket.recv_from(&mut buffer)?;
        let response = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_udp_client() {
        // Set up the server socket
        let server_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let server_socket =
            UdpSocket::bind(server_address).expect("Failed to bind UDP server socket");

        // Spawn a separate thread to handle incoming messages and send a response
        std::thread::spawn(move || {
            let mut buffer = [0; 1024];
            let (bytes_received, origin) = server_socket
                .recv_from(&mut buffer)
                .expect("Failed to receive data");
            let received_message =
                std::str::from_utf8(&buffer[..bytes_received]).expect("Failed to read message");
            assert_eq!(received_message, "Hello, server!");

            let response_message = "Hello, client!";
            server_socket
                .send_to(response_message.as_bytes(), origin)
                .expect("Failed to send response");
        });

        // Set up the client socket
        let client_address: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let client = UdpClient::new(client_address).expect("Failed to create UDP client");

        // Send a message and receive the response
        let message = "Hello, server!";
        client
            .send_message(server_address, message)
            .expect("Failed to send message");

        let response = client
            .receive_response()
            .expect("Failed to receive response");
        println!("Received response: {}", response);

        assert_eq!(response, "Hello, client!");
    }
}
