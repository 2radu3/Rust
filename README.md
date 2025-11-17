# 🦀 rust-cli-tools

A hands-on educational project reimplementing classic Unix command-line tools in **Rust** — to learn:
- File I/O & system calls (`std::fs`, `std::io`)
- Manual argument parsing (no external crates)
- Robust error handling with `Result` and `?`
- Process control, exit codes, and CLI design

---

## 🛠 Implemented Commands ✅

| Command | Features |
|--------|----------|
| `echo`   | `-n`, `--no-newline` |
| `cat`    | concatenate files, read from stdin (`-`), basic error handling |
| `stat`   | display file type, size, permissions, owners, modified date, etc... |
| `pwd`    | print working directory |
| `mkdir`  | create directories (`mkdir dir1 dir2`) |
| `rmdir`  | remove empty directories |
| `mv`     | move/rename files and directories |
| `ln`     | create hard links and symbolic links |
| `head`   | print first 10 lines of a file |
| `date`   | print current date/time |
| `env`    | print environment variables |

> 📌 All tools exit with proper status codes and print helpful errors (no `panic!` on user mistakes).

---

## 🚀 Quick Start

### Build
```bash
git clone https://github.com/2radu3/rust-cli-tools.git
cd rust-cli-tools
cargo build
