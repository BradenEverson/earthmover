//! Physics Informed Backend Implementation

use std::sync::Arc;

use bevy::prelude::*;

use bevy::{
    app::{App, Startup},
    asset::Assets,
    math::Vec3,
    pbr::{DirectionalLightBundle, PbrBundle, StandardMaterial},
    prelude::{Camera3dBundle, Commands, Cuboid, Mesh, ResMut, Resource, Transform},
    utils::default,
    DefaultPlugins,
};
use bevy_rapier3d::prelude::Collider;
use earthmover_achiever::goals::Rewardable;
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

use crate::sim::{ArcSimArgs, SimArgs, SimMessage};

use super::Simulation;

/// A Bevy Resource for the Mpsc Channel
#[derive(Resource)]
pub struct MessageChannel(pub UnboundedSender<SimMessage>);

/// All data held in a single training context. Including mappings from 3 space to peripheral
/// readings
#[derive(Default, Resource)]
pub struct TrainContext<const DIMS: usize> {
    /// 3-space points to peripheral readings at that point
    pub points_to_peripherals: HashMap<(f32, f32, f32), [f32; DIMS]>,
}

/// A Physics Informed Backend Runner
pub struct BevyPhysicsInformedBackend;

impl Simulation for BevyPhysicsInformedBackend {
    fn name(&self) -> String {
        "Bevy-Based Physics Backend".into()
    }

    fn simulate<REWARD: Rewardable, const DIMS: usize>(
        &self,
        args: Arc<SimArgs<REWARD, DIMS>>,
        message_sender: tokio::sync::mpsc::UnboundedSender<SimMessage>,
    ) {
        App::new()
            .insert_resource(MessageChannel(message_sender))
            .insert_resource(ArcSimArgs(args))
            .insert_resource(TrainContext::<DIMS>::default())
            .add_plugins(DefaultPlugins)
            .add_systems(Startup, setup::<REWARD, DIMS>)
            .run();
    }
}

/// Sets up the bevy simulation world with respect to the points provided
#[allow(unused_attributes)]
#[allow(elided_lifetimes_in_paths)]
fn setup<REWARD: Rewardable, const DIMS: usize>(
    mut commands: Commands,
    args: Res<ArcSimArgs<REWARD, DIMS>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(Cuboid::new(0.01, 0.01, 0.01)));

    let entities: Vec<_> = args
        .0
        .data
        .iter()
        .map(|point| {
            let point_in_3_space = Vec3::new(point[0], point[1], point[2]);
            let last = point.len() - 1;
            let r = point[last - 2];
            let g = point[last - 1];
            let b = point[last];

            (
                PbrBundle {
                    mesh: mesh.clone(),
                    material: materials.add(Color::srgb(r, g, b)),
                    transform: Transform::from_translation(point_in_3_space),
                    ..default()
                },
                Collider::cuboid(0.01, 0.01, 0.01),
            )
        })
        .collect();

    commands.spawn_batch(entities);

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, -2.0, 1.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}
