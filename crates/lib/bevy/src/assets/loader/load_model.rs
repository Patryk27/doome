use std::path::Path;

use anyhow::{bail, Context, Result};
use glam::{vec2, vec3};
use tobj::LoadOptions;

use super::{AssetsLoader, Model, ModelMaterial, ModelTriangle};
use crate::components::Color;

impl AssetsLoader {
    pub fn load_model(&mut self, name: &str, path: &Path) -> Result<()> {
        log::info!("Loading model: {}", path.display());

        let model = self.source.read_file(path)?;

        let (models, materials) = tobj::load_obj_buf(
            &mut model.as_slice(),
            &LoadOptions {
                triangulate: true,
                ..LoadOptions::default()
            },
            |mat_name| {
                self.load_material(path.parent().unwrap().join(mat_name))
            },
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
            .map(|mat| self.process_material(mat))
            .transpose()?
            .unwrap_or_default();

        self.models.push(
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
        raw_mat: &tobj::Material,
    ) -> Result<ModelMaterial> {
        let mut mat = ModelMaterial {
            color: Color {
                r: raw_mat.diffuse[0],
                g: raw_mat.diffuse[1],
                b: raw_mat.diffuse[2],
            },
            ..ModelMaterial::default()
        };

        if !raw_mat.diffuse_texture.is_empty() {
            let texture = Path::new(&raw_mat.diffuse_texture)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();

            let texture =
                self.textures.try_by_name(texture).with_context(|| {
                    format!("Model uses an unknown texture: {}", texture)
                })?;

            mat.texture = Some(texture.transmute());
        }

        Ok(mat)
    }
}
