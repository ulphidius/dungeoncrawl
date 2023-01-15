use crate::prelude::*;
use std::{collections::HashSet, fs::File};
use ron::de::from_reader;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Template {
    pub entity_type: EntityType,
    pub name: String,
    pub glyph: char,
    pub levels: HashSet<usize>,
    /// Spawn frequency
    pub frequency: i32,
    /// Item effect
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    /// View range in tile
    pub fov: Option<i32>,
    pub base_damage: Option<i32>,
}

impl Template {
    pub fn spawn_entity(
        &self,
        point: &Point,
        command: &mut CommandBuffer,
    ) {
        let entity = command.push(
            (
                point.clone(),
                Render{
                    color: ColorPair::new(WHITE, BLACK),
                    glyph: to_cp437(self.glyph),
                },
                Name(self.name.clone()),
            )
        );

        match self.entity_type {
            EntityType::Item => command.add_component(entity, Item{}),
            EntityType::Enemy => {
                command.add_component(entity, Enemy{});
                command.add_component(entity, FieldOfView::new(self.fov.unwrap()));
                command.add_component(entity, ChasingPlayer{});
                command.add_component(entity, Health{
                    current: self.hp.unwrap(),
                    max: self.hp.unwrap(),
                });
            }
        };

        if let Some(effects) = &self.provides {
            effects.iter()
                .for_each(|(provides, value)| {
                    match provides.as_str() {
                        "Healing" => command.add_component(
                            entity,
                            ProvidesHealing{ amount: *value },
                        ),
                        "MagicMap" => command.add_component(
                            entity,
                            ProvidesDungeonMap{},
                        ),
                        _ => {
                            println!("Warning: undefinded effect provided {}", provides);
                        }
                    }
                });
        }

        if let Some(damage) = &self.base_damage {
            command.add_component(entity, Damage(*damage));
            if self.entity_type == EntityType::Item {
                command.add_component(entity, Weapon{});
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum EntityType {
    Item,
    Enemy,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load(path: &str) -> Self {
        let file = File::open(path)
            .expect("Failed to opening templates files");
        return from_reader(file).expect("Unable to load templates");
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        self.entities.iter()
            .filter(|entity| entity.levels.contains(&level))
            .for_each(|template| {
                for _ in 0..template.frequency {
                    available_entities.push(template);
                }
            });
        
        let mut command = CommandBuffer::new(ecs);
        spawn_points.iter()
            .for_each(|point| {
                if let Some(entity) = rng.random_slice_entry(&available_entities) {
                    entity.spawn_entity(point, &mut command);
                }
            });
        command.flush(ecs);
    }
}
