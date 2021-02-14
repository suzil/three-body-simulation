use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use ndarray::prelude::*;

struct Star {
    position: Array1<f32>,
    momentum: Array1<f32>,
    mass: f32,
}

struct Materials {
    star_material: Handle<ColorMaterial>,
}

struct StarA;
struct StarB;
struct StarC;

struct UiState {
    started: bool,
    playing: bool,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(ui.system())
        .add_resource(WindowDescriptor {
            title: "Three Celestial Bodies".to_string(),
            width: 1000.0,
            height: 800.0,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_startup_stage("spawn_stars", SystemStage::single(spawn_stars.system()))
        .add_system(star_movement.system())
        .add_system(position_translation.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("blue_star.png");
    commands
        .spawn(Camera2dBundle::default())
        .insert_resource(Materials {
            star_material: materials.add(texture_handle.into()),
        })
        .insert_resource(UiState {
            started: false,
            playing: false,
        });
}

fn ui(
    commands: &mut Commands,
    materials: Res<Materials>,
    mut ui_state: ResMut<UiState>,
    mut egui_context: ResMut<EguiContext>,
    mut star_a_query: Query<&mut Star, With<StarA>>,
    mut star_b_query: Query<&mut Star, With<StarB>>,
    mut star_c_query: Query<&mut Star, With<StarC>>,
    stars_query: Query<Entity, With<Star>>,
) {
    let ctx = &mut egui_context.ctx;
    egui::Window::new("Control Panel").show(ctx, |ui| {
        for mut star_a in star_a_query.iter_mut() {
            for mut star_b in star_b_query.iter_mut() {
                for mut star_c in star_c_query.iter_mut() {
                    ui.heading("Star A");
                    ui.add(egui::Slider::f32(&mut star_a.mass, 1.0..=100.0).text("mass"));
                    ui.heading("Star B");
                    ui.add(egui::Slider::f32(&mut star_b.mass, 1.0..=100.0).text("mass"));
                    ui.heading("Star C");
                    ui.add(egui::Slider::f32(&mut star_c.mass, 1.0..=100.0).text("mass"));

                    let mut ui_button_text = "▶ Play";
                    if ui_state.started && ui_state.playing {
                        ui_button_text = "⏸ Pause"
                    }
                    if ui.button(ui_button_text).clicked() {
                        ui_state.started = true;
                        ui_state.playing = !ui_state.playing;
                    };
                    if ui.button("Reset").clicked() {
                        for star in stars_query.iter() {
                            commands.despawn(star);
                        }
                        // TODO: Maybe could trigger a reset event and have it
                        // without duplicating code?
                        commands
                            .spawn(SpriteBundle {
                                material: materials.star_material.clone(),
                                transform: Transform::from_scale(Vec3::splat(0.3)),
                                ..Default::default()
                            })
                            .with(StarA)
                            .with(Star {
                                position: array![200.0, 0.0],
                                momentum: array![0.0, 20.0],
                                mass: 1.0,
                            })
                            .spawn(SpriteBundle {
                                material: materials.star_material.clone(),
                                transform: Transform::from_scale(Vec3::splat(0.3)),
                                ..Default::default()
                            })
                            .with(StarB)
                            .with(Star {
                                position: array![0.0, 0.0],
                                momentum: array![5.0, 0.0],
                                mass: 10.0,
                            })
                            .spawn(SpriteBundle {
                                material: materials.star_material.clone(),
                                transform: Transform::from_scale(Vec3::splat(0.3)),
                                ..Default::default()
                            })
                            .with(StarC)
                            .with(Star {
                                position: array![-200.0, 0.0],
                                momentum: array![0.0, -20.0],
                                mass: 1.0,
                            });

                        ui_state.started = false;
                        ui_state.playing = false;
                    }
                }
            }
        }
    });
}

fn spawn_stars(commands: &mut Commands, materials: Res<Materials>) {
    commands
        .spawn(SpriteBundle {
            material: materials.star_material.clone(),
            transform: Transform::from_scale(Vec3::splat(0.3)),
            ..Default::default()
        })
        .with(StarA)
        .with(Star {
            position: array![200.0, 0.0],
            momentum: array![0.0, 20.0],
            mass: 1.0,
        })
        .spawn(SpriteBundle {
            material: materials.star_material.clone(),
            transform: Transform::from_scale(Vec3::splat(0.3)),
            ..Default::default()
        })
        .with(StarB)
        .with(Star {
            position: array![0.0, 0.0],
            momentum: array![5.0, 0.0],
            mass: 10.0,
        })
        .spawn(SpriteBundle {
            material: materials.star_material.clone(),
            transform: Transform::from_scale(Vec3::splat(0.3)),
            ..Default::default()
        })
        .with(StarC)
        .with(Star {
            position: array![-200.0, 0.0],
            momentum: array![0.0, -20.0],
            mass: 1.0,
        });
}

fn position_translation(mut query: Query<(&Star, &mut Transform)>) {
    for (star, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(star.position[0], star.position[1], 0.0);
    }
}

fn magnitude(vector: &Array1<f32>) -> f32 {
    vector.mapv(|vector| vector.powi(2)).sum().sqrt()
}

fn norm(vector: &Array1<f32>) -> Array1<f32> {
    vector / magnitude(&vector)
}

fn star_movement(
    mut star_a_query: Query<&mut Star, With<StarA>>,
    mut star_b_query: Query<&mut Star, With<StarB>>,
    mut star_c_query: Query<&mut Star, With<StarC>>,
    ui_state: Res<UiState>,
) {
    if !ui_state.playing {
        return;
    }

    for mut star_a in star_a_query.iter_mut() {
        for mut star_b in star_b_query.iter_mut() {
            for mut star_c in star_c_query.iter_mut() {
                const G: f32 = 10000.0;
                const DT: f32 = 0.1;

                let r12 = star_b.position.clone() - star_a.position.clone();
                let r13 = star_c.position.clone() - star_a.position.clone();
                let r23 = star_c.position.clone() - star_b.position.clone();

                let f12 = -G * star_a.mass * star_b.mass / magnitude(&r12).powi(2) * norm(&r12);
                let f21 = -f12.clone();
                let f13 = -G * star_a.mass * star_c.mass / magnitude(&r13).powi(2) * norm(&r13);
                let f23 = -G * star_b.mass * star_c.mass / magnitude(&r23).powi(2) * norm(&r23);

                star_a.momentum = star_a.momentum.clone() + (f21.clone() - f13.clone()) * DT;
                star_b.momentum = star_b.momentum.clone() + (f12.clone() - f23.clone()) * DT;
                star_c.momentum = star_c.momentum.clone() + (f13.clone() + f23.clone()) * DT;

                star_a.position =
                    star_a.position.clone() + star_a.momentum.clone() * DT / star_a.mass;
                star_b.position =
                    star_b.position.clone() + star_b.momentum.clone() * DT / star_b.mass;
                star_c.position =
                    star_c.position.clone() + star_c.momentum.clone() * DT / star_c.mass;
            }
        }
    }
}
