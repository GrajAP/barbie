# barbie

GTK-based Linux status bar for Hyprland (Rust).

## What it does
- Creates layered status windows for different screen positions (left / center / right)
- Shows:
  - battery level and charging state (`/sys/class/power_supply/...`)
  - current time and date tooltip
  - hyprland workspace indicators
  - volume via `pamixer`
  - screen brightness (`/sys/class/backlight/intel_backlight/...`)
  - CPU and memory percentages
- Uses Linux-native sockets and async channels to keep updates responsive

## Tech stack
- Rust (Tokio, async channels, GTK, GTK-layer-shell, Hyprland API bindings)
- Unix system interfaces (`systemstat`, filesystem proc paths)

## Run / build
Project is Nix/flake-based and uses a Hyprland + GTK stack.

```bash
 # from a Linux system with Rust + Nix (or equivalent toolchain)
 cargo run
```

## Why this project is here
This app is my playground for low-level Linux desktop tooling: process-safe async updates, event-driven widgets, and Rust UI work.
