//! Internal parsing crate for building an Earthmover `Body` from URDF files and serialized json

use std::{collections::HashMap, path::Path};

use earthmover_achiever::body::{Body, PeripheralKey, PeripheralNode};
use slotmap::SlotMap;

/// Loads a full body context with physical urdf
pub fn load_body<P: AsRef<Path>>(urdf_path: P) -> urdf_rs::Result<Body> {
    let robot = urdf_rs::read_file(urdf_path)?;
    let peripheral_graph: SlotMap<PeripheralKey, PeripheralNode> = SlotMap::default();
    let root = vec![];

    let mut name_link_lookup = HashMap::new();

    for link in robot.links.iter() {
        let name = link.name.clone();
        name_link_lookup.insert(name,  link);
    }

    for joint in robot.joints.iter() {
        
    }

    Body::new(Some(robot), peripheral_graph, root);

    todo!()
}
