//! Internal parsing crate for building an Earthmover `Body` from URDF files and serialized json

use std::path::Path;

use earthmover_achiever::body::Body;

/// Loads a full body context with physical urdf and any additional metadata in a serialized file
pub fn load_body<P: AsRef<Path>>(urdf_path: P, body_metadata: P) -> urdf_rs::Result<Body> {
    let robot = urdf_rs::read_file(urdf_path)?;

    todo!()
}
