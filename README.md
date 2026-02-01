# Bevy Fixed-Step Recorder

This project is a modular Rust application built with **Bevy 0.18**. It demonstrates a 3D scene with a deterministic, fixed-timestep recording system that allows you to capture perfectly smooth 30 FPS video regardless of your actual rendering framerate.

The codebase is structured to separate the "Engine/Recorder" logic from the "Game/Animation" content.

## 📂 Project Structure

* **`src/main.rs`**: Entry point that assembles the App.
* **`src/plugin.rs`**: The reusable **Recorder Plugin**. Handles UI, input, fixed-time calculations, and screenshot saving.
* **`src/animation.rs`**: The **Content Plugin**. Contains the scene, camera, and object animation logic.

---

## 🚀 Setup & Installation

This project is designed to run inside a **Distrobox** container to ensure a consistent environment (graphics drivers, system libraries, FFmpeg).

### 1. Initialize the Container (Host Side)
**⚠️ IMPORTANT:** You must start by creating the isolated environment using the provided script on your host machine.

```bash
./init_distrobox.sh

```

*This will create the distrobox container (e.g., Arch or Fedora based).*

### 2. Configure the Environment (Container Side)

Enter the container and run the setup script to install dependencies (Rust, Alsa, Udev, FFmpeg).

**Inside the distrobox terminal:**

```bash
./setup_inside_distrobox.sh

```

---

## 🎮 How to Run

1. Ensure you are inside the Distrobox container.
2. Run the project:

```bash
cargo run

```

### Controls

| Key | Action |
| --- | --- |
| **SPACE** | **Start/Stop Recording** |

* **Recording Mode:** When active, the simulation locks to a fixed 30 FPS timestep (`0.033s` per frame). The on-screen visual speed may change (slow motion or fast forward) depending on your PC's performance, but the saved frames will be perfectly smooth.
* **Saving:** Frames are saved as PNGs in `out/rec_<TIMESTAMP>/`.

---

## 📹 generating Video (MP4)

After recording, you will have a folder of sequential PNG images. To convert them into a video:

1. Navigate to the specific recording folder:
```bash
cd out/rec_<TIMESTAMP>

```


2. Run the processing script (ensure `process_video.sh` is in your project root):
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

This project uses **Bevy 0.18** (dev/main branch).

* **Fix:** Ensure your `Cargo.toml` has the `png` feature enabled:
```toml
[dependencies]
bevy = { version = "0.18", features = ["png"] }

```



### "Black or Transparent Screenshots"

* **Fix:** This project uses the `Screenshot::primary_window()` command with the `save_to_disk` observer, which safely handles GPU readback. Do not attempt to read `image.data` manually from the CPU.

### "FFmpeg not found"

* **Fix:** Run `./setup_inside_distrobox.sh` again to ensure `ffmpeg` is installed, or install it manually via your container's package manager (`pacman`, `dnf`, or `apt`).

```
