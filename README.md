<div align="center">

<img src="docs/images/logo.png" alt="Synt" width="96" />

# Uptime

**How long a host has been up, and how loaded it is.**

</div>

---

An extension for [Synt](https://github.com/syntso/synt), built with the [extension API](https://github.com/syntso/extension-api).

- **One action.** `uptime.check` on a host returns its up time and load averages as JSON.
- **Read only.** It runs `uptime` and `cat /proc/loadavg` through `ssh.exec`, the only core action it may call.
- **Nothing kept.** No storage, no files, no network of its own.

```json
{"up":{"text":"3 days, 4:05","seconds":273900},"load":{"one":0.08,"five":0.03,"fifteen":0.01}}
```

## Build

```bash
git clone https://github.com/syntso/extension-uptime.git && cd extension-uptime
git config core.hooksPath .githooks
cargo test
cargo build --release --target wasm32-wasip2
```

The component is `target/wasm32-wasip2/release/uptime.wasm`.

## License

[MIT](LICENSE).
