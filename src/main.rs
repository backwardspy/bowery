#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
// otherwise we'll get warnings on all Res/ResMut parameters.
#![allow(clippy::needless_pass_by_value)]
use std::f32::consts::PI;

use bevy::{
    prelude::{
        shape, App, Assets, Camera3dBundle, ClearColor, Color, Commands, Component,
        DirectionalLightBundle, EulerRot, Mesh, PbrBundle, Quat, Query, Res, ResMut,
        StandardMaterial, Startup, Transform, Update, Vec3,
    },
    time::Time,
    DefaultPlugins,
};

#[derive(Component)]
struct Spin;

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.5, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 1.2, 2.8, 0.0)),
        ..Default::default()
    });

    let mesh = meshes.add(
        shape::Torus {
            radius: 1.0,
            ring_radius: 0.1,
            subdivisions_segments: 256,
            subdivisions_sides: 64,
        }
        .into(),
    );
    let mat = materials.add(Color::hsl(35.0, 0.77, 0.49).into());

    commands
        .spawn(PbrBundle {
            mesh,
            material: mat,
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_x(PI / 2.0)),
            ..Default::default()
        })
        .insert(Spin);
}

fn spin(mut spinners: Query<(&Spin, &mut Transform)>, time: Res<Time>) {
    for (_, mut transform) in &mut spinners {
        transform.rotate_y(time.delta_seconds());
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hsl(220.0, 0.23, 0.95)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_world)
        .add_systems(Update, spin)
        .run();
}
