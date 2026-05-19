# warp-ipc-dumper

Small helper for mirroring the Warp IPC socket through `socat` while appending the hex dump to a log file.

Run it from this folder with:

```bash
sudo python3 -m ipc_dumper
```

To also write a beautified version after the dump finishes:

```bash
sudo python3 -m ipc_dumper --beautified-output ./dumps/beautified.dump
```
