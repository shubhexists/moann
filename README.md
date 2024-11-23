# Moann

A lightweight, multithreaded alternative to Mechvibes for mechanical keyboard sound simulation, written in Rust. This CLI tool provides blazing-fast performance with minimal resource usage.

## Features

- ⚡ Lightning-fast response time through multithreading
- 🦀 Written in Rust for optimal performance and safety
- 🪶 Minimal resource footprint
- 🎮 Simple CLI interface
- 🎹 Real-time keyboard sound feedback
- 💻 Cross-platform support
- 🔧 Zero configuration needed

## Installation

### From Source

Requires Rust toolchain (1.70.0 or newer):

```bash
cargo install moann
```

## Quick Start

Simply run the following command to start the keyboard sound simulation:

```bash
moann start
```

## System Requirements

- Any modern operating system (Windows, macOS, Linux)
- Audio output device
- Minimal storage space (~10MB)

## How It Works

Moann leverages Rust's powerful concurrency model to process keyboard events and play sounds with minimal latency:

1. Main thread: Handles user input and CLI interface
2. Event listener thread: Captures keyboard events using native OS APIs
3. Audio thread: Manages sound playback with minimal latency
4. Resource thread: Handles file system operations efficiently

This architecture ensures that sound playback never blocks keyboard input processing, resulting in a seamless experience.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. Make sure to run:

```bash
cargo fmt
cargo clippy
cargo test
```

## Troubleshooting

### Common Issues

1. **Sound not playing**
   - Ensure your system's audio is not muted
   - Check if the correct audio output device is selected
   - Verify that you have the required permissions

### Debug Mode

Run with debug logging enabled:

```bash
moann start --debug
```

### Building from Source

If you encounter any issues building from source, ensure you have:

1. Latest stable Rust toolchain
2. Required system dependencies:
   - Linux: `libasound2-dev`, `libxi-dev`and `libxtst-dev`
   ```
    sudo apt install libxi-dev libxtst-dev libasound2-dev
   ```
   - macOS: XCode command line tools
   - Windows: No additional dependencies

## License

MIT License - feel free to use this software for any purpose.

## Credits

Inspired by the original Mechvibes project, but rewritten from scratch in Rust with performance in mind.