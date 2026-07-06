# ⚛ N-Particles 3D Gravitational Simulation

A high-performance, interactive 3D N-body gravitational simulation. It uses a custom **3D Octree** spatial partitioning structure for optimized collision queries and features a rich, responsive interface with orbital camera controls and live telemetry plotting.

<img width="1859" height="1114" alt="image" src="https://github.com/user-attachments/assets/7d4908ab-e996-47e7-9790-d38a9993be89" />


## Features

- **Full 3D Physics Engine** - 3D position, velocity, and acceleration calculations utilizing a custom `Vec3` structure.
- **3D Octree Spatial Partitioning** - Subdivides 3D space into 8 octants recursively to optimize range queries and resolve multi-body collisions efficiently.
- **Interactive Orbiting Camera** - Mouse drag to rotate the view, scroll to zoom in/out, and toggleable slow auto-rotation.
- **Comet-like Particle Trails** - Beautiful, fading orbital trails (particle paths) indicating historical trajectories.
- **Visual Bounding Aids** - Wireframe boundary cubes, grid projection plane, and toggleable real-time 3D Octree division wireframes.
- **Dual Binaries Layout**:
  - **Standalone Showcase (`main.rs`)**: A clean, distraction-free screensaver-like orbit simulation.
  - **Control Dashboard (`n_body_problem.rs`)**: A full control room using `egui-macroquad` for real-time adjustments.
- **Live egui Dashboard controls**:
  * Live-resizable central body (Sun) toggle and mass slider.
  * Adjust count, mass bounds, and spawn parameters.
  * Enable/disable space bounding box constraints.
  * Real-time rolling chart plotting total Kinetic Energy (KE).

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Architecture](#architecture)
- [Physics & Mathematics](#physics--mathematics)
- [Configuration & Settings](#configuration--settings)
- [Troubleshooting](#troubleshooting)

---

## Installation

### Prerequisites

- **Rust 1.70+** (Install via [rustup.rs](https://rustup.rs/))
- **3D Graphics Support** (OpenGL/WebGL compatible drivers)

### Clone & Build

```bash
git clone https://github.com/arcbase/n-particles.git
cd n-particles
cargo build --release
```

---

## Usage

This project contains two distinct binary targets:

### 1. Standalone Orbit Showcase (`main`)
Runs a clean, auto-orbiting 3D planetary system around a fixed sun.
```bash
cargo run --release
```

### 2. Interactive Control Dashboard (`n_body_problem`)
Launches the full N-body laboratory with the egui sidebar panel, controls, and telemetry plots.
```bash
cargo run --release --bin n_body_problem
```

### WebAssembly Compilation
To compile the interactive dashboard to WASM:
```bash
cargo build --target wasm32-unknown-unknown --release --bin n_body_problem
```

---

## Architecture

```
src/
├── main.rs                           # Standalone 3D orbit simulation
├── bin/
│   └── n_body_problem.rs             # Egui dashboard, camera viewports, live plotting
│
└── n_body/
    ├── mod.rs                        # Module exports
    ├── vec3.rs                       # 3D vector, mass volume scaling, elastic collisions
    ├── octree.rs                     # 3D space partitioning, subdivisions, 3D wireframe render
    └── simulate.rs                   # O(N²) gravity attraction, Octree-based collision resolution
```

### Key Components

- **`Vec3`**: Holds 3D coordinates `[f64; 3]` for position, velocity, and acceleration. Radius is dynamically scaled relative to volume ($r \propto \sqrt[3]{\text{mass}}$). Implements 3D boundary bounce and elastic collision physics.
- **`Octree`**: Replaces the 2D QuadTree. Spatially segments the 500x500x500 box into octants. Queries neighbors within a bounding volume to perform collision checks in $O(N \log N)$ rather than $O(N^2)$.
- **`Simulation`**: Contains the vector of bodies and trail coordinates. Coordinates particle motions and updates physics steps.

---

## Physics & Mathematics

### 3D Gravity Attraction
For any two bodies $i$ and $j$, the gravitational force vector is:

$$\vec{F}_{ij} = G \frac{m_i m_j}{|\vec{r}_{ij}|^2} \hat{r}_{ij}$$

Where $G = 5.0$, $\vec{r}_{ij}$ is the displacement vector, and $\hat{r}_{ij}$ is the unit direction. Summed accelerations are integrated each frame using Euler-Cromer integration.

### Collision Resolution
- **Elastic Collision**: When two spherical bodies overlap ($d < r_1 + r_2$), they push away along their displacement vector to resolve overlap, and their velocities are updated based on the 3D conservation of momentum:

$$\vec{v}'_1 = \vec{v}_1 - \frac{2m_2}{m_1+m_2} \frac{\langle\vec{v}_1-\vec{v}_2, \vec{x}_1-\vec{x}_2\rangle}{|\vec{x}_1-\vec{x}_2|^2} (\vec{x}_1-\vec{x}_2)$$

---

## Configuration & Settings

| Control Panel Option | Default | Description |
|----------------------|---------|-------------|
| **Enable Sun** | `true` | Fixed central body at origin (0, 0, 0). |
| **Sun Mass** | `500.0` | Mass of the central body (logarithmic slider up to 10000.0). |
| **Spawn Count** | `3` | Number of orbiting planets generated upon reset or addition. |
| **Bounded Space** | `false` | When true, restricts particles to a 3D box boundary with reflection. |
| **Show Octree** | `false` | Toggles rendering of green wireframe bounding boxes representing Octree nodes. |
| **Show Trails** | `true` | Toggles rendering of fading cyan particle paths. |
| **Auto Rotate Camera** | `true` | Toggles camera rotation automatically around the Y-axis. |

---

## Troubleshooting

### Camera controls not responding
- Make sure your cursor is inside the simulation area (to the right of the sidebar). Interaction events are partitioned by panel boundaries to avoid interfering with egui clicks.

### Performance drops
- Turn off **Show Octree** wireframes, as rendering a highly subdivided tree recursively with many active cells uses many draw calls.
- Compile with the `--release` flag. Rust's debug mode does not optimize vector arithmetic.
