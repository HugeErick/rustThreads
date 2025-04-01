# rustThreads

## Description
A Rust project demonstrating various threading concepts and synchronization patterns. Currently implements a producer-consumer pattern using Rust's thread-safe primitives like Mutex, Arc, and Condvar.

## Table of contents
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)
- [Contact](#contact)

## Installation

### Prerequisites
- Rust (latest stable version)
- Cargo (comes with Rust)

### Steps

1. Clone the repository
```bash
git clone --recursive https://github.com/HugeErick/rustThreads.git
cd rustThreads
```

2. Build the project
```bash
cargo build
```

## Usage

### Running the Producer-Consumer Example

1. For Linux/macOS:
```bash
cargo run --bin producerConsumer
```

2. For Windows:
```bash
cargo run --bin producerConsumer
```

The program will demonstrate a producer-consumer pattern where:
- A producer thread generates numbers (0-99)
- A consumer thread processes these numbers
- The buffer size is limited to 50 items
- The producer sleeps for 50ms between productions
- The consumer sleeps for 100ms between consumptions

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

Erick Gonzalez Parada - erick.parada101@gmail.com

Project Link: [https://github.com/HugeErick/rustThreads](https://github.com/HugeErick/rustThreads)
