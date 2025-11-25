# Rust Snake Game

A classic Snake game implemented in Rust, featuring terminal-based graphics and simple controls. This implementation uses the `crossterm` crate for cross-platform terminal handling.

## Features

- Classic Snake gameplay with score tracking
- Smooth terminal-based rendering
- Cross-platform support (Windows, macOS, Linux)
- Simple and intuitive controls
- Configurable game speed and board size

## Prerequisites

- Rust (latest stable version recommended)
- Cargo (Rust's package manager)
- A terminal that supports ANSI escape codes
- Docker (for containerized deployment)

## Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rust-snake-game.git
   cd rust-snake-game/rustysnake-game
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the game:
   ```bash
   cargo run --release
   ```

## How to Play

- Use arrow keys to control the snake's direction
- Eat the food (`*`) to grow longer and increase your score
- Avoid hitting the walls or yourself
- Press `q` to quit the game at any time

## Docker Support

Build and run using Docker:

```bash
docker build -t rustysnake .
docker run -it --rm rustysnake
```

## Deployment on Render

This game can be deployed to [Render](https://render.com) with minimal configuration:

1. Push your code to a GitHub/GitLab repository
2. Create a new Web Service on Render
3. Connect your repository
4. Use the provided `render.yaml` for configuration
5. Deploy

The game will be accessible via web terminal in your browser.

## Running Tests

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Contact

Your Name - [@yourtwitter](https://twitter.com/yourtwitter) - your.email@example.com

Project Link: [https://github.com/yourusername/rust-snake-game](https://github.com/yourusername/rust-snake-game)

## Acknowledgments

- [crossterm](https://crates.io/crates/crossterm) for cross-platform terminal handling
- [rand](https://crates.io/crates/rand) for random number generation
- All the Rust contributors for an amazing language!
