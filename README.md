# Bevy Fixed-Step Recorder (Bevy 0.18)

This project is a Rust application built with the [Bevy Game Engine](https://bevyengine.org/) (version 0.18 dev). It demonstrates a 3D scene with a deterministic, fixed-timestep recording system that allows you to capture perfectly smooth 30 FPS video regardless of your actual rendering framerate.

## 🌟 Features

* **Fixed-Timestep Recording:** Decouples simulation time from real time. Pressing record locks the physics/animation delta to exactly `1/30`th of a second.
* **Window Capture:** Uses Bevy 0.18's new Observer/Command pattern (`Screenshot::primary_window()`) to safely capture the GPU backbuffer.
* **Automated Export:** Saves frames sequentially to timestamped folders (`out/rec_<timestamp>/`).
* **Video Conversion:** Includes a helper script to stitch PNG frames into an MP4 video using FFmpeg.

---

## 🚀 Setup & Installation

This project is designed to run inside a **Distrobox** container to ensure a consistent environment (especially for graphics drivers and libraries).

### Step 1: Initialize the Container
**⚠️ IMPORTANT:** You must start by creating the isolated environment.

Run the initialization script from your host terminal:
```bash
./init_distrobox.sh

```

*This will create the distrobox container (e.g., Arch or Fedora based) with the necessary system-level configurations.*

### Step 2: Configure the Environment

Once inside the container, you must install the required dependencies (Rust, Alsa, Udev, Wayland/X11 libs, FFmpeg).

Run the setup script **inside the distrobox terminal**:

```bash
./setup_inside_distrobox.sh

```

---

## 🎮 How to Run

1. Ensure you are inside the Distrobox container.
2. Run the project using Cargo:

```bash
cargo run

```

### Controls

| Key | Action |
| --- | --- |
| **SPACE** | **Start/Stop Recording** |

* **When Recording Starts:** The simulation switches to "Fixed Step" mode. Objects may appear to move in slow motion or fast forward depending on your PC's speed, but the recorded output will be perfectly smooth.
* **When Recording Stops:** The simulation returns to real-time.

---

## 📹 Generating Video (MP4)

After recording, you will have a folder filled with PNG images (e.g., `out/rec_173843500/`). To convert these into a video:

1. Navigate to the recording folder:
```bash
cd out/rec_<TIMESTAMP>

```


2. Run the processing script (ensure you copied `process_video.sh` to your project root or the recording folder):
```bash
../../process_video.sh

```


3. Follow the prompts:
* Enter Framerate (Default: **30**)
* Delete source images? (y/N)



The result will be saved as **`output.mp4`**.

---

## 🛠 Troubleshooting

### "Unresolved import ScreenshotManager"

This project uses **Bevy 0.18** (the current `main` branch or dev release). The API for screenshots changed significantly from 0.15.

* **Fix:** Ensure your `Cargo.toml` points to the correct version and has the `png` feature enabled:
```toml
[dependencies]
bevy = { version = "0.18", features = ["png"] }

```



### "Black or Transparent Screenshots"

This usually happens if you try to read the `image.data` from the CPU before the GPU has synced.

* **Fix:** This project uses the `save_to_disk` observer, which handles the GPU readback correctly. Ensure you are not manually trying to read the texture buffer in `main.rs`.

### "FFmpeg not found"

If the video conversion script fails:

* Run `./setup_inside_distrobox.sh` again to ensure `ffmpeg` is installed.
* Or manually install it: `sudo pacman -S ffmpeg` (Arch) or `sudo dnf install ffmpeg` (Fedora).

```
