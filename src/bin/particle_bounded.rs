use n_particles::n_body::simulate::Simulation;
use n_particles::n_body::vec2::Vec2;

use macroquad::prelude::*;

/// Fixed width of the egui sidebar (pixels).
const PANEL_W: f32 = 280.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "N-Particles Simulation".to_owned(),
        window_width: PANEL_W as i32 + 700, // sidebar + 700×700 sim area
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────

/// The simulation canvas rectangle (everything to the right of the panel).
fn sim_rect() -> (f64, f64, f64, f64) {
    let x = PANEL_W as f64;
    let y = 0.0;
    let w = screen_width() as f64 - x;
    let h = screen_height() as f64;
    (x, y, w, h)
}

/// Spawn `count` random particles **inside the simulation area only**.
fn spawn_particles(sim: &mut Simulation, count: usize, mass_min: f64, mass_max: f64) {
    let (sx, _sy, sw, sh) = sim_rect();
    for _ in 0..count {
        let mut p = Vec2::new(sx + fastrand::f64() * sw, fastrand::f64() * sh);
        p.mass = fastrand::f64() * (mass_max - mass_min) + mass_min;
        p.vel = [(fastrand::f64() - 0.5) * 2.0, (fastrand::f64() - 0.5) * 2.0];
        sim.bodies.push(p);
    }
}

/// Create a fixed central body (sun) at the centre of the **simulation area**.
fn spawn_sun(sim: &mut Simulation, sun_mass: f64) {
    let (sx, _sy, sw, sh) = sim_rect();
    let mut sun = Vec2::new(sx + sw / 2.0, sh / 2.0);
    sun.mass = sun_mass;
    sun.fixed = true;
    sim.bodies.insert(0, sun);
}

/// Keep a particle inside the simulation rectangle (not the full window).
fn keep_in_sim_bounds(p: &mut Vec2) {
    if p.fixed {
        return;
    }
    let (sx, _sy, sw, sh) = sim_rect();
    let r = p.radius();

    // Horizontal
    if p.particle[0] - r < sx {
        p.particle[0] = sx + r;
        p.vel[0] *= -0.8;
    } else if p.particle[0] + r > sx + sw {
        p.particle[0] = sx + sw - r;
        p.vel[0] *= -0.8;
    }

    // Vertical
    if p.particle[1] - r < 0.0 {
        p.particle[1] = r;
        p.vel[1] *= -0.8;
    } else if p.particle[1] + r > sh {
        p.particle[1] = sh - r;
        p.vel[1] *= -0.8;
    }
}

// ── Main ─────────────────────────────────────────────────────────────────

#[macroquad::main(window_conf)]
async fn main() {
    let bg = Color::from_rgba(18, 18, 18, 255);

    // ── Simulation state ──────────────────────────────────────────────
    let mut sim = Simulation::new();
    let initial_count: usize = 10;
    let mut spawn_count: f32 = initial_count as f32;
    let mut mass_min: f32 = 1.0;
    let mut mass_max: f32 = 21.0;
    let mut paused = false;
    let mut show_quadtree = false;
    let mut bounded = false;

    // ── Central body (sun) ────────────────────────────────────────────
    let mut has_sun = true;
    let mut sun_mass: f32 = 80.0;
    spawn_sun(&mut sim, sun_mass as f64);

    // Spawn initial orbiting particles
    spawn_particles(&mut sim, initial_count, mass_min as f64, mass_max as f64);

    // ── Energy history for the live plot ───────────────────────────────
    let mut ke_history: Vec<f64> = Vec::new();
    let max_history: usize = 600;
    let mut frame_counter: usize = 0;

    loop {
        clear_background(bg);

        // ── Physics step ──────────────────────────────────────────────
        if !paused {
            sim.gravitational_attration();
            sim.quad_tree_collision(screen_width(), screen_height());

            for p in &mut sim.bodies {
                p.update();
                if bounded {
                    keep_in_sim_bounds(p);
                }
            }
        }

        // ── Kinetic energy ────────────────────────────────────────────
        let total_ke: f64 = sim
            .bodies
            .iter()
            .map(|p| 0.5 * p.mass * (p.vel[0].powi(2) + p.vel[1].powi(2)))
            .sum();
        ke_history.push(total_ke);
        if ke_history.len() > max_history {
            ke_history.remove(0);
        }
        frame_counter += 1;

        // ── Draw a subtle border between panel and sim area ───────────
        let (sx, _sy, _sw, sh) = sim_rect();
        draw_line(
            sx as f32,
            0.0,
            sx as f32,
            sh as f32,
            1.0,
            Color::from_rgba(60, 60, 80, 255),
        );

        // ── Render particles ──────────────────────────────────────────
        for (i, p) in sim.bodies.iter().enumerate() {
            if i == 0 && has_sun {
                let r = p.radius();
                draw_circle(
                    p.x() as f32,
                    p.y() as f32,
                    r as f32,
                    Color::from_rgba(255, 200, 50, 255),
                );
            } else {
                p.draw();
            }
        }

        // ── egui sidebar ──────────────────────────────────────────────
        egui_macroquad::ui(|egui_ctx| {
            let mut visuals = egui::Visuals::dark();
            // Fully opaque panel — no bleed-through
            visuals.panel_fill = egui::Color32::from_rgb(22, 22, 32);
            visuals.window_fill = egui::Color32::from_rgb(22, 22, 32);
            egui_ctx.set_visuals(visuals);

            egui::SidePanel::left("control_panel")
                .exact_width(PANEL_W)
                .resizable(false)
                .show(egui_ctx, |ui| {
                    ui.heading("⚛  N-Particles");
                    ui.separator();

                    // ── Info ───────────────────────────────────────────
                    ui.label(format!("Bodies: {}", sim.bodies.len()));
                    ui.label(format!("FPS: {:.0}", get_fps()));
                    ui.label(format!("KE: {:.1}", total_ke));
                    ui.separator();

                    // ── Central Body (Sun) ─────────────────────────────
                    ui.collapsing("☀ Central Body", |ui| {
                        let prev_has_sun = has_sun;
                        ui.checkbox(&mut has_sun, "Enable Sun");

                        if has_sun {
                            ui.add(
                                egui::Slider::new(&mut sun_mass, 10.0..=500.0)
                                    .text("Sun Mass")
                                    .logarithmic(true),
                            );

                            // Live-update the sun
                            if !sim.bodies.is_empty() && sim.bodies[0].fixed {
                                sim.bodies[0].mass = sun_mass as f64;
                                let (sx, _sy, sw, sh) = sim_rect();
                                sim.bodies[0].particle[0] = sx + sw / 2.0;
                                sim.bodies[0].particle[1] = sh / 2.0;
                            }

                            // Sun was just toggled ON
                            if !prev_has_sun {
                                spawn_sun(&mut sim, sun_mass as f64);
                            }
                        } else if prev_has_sun {
                            if !sim.bodies.is_empty() && sim.bodies[0].fixed {
                                sim.bodies.remove(0);
                            }
                        }
                    });
                    ui.separator();

                    // ── Spawn Controls ─────────────────────────────────
                    ui.collapsing("Spawn Settings", |ui| {
                        ui.add(
                            egui::Slider::new(&mut spawn_count, 1.0..=500.0)
                                .text("Count")
                                .logarithmic(true),
                        );
                        ui.add(egui::Slider::new(&mut mass_min, 0.1..=50.0).text("Mass min"));
                        ui.add(egui::Slider::new(&mut mass_max, 1.0..=100.0).text("Mass max"));
                    });
                    ui.separator();

                    // ── Bounded Space ──────────────────────────────────
                    ui.checkbox(&mut bounded, "🔲 Bounded Space");
                    ui.separator();

                    // ── Actions ────────────────────────────────────────
                    ui.horizontal(|ui| {
                        if ui
                            .button(if paused { "▶ Resume" } else { "⏸ Pause" })
                            .clicked()
                        {
                            paused = !paused;
                        }
                        if ui.button("🔄 Reset").clicked() {
                            sim.bodies.clear();
                            ke_history.clear();
                            frame_counter = 0;
                            if has_sun {
                                spawn_sun(&mut sim, sun_mass as f64);
                            }
                            spawn_particles(
                                &mut sim,
                                spawn_count as usize,
                                mass_min as f64,
                                mass_max as f64,
                            );
                        }
                    });

                    if ui.button("➕ Add Particles").clicked() {
                        spawn_particles(
                            &mut sim,
                            spawn_count as usize,
                            mass_min as f64,
                            mass_max as f64,
                        );
                    }

                    if ui.button("💥 Clear All").clicked() {
                        sim.bodies.clear();
                        ke_history.clear();
                        frame_counter = 0;
                        if has_sun {
                            spawn_sun(&mut sim, sun_mass as f64);
                        }
                    }

                    ui.separator();
                    ui.checkbox(&mut show_quadtree, "Show QuadTree (TODO)");

                    ui.separator();

                    // ── Live Kinetic Energy Plot ───────────────────────
                    ui.label("Kinetic Energy");
                    let points: egui_plot::PlotPoints = ke_history
                        .iter()
                        .enumerate()
                        .map(|(i, &ke)| {
                            let x = (frame_counter as f64 - ke_history.len() as f64) + i as f64;
                            [x, ke]
                        })
                        .collect();

                    let line = egui_plot::Line::new("KE", points)
                        .color(egui::Color32::from_rgb(100, 200, 255));

                    egui_plot::Plot::new("ke_plot")
                        .height(150.0)
                        .show_axes(true)
                        .allow_drag(false)
                        .allow_zoom(false)
                        .allow_scroll(false)
                        .show(ui, |plot_ui| {
                            plot_ui.line(line);
                        });
                });
        });

        egui_macroquad::draw();

        next_frame().await;
    }
}
