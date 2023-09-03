# ucpu

A small utility for reporting CPU utilization.

### Synopsis:
```help
Usage: ucpu [OPTIONS]

Options:
  -i, --interval <INTERVAL>  Interval of CPU utilization reporting (format
                             "[0-9]+(ns|us|ms|[smhdwy])"). Defaults to 1s.
  -c, --per-core             Report utilization of each CPU core separately
  -h, --help                 Print help
```

### Build
```bash
cargo build --release
```
or simply
```bash
make
```

### Installation
```bash
sudo make install
```
