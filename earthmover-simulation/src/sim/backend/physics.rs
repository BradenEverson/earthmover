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
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::prelude::{Collider, GravityScale, Restitution, RigidBody, Velocity};
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
#[derive(Clone, Copy)]
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
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_systems(Startup, setup::<REWARD, DIMS>)
            //.add_systems(Update, update::<REWARD, DIMS>)
            .run();
    }
}

/// Sets up the bevy simulation world with respect to the points provided
#[allow(unused_attributes)]
#[allow(elided_lifetimes_in_paths)]
/// Sets up the Bevy simulation world with respect to the points provided
fn setup<REWARD: Rewardable, const DIMS: usize>(
    mut commands: Commands,
    args: Res<ArcSimArgs<REWARD, DIMS>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_handle = meshes.add(Mesh::from(Cuboid::new(0.01, 0.01, 0.01)));
    let mut color_materials = std::collections::HashMap::new();

    let points: Vec<_> = args
        .0
        .data
        .iter()
        .map(|point| {
            let (r, g, b) = (point[DIMS - 3], point[DIMS - 2], point[DIMS - 1]);

            let color_key = ((r * 10f32) as u32, (g * 10f32) as u32, (b * 10f32) as u32);

            let material_handle = color_materials
                .entry(color_key)
                .or_insert_with(|| materials.add(Color::srgb(r, g, b)))
                .clone();

            (
                PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: material_handle,
                    transform: Transform::from_translation(Vec3::new(point[0], point[1], point[2])),
                    ..default()
                },
                Collider::cuboid(0.01, 0.01, 0.01),
                RigidBody::Fixed,
            )
        })
        .collect();

    commands.spawn_batch(points);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere::new(0.1))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.0, 5.0), // Initial position
            ..default()
        })
    .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.01))
        .insert(GravityScale(1.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Velocity::linear(Vec3::new(0.0, -0.01, 0.0)));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, -5.0, 0.5).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });


    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}

#[allow(unused_attributes)]
#[allow(elided_lifetimes_in_paths)]
/// Sets up the Bevy simulation world with respect to the points provided
fn update<REWARD: Rewardable, const DIMS: usize>() {}
