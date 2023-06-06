use bevy::{
    prelude::{Mesh, Vec3},
    render::render_resource::PrimitiveTopology,
};

/// Computes the normal vector of a face defined by three vertices.
///
/// # Arguments
///
/// * `a` - The first vertex of the face.
/// * `b` - The second vertex of the face.
/// * `c` - The third vertex of the face.
///
/// # Returns
///
/// The normal vector of the face.
fn face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let (a, b, c) = (Vec3::from(a), Vec3::from(b), Vec3::from(c));
    (b - a).cross(c - a).normalize().into()
}

// Taken from https://github.com/bevyengine/bevy/pull/3987
/// Calculates the [`Mesh::ATTRIBUTE_NORMAL`] of a mesh.
///
/// # Panics
/// Panics if [`Mesh::ATTRIBUTE_POSITION`] is not of type `float3`.
/// Panics if the mesh has any other topology than [`PrimitiveTopology::TriangleList`].
///
/// FIXME: The should handle more cases since this is called as a part of gltf
/// mesh loading where we can't really blame users for loading meshes that might
/// not conform to the limitations here!
pub fn compute_normals(mesh: &mut Mesh) {
    assert!(
        matches!(mesh.primitive_topology(), PrimitiveTopology::TriangleList),
        "`compute_normals` can only work on `TriangleList`s"
    );

    let positions = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .unwrap()
        .as_float3()
        .expect("`Mesh::ATTRIBUTE_POSITION` vertex attributes should be of type `float3`");

    match mesh.indices() {
        Some(indices) => {
            let mut count: usize = 0;
            let mut corners = [0_usize; 3];
            let mut normals = vec![[0.0f32; 3]; positions.len()];
            let mut adjacency_counts = vec![0_usize; positions.len()];

            for i in indices.iter() {
                corners[count % 3] = i;
                count += 1;
                if count % 3 == 0 {
                    let normal = face_normal(
                        positions[corners[0]],
                        positions[corners[1]],
                        positions[corners[2]],
                    );
                    for corner in corners {
                        normals[corner] = (Vec3::from(normal) + Vec3::from(normals[corner])).into();
                        adjacency_counts[corner] += 1;
                    }
                }
            }

            // average (smooth) normals for shared vertices...
            // TODO: support different methods of weighting the average
            for i in 0..normals.len() {
                let count = adjacency_counts[i];
                if count > 0 {
                    normals[i] = (Vec3::from(normals[i]) / (count as f32)).normalize().into();
                }
            }

            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        }
        None => {
            let normals: Vec<_> = positions
                .chunks_exact(3)
                .map(|p| face_normal(p[0], p[1], p[2]))
                .flat_map(|normal| [normal; 3])
                .collect();

            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        }
    }
}
