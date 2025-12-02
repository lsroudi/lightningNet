# LightningNet AI Agent Instructions

## Project Overview
LightningNet is a **Rust-based peer-to-peer networking library** implementing Lightning Network-style node communication. It provides a foundation for building distributed networks with multiple node types (Entry, Middle, Exit, Relay) that establish connections, manage circuits, and relay messages.

## Architecture

### Core Components

**`src/node/`** - Main networking module:
- **`lightning_node.rs`**: Core `LightningNode` struct managing TCP connections, circuits, and peer communication. Uses `tokio` for async I/O.
- **`config.rs`**: `NodeConfig` struct defining node identity, listen address, type, connection limits, and storage paths. Includes validation logic.
- **`types.rs`**: `NodeType` enum (Entry, Middle, Exit, Relay) and `NetworkMessage` enum for P2P protocol.

**Data Flow**: `NodeConfig` → `LightningNode::new()` → `start()` binds to address → `accept_connections_background()` spawns handlers → `Circuit`/`NetworkMessage` routing

### Key Patterns

**Async Task Management**: Use `tokio::spawn()` for background tasks (connection accepting, message handling). Never block the main task—always `.await` on I/O operations.

**Connection Lifecycle**: 
```rust
// Example from main.rs
let mut node = LightningNode::new(config);
node.start().await?;           // Bind and listen
node.connect_to_node(addr).await?;  // Outbound connection
node.stop().await?;            // Cleanup
```

**State Management**: `LightningNode` maintains `HashMap<SocketAddr, mpsc::Sender<()>>` for active connections and `HashMap<u32, Circuit>` for routing circuits. Both cleared on shutdown.

## Development Workflow

### Build & Run
```bash
cargo build                    # Debug build
cargo build --release         # Optimized
cargo run                      # Execute src/main.rs demo
```

### Testing (Planned)
Project uses `tokio-test` for dev dependencies. Tests should use `#[tokio::test]` macro for async tests.

### Key Ports
- Node 1: `127.0.0.1:8080` (Entry node, starts first)
- Node 2: `127.0.0.1:8081` (Middle node, connects after 1s delay)

## Project-Specific Conventions

**Error Handling**: Use `Result<T, String>` throughout (not custom Error types yet). Error messages are descriptive: `"Failed to bind to {}: {}"`.

**Logging**: Use `println!()` with emoji prefixes (`✅`, `❌`, `📊`, `🕐`, `🎉`) for status visibility. No structured logging setup yet.

**Address Binding**: Never use `:0` for automatic port selection in production code—use fixed ports. See FIX comment in `main.rs`.

**Serialization**: Serde integration exists (`serde`, `bincode`) in types for future protocol encoding.

## Integration Points

- **Tokio Runtime**: All async I/O through `tokio::{net, sync, time}`. `#[tokio::main]` macro required for entry point.
- **Network Protocol**: `NetworkMessage` enum defines Ping/Pong/PeerAdvertisement messages (basic structure, handlers TBD).
- **Circuit Routing**: `Circuit` struct tracks route (Vec<SocketAddr>), established state, and creation time—core for multi-hop relay logic.

## Common Tasks

| Task | Where | How |
|------|-------|-----|
| Add new node type | `src/node/types.rs` | Extend `NodeType` enum |
| Implement message handler | `src/node/lightning_node.rs` | Extend `handle_connection()` method |
| Add configuration option | `src/node/config.rs` | Add field to `NodeConfig`, update `Default` and validation |
| Create circuit | `src/node/lightning_node.rs` | Insert into `self.circuits` HashMap with unique ID |
| Send message to peer | `src/node/lightning_node.rs` | Use `mpsc::Sender` from `active_connections` map |

## Dependencies Overview
- **tokio**: Async runtime (full features enabled)
- **serde/serde_json/bincode**: Serialization (ready for protocol encoding)
- **log/env_logger**: Logging infrastructure (not yet integrated)
- **anyhow/thiserror**: Error handling (imported but minimal usage)
- **rand/hex**: Utility functions (ID generation, encoding)

---
**Last Updated**: 2025-11-30  
**Key Files**: `src/node/lightning_node.rs`, `src/main.rs`, `src/node/config.rs`
