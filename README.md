# lightweight-system-monitor

A Linux system monitor so lightweight even floating point math is banned.

[Compiles](./build-minimal-binary.sh) 100% safe Rust to a 33 kB binary.

## Usage

Typically used over ssh:

```console
$ ssh $TARGET_HOST lightweight-system-monitor | tee monitor.tsv
ms      CPU‰    Avail_kB
1008    4       -324
2053    1       -324
3070    2       -1112
4088    1       -2168
5102    4       -3064
6148    0       -3568
7164    7       -20308
...
```

### Columns
- `ms`: Elapsed time in milliseconds since the monitor started.
- `CPU‰`: CPU usage in parts per thousand (‰) since the last sample.
- `Avail_kB`: Change in available memory (`MemAvailable` from `/proc/meminfo`) since the monitor started.

### Visualization

`monitor.tsv` can easily be visualized by tools such as _Google Sheets_, _Microsoft Excel_, or (not as easily) _gnuplot_.
