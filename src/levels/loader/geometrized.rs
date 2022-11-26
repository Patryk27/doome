use super::*;

#[derive(Default)]
pub struct Map<'a> {
    features: Vec<Feature<'a>>,
}

impl<'a> Map<'a> {
    pub fn add(&mut self, item: Feature<'a>) {
        self.features.push(item);
    }

    pub fn spawn(self, lvl: &mut LevelBuilder) {
        for feature in self.features {
            feature.spawn(lvl);
        }
    }
}

impl fmt::Display for Map<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ceilings = 0;
        let mut floors = 0;
        let mut walls = 0;

        for feature in &self.features {
            match feature {
                Feature::Ceiling { .. } => ceilings += 1,
                Feature::Floor { .. } => floors += 1,
                Feature::Wall { .. } => walls += 1,
            }
        }

        write!(
            f,
            "{} ceiling(s), {} floor(s), {} wall(s)",
            ceilings, floors, walls
        )
    }
}

pub enum Feature<'a> {
    Ceiling {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        tex: &'a str,
    },

    Floor {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        tex: &'a str,
    },

    Wall {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        rot: u8,
        tex: &'a str,
    },
}

impl<'a> Feature<'a> {
    fn spawn(self, lvl: &mut LevelBuilder) {
        match self {
            Feature::Ceiling {
                x1,
                y1,
                x2,
                y2,
                tex,
            } => {
                let tex_handle = lvl.assets().load_texture(tex);

                lvl.ceiling(x1, y1, x2, y2)
                    .alter_material(|mat| mat.with_texture(tex_handle))
                    .spawn();
            }

            Feature::Floor {
                x1,
                y1,
                x2,
                y2,
                tex,
            } => {
                assert!(x1 <= x2);
                assert!(y1 <= y2);

                let tex_handle = lvl.assets().load_texture(tex);

                lvl.floor(x1, y1, x2, y2)
                    .alter_material(|mat| {
                        let mat = mat.with_texture(tex_handle);

                        match tex {
                            "floor.stone.mossy.water" => mat
                                .with_reflectivity(0.1)
                                .with_reflection_color(Color::hex(0xffffff)),

                            "floor.checkerboard" => mat
                                .with_reflectivity(0.8)
                                .with_reflection_color(Color::hex(0xffffff)),

                            _ => mat,
                        }
                    })
                    .spawn();
            }

            Feature::Wall {
                x1,
                y1,
                x2,
                y2,
                rot,
                tex,
            } => {
                assert!(x1 <= x2);
                assert!(y1 <= y2);
                assert!(rot <= 3);

                let tex_handle = lvl.assets().load_texture(tex);

                lvl.wall(x1, y1, x2, y2, rot)
                    .alter_material(|mat| {
                        let mat = mat.with_texture(tex_handle);

                        match tex {
                            "wall.marble" => mat
                                .with_reflectivity(0.8)
                                .with_reflection_color(Color::hex(0xffffff)),

                            _ => mat,
                        }
                    })
                    .spawn();
            }
        }
    }
}
