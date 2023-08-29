# cpu-utilization

A small utility for reporting CPU utilization.

### Synopsis:
```help
Usage: cpu-utilization [OPTIONS]

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

### Installation
```bash
cp target/release/cpu-utilization /usr/local/bin/
```
