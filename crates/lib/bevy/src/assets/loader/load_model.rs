use std::path::Path;

use anyhow::{anyhow, bail, Context, Result};
use doome_raytracer as rt;
use glam::{vec2, vec3, Vec2, Vec3};
use tobj::LoadOptions;

use super::{AssetsLoader, Model, ModelName};

impl AssetsLoader {
    pub fn load_model(
        &mut self,
        handle: ModelName,
        path: impl AsRef<Path>,
        material_id: rt::MaterialId,
    ) -> Result<()> {
        let path = path.as_ref();

        log::info!("Loading model: {}", path.display());

        let model_file = self.dir.get_file(path).context("File not found")?;

        let (models, materials) = tobj::load_obj_buf(
            &mut model_file.contents(),
            &LoadOptions {
                triangulate: true,
                ..LoadOptions::default()
            },
            |path| self.load_material(path),
        )?;

        let materials = materials.context("Couldn't load materials")?;

        if materials.len() > 1 {
            bail!("Model uses multiple materials, which is not supported");
        }

        if models.len() != 1 {
            bail!("File contains multiple models ({})", models.len());
        }

        let triangles = load_mesh_triangles(&models[0].mesh, material_id);

        if let Some(material) = materials.get(0) {
            if material.diffuse_texture.is_empty() {
                bail!("Model doesn't use diffuse texture");
            }

            let tex = self.dir.get_file(&material.diffuse_texture).ok_or_else(
                || anyhow!("Texture not found: {}", material.diffuse_texture),
            )?;

            let img = image::load_from_memory(tex.contents()).unwrap();
            let img = img.to_rgba8();

            self.textures
                .entry(material.diffuse_texture.clone())
                .and_modify(|e| {
                    e.1.push(handle);
                })
                .or_insert((img, vec![handle]));
        }

        self.models.insert(handle, Model::new(triangles));

        Ok(())
    }
}

fn load_mesh_triangles(
    mesh: &tobj::Mesh,
    material_id: rt::MaterialId,
) -> Vec<(rt::Triangle, rt::TriangleMapping)> {
    let mut triangles = vec![];

    assert_eq!(mesh.texcoord_indices.len(), mesh.indices.len());

    let vertex_indices = mesh.indices.chunks(3);
    let texcoord_indices = mesh.texcoord_indices.chunks(3);

    for (vertex_indices, texcoord_indices) in
        vertex_indices.zip(texcoord_indices)
    {
        let vertices: Vec<Vec3> = vertex_indices
            .iter()
            .copied()
            .map(|index| {
                let index = index as usize;

                let vertex = vec3(
                    mesh.positions[3 * index],
                    mesh.positions[3 * index + 1],
                    mesh.positions[3 * index + 2],
                );

                vertex
            })
            .collect();

        let texcoords: Vec<Vec2> = texcoord_indices
            .iter()
            .copied()
            .map(|index| {
                let index = index as usize;

                let texcoord = vec2(
                    mesh.texcoords[2 * index],
                    mesh.texcoords[2 * index + 1],
                );

                texcoord
            })
            .collect();

        triangles.push((
            rt::Triangle::new(
                vertices[0],
                vertices[1],
                vertices[2],
                material_id,
            ),
            rt::TriangleMapping::new(texcoords[0], texcoords[1], texcoords[2]),
        ));
    }

    triangles
}
