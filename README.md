# 🌟 pattern-forge-rust

Rust-powered command-line utility for generating and printing classic coding challenge patterns in the terminal.

Inspired by the fundamental logic questions frequently asked in technical interviews, pattern-forge-rust is a practical way to master nested loops, control flow, and modular code organization in Rust.

## ✨ Features

- **Versatile Patterns**: Generates various geometrical patterns (Diamonds, Triangles, Cross Boxes, etc.)
- **Interactive Menu**: User-friendly menu-driven interface for pattern selection
- **Customizable Size**: Easily control the size of the pattern (1-5)
- **Modular Design**: Clean, organized code structure with separate modules for each pattern
- **Idiomatic Rust**: Built using modern Rust conventions, focusing on safety, performance, and clean code
- **Interview Prep**: A hands-on resource for practicing core programming logic

## 📦 Packages Used

This project uses only Rust standard library:

- **`std::io`**: Standard library utilities for efficient terminal input/output

No external dependencies required! The project is lightweight and focuses on core Rust concepts.

## 🚀 Getting Started

### Prerequisites

You must have Rust and Cargo installed on your system. The easiest way is via [rustup](https://rustup.rs/).

### Installation

1. **Clone the Repository:**

```bash
git clone https://github.com/ajay-develops/pattern-forge-rust.git
cd pattern-forge-rust
```

2. **Build and Run:**

```bash
cargo run
```

3. **Build for Release (Optional):**

```bash
cargo build --release
./target/release/patterns
```

## 💻 Usage

The program features an interactive menu system. Simply run the program and follow the prompts:

```bash
cargo run
```

### Pattern Selection

When you run the program, you'll see a menu:

```
Which pattern would you like to print?
1. Barfi (rotated square/diamond)
2. Triangle
3. Cross Box
Enter your choice (1-3):
```

After selecting a pattern, you'll be prompted for the size (1-5).

### Available Patterns

#### 1. Barfi (Rotated Square/Diamond)
A diamond pattern created using Manhattan distance calculations.

**Example (size 3):**
```
      *       
    * * *     
  * * * * *   
* * * * * * * 
  * * * * *   
    * * *     
      *       
```

#### 2. Triangle
An isosceles triangle pattern pointing upward.

**Example (size 3):**
```
    * 
  * * * 
* * * * * 
```

#### 3. Cross Box
An X pattern with a border around it.

**Example (size 3):**
```
* * * * * * * 
* *       * * 
*   *   *   * 
*     *     * 
*   *   *   * 
* *       * * 
* * * * * * * 
```

## 🏗️ Project Structure

```
patterns/
├── src/
│   ├── main.rs              # Main entry point with menu logic
│   └── patterns/
│       ├── mod.rs           # Module declarations
│       ├── barfi.rs         # Diamond pattern implementation
│       ├── triangle.rs      # Triangle pattern implementation
│       └── cross_box.rs     # Cross box pattern implementation
├── Cargo.toml               # Project dependencies and metadata
└── README.md                # This file
```

## 🤝 Contributing

This project is open source, and contributions are highly welcome! Whether you are a Rust beginner or a seasoned pro, your help is valuable.

### How to Contribute

1. Fork the repository
2. Clone your fork
3. Create a new branch for your feature or fix (e.g., `feat/add-new-pattern`)
4. Implement a new pattern or refactor an existing one, placing the logic in `src/patterns/`
5. Write tests for your new functionality (if applicable)
6. Commit your changes and push to your fork
7. Open a Pull Request (PR) against the main branch of this repository

### Ideas for Contribution

- Implement new patterns (e.g., Spiral Matrix, Number Pyramid, Hollow Square)
- Add color support using the `colored` crate
- Refactor input parsing using a Factory Pattern for creating pattern objects
- Improve error handling and validation
- Add unit tests for pattern generation functions
- Extend size range beyond 1-5
- Add command-line argument support using `clap` for non-interactive usage

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 👤 Author

**ajay-develops**

- GitHub: [@ajay-develops](https://github.com/ajay-develops)

---

⭐ If you find this project helpful, please consider giving it a star!

