# 🌡️ Temperature Converter in Rust 🦀

**Because converting temperatures shouldn't give you a headache, even if Rust compilers do!** 🚀 🔥

Welcome to the ultimate, crash-proof, and super-smart Temperature Converter written in **Rust**. This CLI tool effortlessly converts temperatures between **Celsius (°C)**, **Fahrenheit (°F)**, and **Kelvin (°K)** with a built-in smart input validation system that prevents annoying crashes! 🧠 🛡️

---

## ✨ Features 🚀

* **12-in-1 Conversion Matrix:** Handles all 6 primary conversions (and their inverses) using a smart mid-layer Celsius logic! 📐 🧭
* **Crash-Proof Input Validation:** Robust handling of empty inputs, invalid strings, and alphabets where numbers are expected. ❌ 🙅‍♂️
* **Linux & Windows Clean UI:** Automatically clears the screen before every fresh calculation (`cls` on Windows, `clear` on Linux). 🧼 🪟
* **Graceful Interruptions:** Supports standard terminal exits. If you hit `Ctrl+D` (EOF), the program shuts down gracefully instead of panicking! 👋 💀

---

## 🛠️ Project Structure 📁

Here is how the modules are structured to keep the codebase clean and maintainable:

* `src/main.rs` — The grand orchestra / main loop. 🎻
* `src/helper.rs` — Cross-platform screen-clearing utilities. 🧹
* `src/input_validator.rs` — The robust wall defending against bad user inputs. 🧱
* `src/converter.rs` — The core mathematical engine. 🧮

---

## 🚀 How to Run

Make sure you have Rust and Cargo installed. Then, follow these steps:

1. **Clone the repository:**
   ```bash
   git clone [https://github.com/siyametj/Rust-Language.git](https://github.com/siyametj/Rust-Language.git)
   cd Rust-Language/temperature_converter
