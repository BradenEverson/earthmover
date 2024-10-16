//! Internal parsing crate for building an Earthmover `Body` from URDF files and serialized json

use std::{collections::HashMap, path::Path};

use earthmover_achiever::body::{Body, Peripheral, PeripheralKey, PeripheralNode};
use slotmap::SlotMap;

// TODO: Metadata still needed:
// * Peripheral to Pin mappings (name -> GPIO)
// * Peripheral to Peripheral type mappings

/// Loads a full body context with physical urdf
pub fn load_body<P: AsRef<Path>>(urdf_path: P) -> urdf_rs::Result<Body> {
    let robot = urdf_rs::read_file(urdf_path)?;
    let mut peripheral_graph: SlotMap<PeripheralKey, PeripheralNode> = SlotMap::default();
    let mut root = vec![];

    let mut name_link_lookup = HashMap::new();

    for link in robot.links.iter() {
        let name = link.name.clone();
        name_link_lookup.insert(name,  link);
    }

    for joint in robot.joints.iter() {
        let peripheral = default_peripheral_from_hw();
        let joint_peripheral: PeripheralNode = peripheral.into();

        if !name_link_lookup.contains_key(&joint.parent.link) {
            let id = peripheral_graph.insert(joint_peripheral);
            root.push(id)
        } else {
            
        }
    }

    Ok(Body::new(Some(robot), peripheral_graph, root))
}

fn default_peripheral_from_hw() -> Peripheral {
    todo!()
}
