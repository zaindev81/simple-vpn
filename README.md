# simple-vpn

I'll help you create a VPN implementation in Rust! This is a complex project, so I'll break it down into key components and provide you with a foundation to build upon.I've created a foundational VPN implementation in Rust for you! This implementation includes both server and client components with the following key features:

## Key Components:

1. **Encrypted Communication**: Uses AES-256-GCM encryption for all client-server communication
2. **TUN Interface**: Creates virtual network interfaces for packet routing
3. **IP Address Management**: Server assigns IP addresses from a pool (10.0.0.x range)
4. **Async Architecture**: Built with Tokio for high-performance async networking
5. **Command-Line Interface**: Easy-to-use CLI with server and client modes

## Features Included:

- **Handshake Protocol**: Secure client authentication and IP assignment
- **Packet Routing**: Basic packet forwarding between clients and TUN interface
- **Encryption**: All messages encrypted with AES-256-GCM
- **Connection Management**: Handles multiple concurrent clients
- **Logging**: Comprehensive logging with tracing

## To Use This Code:

1. **Add to Cargo.toml**:
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
tokio-tun = "0.8"
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
ring = "0.16"
base64 = "0.21"
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"
uuid = { version = "1.0", features = ["v4"] }
```

2. **Run the server**:

```bash
cargo run -- server --bind 0.0.0.0:8080
```

3. **Connect a client**:

```bash
cargo run -- client --server 127.0.0.1:8080
```

## Important Notes:

- **Root Privileges**: You'll need root/administrator privileges to create TUN interfaces
- **Key Management**: The current implementation uses a static key - in production, implement proper key exchange
- **Routing**: The routing logic is simplified - you'll want to add proper IP packet parsing
- **Error Handling**: Add more robust error handling for production use
- **Authentication**: Consider adding user authentication beyond the basic handshake

This gives you a solid foundation to build upon. Would you like me to explain any specific part in more detail or help you implement additional features like proper key exchange, user authentication, or more sophisticated routing?
