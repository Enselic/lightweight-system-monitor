# lightweight-system-monitor

A lightweight ([48 kB](./build-minimal-binary.sh)) Linux system monitor.

## Usage

Typically used over ssh:

```console
$ ssh $TARGET_HOST lightweight-system-monitor | tee monitor.tsv
seconds CPU%    Avail_kB
1.0     0.1     4231316
2.0     0.1     4231464
3.1     0.1     4232116
4.1     0.2     4231740
5.1     1.3     4230692
6.1     0.2     4269324
7.1     0.4     4312084
...
```

### CLI options

- `--interval-millis <ms>`: Sampling interval in milliseconds (default: `1000`).
- `--mem-available-baseline-kb <kB>`: Value subtracted from `MemAvailable` before printing `Avail_kB` (default: `0`).

### Columns
- `seconds`: Elapsed time in seconds since the monitor started.
- `CPU%`: CPU usage in percent (%) since the last sample.
- `Avail_kB`: Current `MemAvailable` from `/proc/meminfo`, minus `--mem-available-baseline-kb`.

### Visualization

`monitor.tsv` can easily be visualized by tools such as _Google Sheets_, _Microsoft Excel_, or (not as easily) _gnuplot_.
