use bevy::prelude::*;
use ndarray::prelude::*;

struct Planet;

struct State {
    position: Array1<f32>,
    momentum: Array1<f32>,
    mass: f32,
}
struct Materials {
    planet_material: Handle<ColorMaterial>,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage("game_steup", SystemStage::single(spawn_planets.system()))
        .add_system(planet_movement.system())
        .add_system(position_translation.system())
        .run();
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dBundle::default())
        .insert_resource(Materials {
            planet_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        });
}

fn spawn_planets(commands: &mut Commands, materials: Res<Materials>) {
    commands
        .spawn(SpriteBundle {
            material: materials.planet_material.clone(),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .with(Planet)
        .with(State {
            position: array![100.0, -100.0],
            momentum: array![0.0, 2.0],
            mass: 1.0,
        })
        .spawn(SpriteBundle {
            material: materials.planet_material.clone(),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .with(Planet)
        .with(State {
            position: array![100.0, 100.0],
            momentum: array![2.0, 2.0],
            mass: 1.0,
        })
        .spawn(SpriteBundle {
            material: materials.planet_material.clone(),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .with(Planet)
        .with(State {
            position: array![-100.0, 0.0],
            momentum: array![-2.0, 0.0],
            mass: 1.0,
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

fn planet_movement(mut planet_states: Query<&mut State, With<Planet>>) {
    let mut all_states = vec![];
    for state in planet_states.iter_mut() {
        all_states.push(state);
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
