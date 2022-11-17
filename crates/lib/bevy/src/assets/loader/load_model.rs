use std::path::Path;

use anyhow::{bail, Context, Result};
use glam::{vec2, vec3};
use tobj::LoadOptions;

use super::{AssetsLoader, Model, ModelMaterial, ModelName, ModelTriangle};

impl AssetsLoader {
    pub fn load_model(
        &mut self,
        name: ModelName,
        path: impl AsRef<Path>,
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

        let triangles = self.process_mesh(&models[0].mesh);

        let material = materials
            .get(0)
            .map(|mat| self.process_material(name, mat))
            .transpose()?
            .unwrap_or_default();

        self.models.insert(
            name,
            Model {
                triangles,
                material,
            },
        );

        Ok(())
    }

    fn process_mesh(&self, mesh: &tobj::Mesh) -> Vec<ModelTriangle> {
        assert_eq!(mesh.texcoord_indices.len(), mesh.indices.len());

        let vertex_indices = mesh.indices.chunks(3);
        let texcoord_indices = mesh.texcoord_indices.chunks(3);

        vertex_indices
            .zip(texcoord_indices)
            .map(|(vertex_indices, texcoord_indices)| {
                let vertices: Vec<_> = vertex_indices
                    .iter()
                    .copied()
                    .map(|index| {
                        let index = index as usize;

                        vec3(
                            mesh.positions[3 * index],
                            mesh.positions[3 * index + 1],
                            mesh.positions[3 * index + 2],
                        )
                    })
                    .collect();

                let uvs: Vec<_> = texcoord_indices
                    .iter()
                    .copied()
                    .map(|index| {
                        let index = index as usize;

                        vec2(
                            mesh.texcoords[2 * index],
                            mesh.texcoords[2 * index + 1],
                        )
                    })
                    .collect();

                ModelTriangle {
                    vertices: [vertices[0], vertices[1], vertices[2]],
                    uvs: [uvs[0], uvs[1], uvs[2]],
                }
            })
            .collect()
    }

    fn process_material(
        &mut self,
        name: ModelName,
        mat: &tobj::Material,
    ) -> Result<ModelMaterial> {
        let mut mm = ModelMaterial::default();

        // TODO rgb to srgb?
        mm.color = vec3(mat.diffuse[0], mat.diffuse[1], mat.diffuse[2]);

        if !mat.diffuse_texture.is_empty() {
            mm.is_textured = true;

            let tex =
                self.dir.get_file(&mat.diffuse_texture).with_context(|| {
                    format!("Texture not found: {}", mat.diffuse_texture)
                })?;

            let tex =
                image::load_from_memory(tex.contents()).with_context(|| {
                    format!("Texture is invalid: {}", mat.diffuse_texture)
                })?;

            let tex = tex.to_rgba8();

            self.textures
                .entry(mat.diffuse_texture.clone())
                .and_modify(|e| {
                    e.1.push(name);
                })
                .or_insert((tex, vec![name]));
        }

        Ok(mm)
    }
}
