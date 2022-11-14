use std::collections::HashMap;
use std::path::Path;

use anyhow::{anyhow, bail};
use doome_raytracer_shader_common as sc;
use glam::{vec2, vec3, Vec2, Vec3};
use include_dir::{include_dir, Dir};
use tobj::LoadOptions;

const ASSETS: Dir = include_dir!("assets");

#[derive(Clone, Copy)]
pub struct ModelHandle(usize);

pub struct Pipeline {
    models: Vec<Vec<sc::Triangle>>,
}

impl Pipeline {
    pub fn builder() -> PipelineBuilder {
        PipelineBuilder {
            models: Vec::new(),
            textures: HashMap::new(),
        }
    }

    pub fn insert_to_geometry(
        &self,
        model_handle: ModelHandle,
        geometry: &mut sc::Geometry,
        pos: Vec3,
    ) {
        let model = &self.models[model_handle.0];
        for tri in model {
            geometry.push(tri.offset(pos));
        }
    }
}

pub struct PipelineBuilder {
    models: Vec<Vec<sc::Triangle>>,
    textures: HashMap<String, (image::RgbaImage, Vec<ModelHandle>)>,
}

impl PipelineBuilder {
    pub fn build(self) -> Pipeline {
        Pipeline {
            models: self.models,
        }
    }

    pub fn load_model(
        &mut self,
        path: impl AsRef<Path>,
        material_id: sc::MaterialId,
    ) -> anyhow::Result<ModelHandle> {
        let new_handle = ModelHandle(self.models.len());

        let path = path.as_ref();
        let model_file = ASSETS
            .get_file(path)
            .ok_or_else(|| anyhow!("Missing file {}", path.display()))?;

        let (models, materials) = tobj::load_obj_buf(
            &mut model_file.contents(),
            &LoadOptions {
                triangulate: true,
                ..LoadOptions::default()
            },
            |mat| Self::load_material(mat),
        )?;

        let materials = materials?;

        if materials.len() > 1 {
            bail!("Only one or none material per model is supported");
        }

        if models.len() != 1 {
            bail!("Expected exactly one model, got {}", models.len());
        }

        let triangles = load_mesh_triangles(&models[0].mesh, material_id);

        if let Some(material) = materials.get(0) {
            if material.diffuse_texture.is_empty() {
                bail!("Expected diffuse texture");
            }

            let tex = ASSETS.get_file(&material.diffuse_texture).ok_or_else(
                || anyhow!("Missing texture {}", material.diffuse_texture),
            )?;

            let img = image::load_from_memory(tex.contents()).unwrap();
            let img = img.to_rgba8();

            self.textures
                .entry(material.diffuse_texture.clone()) // :(
                .and_modify(|e| {
                    e.1.push(new_handle);
                })
                .or_insert((img, vec![new_handle]));
        }

        self.models.push(triangles);

        Ok(new_handle)
    }

    fn load_material(path: impl AsRef<Path>) -> tobj::MTLLoadResult {
        let path = path.as_ref();

        log::info!("Loading material from {}", path.display());

        let material_file =
            ASSETS.get_file(path).ok_or(tobj::LoadError::ReadError)?;

        let ret = tobj::load_mtl_buf(&mut material_file.contents())?;

        log::info!("Loaded material {ret:?}");

        Ok(ret)
    }
}

fn load_mesh_triangles(
    mesh: &tobj::Mesh,
    material_id: sc::MaterialId,
) -> Vec<sc::Triangle> {
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

        triangles.push(sc::Triangle::new(
            vertices[0],
            vertices[1],
            vertices[2],
            texcoords[0],
            texcoords[1],
            texcoords[2],
            material_id,
        ));
    }

    triangles
}
