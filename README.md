# amdgpu_top_nvitop

A [nvitop](https://github.com/XuehaiPan/nvitop)-style TUI frontend for AMD GPUs and
Strix Halo XDNA NPUs, built on top of
[`libamdgpu_top`](https://github.com/Umio-Yasuno/amdgpu_top/tree/main/crates/libamdgpu_top).

Goal: the polished process/GPU panel layout of nvitop, but for AMD hardware that
nvitop will never support — including mixed rigs (e.g. 7900 XTX + W7900) and
Strix Halo APUs with their NPU.

This is a standalone frontend crate, not a fork of `amdgpu_top`. It depends on
`libamdgpu_top` via git and renders its own ratatui UI. The intent is for the
eventual upstream story to be a `--nvitop` alternate UI in `amdgpu_top` itself,
mirroring the existing `--gui` / `--json` frontends.

## Status

Skeleton. Enumerates AMDGPU/XDNA devices and renders a nvitop-style shell.
Sampling (utilization, VRAM, sensors, fdinfo process list, NPU metrics) is being
wired up next.

## Build

Requires `libdrm` dev headers (e.g. `libdrm-dev` / Arch: `libdrm`).

```
cargo run
```
