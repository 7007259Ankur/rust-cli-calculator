# ⚡ Rust CLI Calculator

A high-performance command-line calculator built in **Rust**, designed to demonstrate core Rust concepts like ownership, pattern matching, and error handling.

---

## 🚀 Features

* ✅ Perform basic arithmetic operations:

  * Addition (`add`)
  * Subtraction (`sub`)
  * Multiplication (`mul`)
  * Division (`div`)
* ✅ Fast and lightweight CLI execution
* ✅ Robust input validation
* ✅ Safe error handling (no crashes on invalid input)
* ✅ Beginner-friendly and extensible codebase

---

## 🛠️ Tech Stack

* **Language:** Rust
* **Tooling:** Cargo (Rust package manager)
* **Concepts Used:**

  * Ownership & Borrowing
  * Pattern Matching (`match`)
  * Error Handling (`Result`, `Option`)
  * CLI Argument Parsing

---

## 📦 Installation

### 1. Install Rust

```bash
rustc --version
cargo --version
```

If not installed:
👉 https://www.rust-lang.org/tools/install

---

### 2. Clone the Repository

```bash
git clone https://github.com/your-username/cli_calculator.git
cd cli_calculator
```

---

### 3. Run the Project

```bash
cargo run <operation> <num1> <num2>
```

---

## ▶️ Usage

### ➕ Addition

```bash
cargo run add 10 5
# Output: Result: 15
```

### ➖ Subtraction

```bash
cargo run sub 10 5
# Output: Result: 5
```

### ✖️ Multiplication

```bash
cargo run mul 10 5
# Output: Result: 50
```

### ➗ Division

```bash
cargo run div 10 5
# Output: Result: 2
```

---

## ⚠️ Error Handling

The application gracefully handles:

* ❌ Missing arguments
* ❌ Invalid numbers
* ❌ Division by zero
* ❌ Unsupported operations

Example:

```bash
cargo run add 10
# Output: Usage: cargo run <operation> <num1> <num2>
```

---

## 🧠 Learning Outcomes

This project was built to gain hands-on experience with:

* Rust’s ownership model
* Memory safety without garbage collection
* Writing efficient CLI tools
* Structuring small Rust applications

---

## 📈 Future Improvements

* [ ] Add support for advanced operations (power, modulo, sqrt)
* [ ] Use `clap` crate for professional CLI parsing
* [ ] Add support for expression input (e.g., `"5 + 3"`)
* [ ] Package as a global CLI tool
* [ ] Add unit tests

---

## 🤝 Contributing

Contributions are welcome! Feel free to fork this repo and submit a PR.

---

## 📄 License

This project is licensed under the MIT License.

---

## 👨‍💻 Author

**Ankur Gupta**
B.Tech CSE @ IIIT Guwahati
Backend Engineer | Rust Learner

---

⭐ If you found this useful, consider giving it a star!
