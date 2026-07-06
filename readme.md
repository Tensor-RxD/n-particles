# N-Particles Gravitational Simulation

A high-performance N-body gravitational simulation using the **Barnes-Hut algorithm** and **QuadTree** spatial partitioning. Watch particles orbit, collide, and merge in real-time!

<img width="1859" height="1114" alt="image" src="https://github.com/user-attachments/assets/7d4908ab-e996-47e7-9790-d38a9993be89" />


## Features

- **Barnes-Hut Algorithm** - O(n log n) complexity instead of O(n²)
- **QuadTree Spatial Partitioning** - Efficient neighbor queries and force calculations
- **Realistic Physics** - Gravitational attraction with collision handling
- **Collision System** - Particles merge (momentum conserved) or bounce elastically
- **Real-time Visualization** - Smooth rendering with Macroquad
- **Configurable Parameters** - Easy tweaking of gravity, masses, and velocities
- **Scalable** - Handles 100+ particles smoothly with optimization

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Usage](#usage)
- [Architecture](#architecture)
- [Configuration](#configuration)
- [Performance](#performance)
- [Physics](#physics)
- [Troubleshooting](#troubleshooting)
- [Future Enhancements](#future-enhancements)

## Overview

This project simulates gravitational interactions between particles in 2D space. Instead of computing forces between every pair of particles (which becomes slow with many particles), it uses the **Barnes-Hut approximation**:

- Particles are organized in a **QuadTree**
- Distant groups of particles are approximated as a single point mass
- Only nearby particles are computed exactly
- Result: **14x faster** for 100 particles compared to brute force

### Key Concept: Barnes-Hut Algorithm

```
Traditional N-Body: O(n²)
├─ Calculate force between every pair
├─ 100 particles = 10,000 calculations
└─ Becomes prohibitively slow at scale

Barnes-Hut: O(n log n)
├─ Organize particles in QuadTree
├─ Group distant particles
├─ 100 particles ≈ 700 calculations
└─ 14x faster!
```

## Installation

### Prerequisites

- **Rust 1.70+** ([Install here](https://rustup.rs/))
- **Linux/macOS/Windows** - Cross-platform compatible

### Clone & Build

```bash
git clone https://github.com/arcbase/n-particles.git
cd n-particles
cargo build --release
```

## Usage

### Run the Simulation

```bash
cargo run --release
```

The window will display particles orbiting and interacting gravitationally.

### Modify Parameters

Edit `src/main.rs` to change initial conditions:

```rust
// Number of particles
for _ in 0..100 {
    // Particle mass range
    p.mass = fastrand::f64() * 5.0 + 1.0;
    
    // Initial velocity
    p.vel = [
        (fastrand::f64() - 0.5) * 2.0,
        (fastrand::f64() - 0.5) * 2.0,
    ];
    
    sim.bodies.push(p);
}
```

Edit `src/n_body/simulate.rs` for physics constants:

```rust
const G: f64 = 5.0;                    // Gravitational constant
const MIN_DISTANCE: f64 = 5.0;         // Bounce distance
const MERGE_THRESHOLD: f64 = 10.0;     // Merge distance
```

## Architecture

### Project Structure

```
src/
├── main.rs                           # Entry point
│   ├─ Particle initialization
│   ├─ Rendering loop
│   └─ Input handling
│
└── n_body/
    ├── mod.rs                        # Module declarations
    ├── vec2.rs                       # 2D Physics Vector
    │   ├─ Position (x, y)
    │   ├─ Velocity (vx, vy)
    │   ├─ Acceleration (ax, ay)
    │   └─ Mass
    │
    ├── quadtree.rs                   # Spatial Partitioning
    │   ├─ Node structure
    │   ├─ Recursive subdivision
    │   ├─ insert()
    │   ├─ calculate_force()
    │   └─ compute_mass_center()
    │
    └── simulate.rs                   # Physics Engine
        ├─ Gravity calculations
        ├─ Collision detection
        ├─ Collision resolution
        └─ update()
```

### Class Diagram

```
Vec2
├── e: [f64; 2]              (position)
├── vel: [f64; 2]            (velocity)
├── acc: [f64; 2]            (acceleration)
├── mass: f64
├── update()
├── draw()
└── dist_sq()

QuadTree
├── boundary: Node
├── capacity: usize
├── bodies: Vec<Vec2>
├── divided: bool
├── nw, ne, sw, se: Option<Box<QuadTree>>
├── insert()
├── subdivide()
├── calculate_force()
├── compute_mass_center()
└── intersects()

Simulation
├── bodies: Vec<Vec2>
├── quad_tree: Option<QuadTree>
├── gravitation_attraction()
├── handle_collisions()
└── update()
```

## Configuration

### Physics Constants

| Parameter | Default | Description |
|-----------|---------|-------------|
| `G` | 5.0 | Gravitational constant |
| `MIN_DISTANCE` | 5.0 | Distance for elastic collision |
| `MERGE_THRESHOLD` | 10.0 | Distance for particle merging |

### QuadTree Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `capacity` | 4 | Max particles per node before subdividing |
| `boundary_size` | 2000.0 | Size of root QuadTree node |
| `theta` | 0.5 | Barnes-Hut approximation threshold |

### Particle Generation

```rust
// Orbital configuration (from main.rs)
for _ in 0..100 {
    let angle = fastrand::f64() * TAU;
    let dist = fastrand::f64() * 300.0 + 100.0;
    
    let p_x = center_x + angle.cos() * dist;
    let p_y = center_y + angle.sin() * dist;
    
    p.mass = fastrand::f64() * 5.0 + 1.0;
    
    let orb_speed = (G * sun.mass / dist).sqrt();
    p.vel = [-angle.sin() * orb_speed, angle.cos() * orb_speed];
}
```

## Performance

### Complexity Analysis

| Algorithm | Complexity | N=100 | N=1000 | N=10000 |
|-----------|-----------|-------|--------|---------|
| Brute Force O(n²) | 10,000 | 1,000,000 | 100,000,000 |
| Barnes-Hut O(n log n) | ~700 | ~10,000 | ~130,000 |
| **Speedup** | 14x | 100x | 769x |

### Benchmarks

```bash
# Run with different particle counts
cargo run --release -- --particles 50    # ~60 FPS
cargo run --release -- --particles 100   # ~50 FPS
cargo run --release -- --particles 500   # ~15 FPS
```

Use `--release` build for optimal performance:

```bash
cargo run --release
```

## Physics

### Gravitational Force

The gravitational force between two particles:

```
F = G × (m₁ × m₂) / r²
```

Where:
- **G** = Gravitational constant (tuned for simulation)
- **m₁, m₂** = Particle masses
- **r** = Distance between particles
- **a = F / m** = Acceleration

### Barnes-Hut Approximation

```
if (node_size / distance) < theta:
    # Use center of mass
    F = G × (m_particle × m_node) / r²
else:
    # Recurse into children
    for each child_node:
        calculate_force(particle, child_node)
```

### Collision Physics

**Merging** (distance < MERGE_THRESHOLD):
```
Conservation of momentum:
m_new = m₁ + m₂
v_new = (m₁×v₁ + m₂×v₂) / m_new
pos_new = (m₁×pos₁ + m₂×pos₂) / m_new
```

**Elastic Collision** (MIN_DISTANCE < distance < MERGE_THRESHOLD):
```
Particles push apart:
overlap = MIN_DISTANCE - distance
push = overlap / 2 + 0.1
new_pos = old_pos ± push × unit_direction
```

### Update Equations

Each frame:

```rust
// 1. Calculate accelerations from gravity
acceleration = calculate_forces() / mass

// 2. Update velocity
velocity += acceleration × dt

// 3. Update position
position += velocity × dt
```

## Troubleshooting

### Particles Not Moving
- **Check:** `gravitation_attraction()` is being called in `update()`
- **Check:** Particles have non-zero `acc` values
- **Check:** `G` constant is not too small (try G = 5.0)

### Particles Disappear
- **Check:** Particles going out of bounds
- **Add:** `keep_in_bounds()` call in main loop
- **Or:** Increase boundary size in QuadTree

### Performance Issues
- **Use:** `cargo run --release` (not debug)
- **Reduce:** Number of particles
- **Increase:** `theta` parameter (less accurate but faster)
- **Check:** Profile with `cargo flamegraph`

### Collisions Not Working
- **Check:** `handle_collisions()` is called in `update()`
- **Check:** `MERGE_THRESHOLD > 0`
- **Check:** Particle masses are reasonable

## Future Enhancements

### Short Term
- Velocity damping for stable orbits
- Trail rendering (particle paths)
- Softening parameter (prevent singularities)
- Performance metrics display (FPS, particle count)

### Medium Term
- Interactive controls
  - Click to add particles
  - Drag to set velocity
  - Delete particles
- UI for parameter adjustment
- Pause/play simulation
- Speed control (1x, 2x, 10x)

### Long Term
- 3D simulation support
- GPU acceleration (WGPU)
- Particle systems (jets, rings)
- Save/load simulation state
- Procedural universe generation

## Resources

- [Barnes-Hut Algorithm](https://en.wikipedia.org/wiki/Barnes%E2%80%93Hut_simulation)
- [QuadTree Data Structure](https://en.wikipedia.org/wiki/Quadtree)
- [N-Body Problem](https://en.wikipedia.org/wiki/N-body_problem)
- [Macroquad](https://macroquad.rs/)

## License

MIT License - See LICENSE file for details

## Author

Created as a learning project for efficient gravitational simulations.

---

Star if you find
