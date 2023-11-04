#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
// otherwise we'll get warnings on all Res/ResMut parameters.
#![allow(clippy::needless_pass_by_value)]
// intentional cast truncation & sign loss is pretty common in gamedev.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use bevy::{
    asset::LoadState,
    core_pipeline::{
        experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
        Skybox,
    },
    prelude::{
        debug, App, AssetServer, Assets, Camera3dBundle, ClearColor, Color, Commands, Component,
        DirectionalLight, DirectionalLightBundle, EnvironmentMapLight, EulerRot, Handle, Image,
        Quat, Query, Res, ResMut, Resource, Startup, Transform, Update, Vec3,
    },
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
    scene::SceneBundle,
    time::Time,
    DefaultPlugins,
};

#[derive(Component)]
struct Spin;

#[derive(Component)]
struct OrbitCamera;

#[derive(Resource)]
struct Cubemaps(Vec<Handle<Image>>);

fn setup_world(mut commands: Commands, mut cubemaps: ResMut<Cubemaps>, assets: Res<AssetServer>) {
    let rang = assets.load("ring.glb#Scene0");
    let sky = assets.load("sky.png");

    cubemaps.0.push(sky.clone());

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.5, 1.5).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        TemporalAntiAliasBundle::default(),
        Skybox(sky.clone()),
        EnvironmentMapLight {
            diffuse_map: sky.clone(),
            specular_map: sky,
        },
        OrbitCamera,
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::hsl(30.0, 0.5, 0.8),
            ..Default::default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 1.2, 2.8, 0.0)),
        ..Default::default()
    });

    commands.spawn((
        SceneBundle {
            scene: rang,
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_x(0.2)),
            ..Default::default()
        },
        Spin,
    ));
}

fn setup_cubemaps(
    mut cubemaps: ResMut<Cubemaps>,
    mut images: ResMut<Assets<Image>>,
    assets: Res<AssetServer>,
) {
    cubemaps.0.retain(|handle| {
        if assets.get_load_state(handle) == LoadState::Loaded {
            debug!("cubemap texture loaded, setting up sampler...");
            let image = images
                .get_mut(handle)
                .expect("cubemap image handle is valid");

            if image.texture_descriptor.array_layer_count() == 1 {
                // the cubemap can't have only one image element, so we have to tell it to reinterpret the image as a stacked array.
                image.reinterpret_stacked_2d_as_array(image.aspect_2d() as u32);
                image.texture_view_descriptor = Some(TextureViewDescriptor {
                    dimension: Some(TextureViewDimension::Cube),
                    ..Default::default()
                });
            }

            false // loaded, drop
        } else {
            true // not loaded yet, retain
        }
    });
}

fn spin(mut spinners: Query<(&Spin, &mut Transform)>, time: Res<Time>) {
    for (_, mut transform) in &mut spinners {
        transform.rotate_y(time.delta_seconds());
    }
}

fn orbit(mut orbiters: Query<(&OrbitCamera, &mut Transform)>, time: Res<Time>) {
    for (_, mut transform) in &mut orbiters {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(-time.delta_seconds() / 15.0),
        );
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.10, 0.08, 0.06)))
        .insert_resource(Cubemaps(vec![]))
        .add_plugins(DefaultPlugins)
        .add_plugins(TemporalAntiAliasPlugin)
        .add_systems(Startup, setup_world)
        .add_systems(Update, (setup_cubemaps, spin, orbit))
        .run();
}
