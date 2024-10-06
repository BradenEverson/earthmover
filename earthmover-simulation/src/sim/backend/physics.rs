//! Physics Informed Backend Implementation

use bevy::{
    app::{App, Startup},
    asset::Assets,
    color::Color,
    math::Vec3,
    pbr::{DirectionalLightBundle, PbrBundle, StandardMaterial},
    prelude::{Camera3dBundle, Commands, Cuboid, Mesh, ResMut, Resource, Transform},
    utils::default,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::sim::SimMessage;

use super::Simulation;

/// A Bevy Resource for the Mpsc Channel
#[derive(Resource)]
pub struct MessageChannel(pub UnboundedSender<SimMessage>);

/// A Physics Informed Backend Runner
pub struct BevyPhysicsInformedBackend;

impl Simulation for BevyPhysicsInformedBackend {
    fn name(&self) -> String {
        "Bevy-Based Physics Backend".into()
    }

    fn simulate<REWARD: earthmover_achiever::goals::Rewardable>(
        &self,
        _args: std::sync::Arc<crate::sim::SimArgs<REWARD>>,
        message_sender: tokio::sync::mpsc::UnboundedSender<crate::sim::SimMessage>,
    ) {
        App::new()
            .insert_resource(MessageChannel(message_sender))
            .add_systems(Startup, setup)
            .run();
    }
}

/// Sets up the bevy simulation world with respect to the points provided
#[allow(unused_attributes)]
#[allow(elided_lifetimes_in_paths)]
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::WHITE),
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    },));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
