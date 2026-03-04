# lightweight-system-monitor

A lightweight ([49 kB](./build-minimal-binary.sh)) Linux system monitor.

## Usage

Typically used over ssh:

```console
$ ssh $TARGET_HOST lightweight-system-monitor | tee monitor.tsv
seconds\tCPU%\tAvai_kB
1.008    4       -324
2.053    1       -324
3.070    2       -1112
4.088    1       -2168
5.102    4       -3064
6.148    0       -3568
7.164    7       -20308
...
```

### Adding Auxiliary Data Points

The `--polled-path` option allows you to specify a file path to be read and included as an additional column in the output. This is useful for monitoring custom metrics or values that are not covered by CPU and memory usage.

Use `--polled-title` to specify a custom title for this additional column in the header.

Example:

```console
lightweight-system-monitor --polled-path /path/to/aux_data.txt --polled-title "AuxData"
```

### Columns
- `seconds`: Elapsed time in seconds since the monitor started.
- `CPU%`: CPU usage in percent (%) since the last sample.
- `Avail_kB`: Change in available memory (`MemAvailable` from `/proc/meminfo`) since the monitor started.

### Visualization

`monitor.tsv` can easily be visualized by tools such as _Google Sheets_, _Microsoft Excel_, or (not as easily) _gnuplot_.
