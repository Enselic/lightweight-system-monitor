# lightweight-system-monitor

A lightweight ([47 kB](./build-minimal-binary.sh)) Linux system monitor.

## Usage

Typically used over ssh:

```console
$ ssh $TARGET_HOST lightweight-system-monitor | tee monitor.tsv
seconds CPU%    ΔRAM kB
1.0     0.1     1316
2.0     0.1     148
3.1     0.1     652
4.1     0.2     -376
5.1     1.3     -1048
6.1     0.2     38632
7.1     0.4     42760
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
- `ΔRAM kB`: Change in available memory (`MemAvailable` from `/proc/meminfo`) since the monitor started.

### Visualization

`monitor.tsv` can easily be visualized by tools such as _Google Sheets_, _Microsoft Excel_, or (not as easily) _gnuplot_.
