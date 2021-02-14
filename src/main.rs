use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use ndarray::prelude::*;

struct Star;

struct State {
    position: Array1<f32>,
    momentum: Array1<f32>,
    mass: f32,
}
struct Materials {
    star_material: Handle<ColorMaterial>,
}

struct UiState {
    started: bool,
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
        .insert_resource(UiState { started: false });
}

fn ui(
    commands: &mut Commands,
    mut ui_state: ResMut<UiState>,
    materials: Res<Materials>,
    mut egui_context: ResMut<EguiContext>,
) {
    let ctx = &mut egui_context.ctx;
    egui::Window::new("Control Panel").show(ctx, |ui| {
        ui.set_enabled(!ui_state.started);
        if ui.button("Start").clicked() && !ui_state.started {
            commands
                .spawn(SpriteBundle {
                    material: materials.star_material.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.3)),
                    ..Default::default()
                })
                .with(Star)
                .with(State {
                    position: array![200.0, 0.0],
                    momentum: array![0.0, 20.0],
                    mass: 1.0,
                })
                .spawn(SpriteBundle {
                    material: materials.star_material.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.3)),
                    ..Default::default()
                })
                .with(Star)
                .with(State {
                    position: array![0.0, 0.0],
                    momentum: array![5.0, 0.0],
                    mass: 10.0,
                })
                .spawn(SpriteBundle {
                    material: materials.star_material.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.3)),
                    ..Default::default()
                })
                .with(Star)
                .with(State {
                    position: array![-200.0, 0.0],
                    momentum: array![0.0, -20.0],
                    mass: 1.0,
                });
            ui_state.started = true;
        };
    });
}

fn position_translation(mut query: Query<(&State, &mut Transform)>) {
    for (state, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(state.position[0], state.position[1], 0.0);
    }
}

fn magnitude(vector: &Array1<f32>) -> f32 {
    vector.mapv(|vector| vector.powi(2)).sum().sqrt()
}

fn norm(vector: &Array1<f32>) -> Array1<f32> {
    vector / magnitude(&vector)
}

fn star_movement(mut star_states: Query<&mut State, With<Star>>) {
    let mut all_states = vec![];
    for state in star_states.iter_mut() {
        all_states.push(state);
    }
    if all_states.len() != 3 {
        return;
    }

    const G: f32 = 10000.0;
    const DT: f32 = 0.1;

    let r12 = all_states[1].position.clone() - all_states[0].position.clone();
    let r13 = all_states[2].position.clone() - all_states[0].position.clone();
    let r23 = all_states[2].position.clone() - all_states[1].position.clone();

    let f12 = -G * all_states[0].mass * all_states[1].mass / magnitude(&r12).powi(2) * norm(&r12);
    let f21 = -f12.clone();
    let f13 = -G * all_states[0].mass * all_states[2].mass / magnitude(&r13).powi(2) * norm(&r13);
    let f23 = -G * all_states[1].mass * all_states[2].mass / magnitude(&r23).powi(2) * norm(&r23);

    all_states[0].momentum = all_states[0].momentum.clone() + (f21.clone() - f13.clone()) * DT;
    all_states[1].momentum = all_states[1].momentum.clone() + (f12.clone() - f23.clone()) * DT;
    all_states[2].momentum = all_states[2].momentum.clone() + (f13.clone() + f23.clone()) * DT;

    all_states[0].position =
        all_states[0].position.clone() + all_states[0].momentum.clone() * DT / all_states[0].mass;
    all_states[1].position =
        all_states[1].position.clone() + all_states[1].momentum.clone() * DT / all_states[1].mass;
    all_states[2].position =
        all_states[2].position.clone() + all_states[2].momentum.clone() * DT / all_states[2].mass;
}
