use crate::pickable::Pickable;
use crate::prelude::*;

pub struct Picker {
    texture: &'static str,
    position: Vec2,
    scale: Vec3,
    color: Color,
    on_pickup: Option<Command>,
    infinite: bool,
}

impl Picker {
    pub fn new(texture: &'static str) -> Self {
        Self {
            texture,
            on_pickup: Default::default(),
            position: Default::default(),
            scale: Vec3::splat(0.75),
            color: Color::hex(0xffffff),
            infinite: false,
        }
    }

    pub fn flashlight() -> Self {
        Self::new("flashlight").on_pickup(Command::Give {
            what: Item::Flashlight,
        })
    }

    pub fn heart() -> Self {
        Self::new("heart").on_pickup(Command::Heal {
            entity: EntityOrPlayer::Player,
            amount: 25.0,
        })
    }

    pub fn rifle() -> Self {
        Self::new("rifle").on_pickup(Command::Give { what: Item::Rifle })
    }

    pub fn rpg() -> Self {
        Self::new("rpg").on_pickup(Command::Give {
            what: Item::RocketLauncher,
        })
    }

    pub fn key(key: Key) -> Self {
        Self::new("key")
            .with_scale(Vec3::splat(0.35))
            .with_color(key.color())
            .on_pickup(Command::Give {
                what: Item::Key(key.clone()),
            })
    }

    pub fn with_position(mut self, val: Vec2) -> Self {
        self.position = val;
        self
    }

    pub fn with_scale(mut self, val: Vec3) -> Self {
        self.scale = val;
        self
    }

    pub fn with_color(mut self, val: Color) -> Self {
        self.color = val;
        self
    }

    pub fn on_pickup(mut self, val: Command) -> Self {
        self.on_pickup = Some(val);
        self
    }

    pub fn infinite(mut self) -> Self {
        self.infinite = true;
        self
    }

    pub fn spawn(self, assets: &Assets, commands: &mut Commands) -> Entity {
        let model = assets.load_model("picker");
        let position = vec3(self.position.x, 1.0, self.position.y);
        let texture = assets.load_texture(&format!("picker.{}", self.texture));

        let mut entity = commands.spawn((
            model,
            Transform::from_translation(position).with_scale(self.scale),
            Float {
                anchor: position,
                amplitude: 0.5,
                speed: 1.75,
            },
            Billboard,
            GeometryType::Dynamic,
            Material::default()
                .with_color(self.color)
                .with_texture(texture)
                .emissive()
                .with_uv_transparency(),
            Collider::circle(1.25, 6).detector(),
        ));

        if let Some(on_pickup) = self.on_pickup {
            entity.insert(Pickable {
                on_pickup,
                infinite: self.infinite,
            });
        }

        entity.id()
    }
}
