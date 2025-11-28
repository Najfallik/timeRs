# timeRs

### Ultra-Fast and lightweight Python Script Timer — Written in Rust

![GitHub Repo stars](https://img.shields.io/github/stars/Najfallik/timeRs?style=flat-square)
![GitHub License](https://img.shields.io/github/license/Najfallik/timeRs?style=flat-square)
![GitHub last commit](https://img.shields.io/github/last-commit/Najfallik/timeRs?style=flat-square)
![Rust Version](https://img.shields.io/badge/Rust-stable-orange?style=flat-square)

**timeRs** is a high-performance command-line benchmarking tool written in Rust. It measures execution time of Python scripts with extreme accuracy and minimal overhead. Thanks to multi-threading and optimized execution, you get consistent and statistically meaningful results.

---

## Features

* **Multi-threaded benchmarking**: Run multiple tests concurrently for faster results.
* **Statistical stability with reruns**: Each thread can re-execute your script multiple times.
* **Detailed timing data**: Displays total, accumulated, and average execution times.
* **Low overhead, high precision**: Rust’s native performance ensures accurate measurements.

---

## Requirements

You’ll need:

* **Rust toolchain** (install via rustup)
* **Build essentials** (e.g., Base Devel group)
* **Python 3** in your system PATH (available as python3)

---

## Installation

### 1. Install Using Cargo (Recommended)

Use Cargo to install the latest version directly from GitHub:

```bash
cargo install --git https://github.com/Najfallik/timeRs/
```

### 2. Build Manually

Clone the repository, enter the directory, and compile the project using the release profile:

```bash
git clone https://github.com/Najfallik/timeRs/
cd timeRs
cargo build --release
```

After building, run the binary from the target/release directory:

```bash
./target/release/timeRs [OPTIONS] <PYTHON_FILE_PATH>
```
or add it to your user bin:
```bash
cp ./target/release/timeRs /usr/local/bin/timeRs && chmod +x /usr/local/bin/timeRs
```
---

## Usage

The only required argument is the path to the Python file.

A basic example is running a script once on a single thread:

```bash
timeRs my_script.py
```

---

## CLI Options
```bash 
timeRs [OPTIONS] <PYTHON_FILE_PATH>
```

| Short | Long               | Description                  | Default  |
|-------| ------------------ |------------------------------|----------|
| —     | <PYTHON_FILE_PATH> | Python file to benchmark     | Required |
| -t    | --threads          | Number of concurrent threads | 1        |
| -r    | --reruns           | Number of runs per thread    | 1        |
| -h    | --help             | Show this menu and exit      | —        |
| -v    | --version          | Display version info         | —        |

---

## Examples

* Running multiple threads for concurrent benchmarking:

```bash
./timeRs -t 5 -r 1 benchmark.py
```

* Performing heavy testing by combining threads and reruns:

```bash
./timeRs --threads 4 --reruns 25 expensive_task.py
```

---

## License

This project is distributed under the MIT License. See the LICENSE file for more information.

---

## Contributing

Contributions, issues, and feature requests are welcome. Feel free to open a pull request or create an issue.

---

## Show Your Support

If you find timeRs useful, consider giving the repository a star on GitHub.

Visit [https://github.com/Najfallik/timeRs/](https://github.com/Najfallik/timeRs/)

---

Happy benchmarking!
