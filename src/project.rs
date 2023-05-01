use std::sync::{Arc, Mutex};

use bevy::ecs::entity::EntityMap;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::reflect::{ParsedPath, ReflectOwned};
use bevy::utils::HashMap;
use uuid::Uuid;

#[derive(Resource, Default)]
pub struct ProjectItemRegistry {
    pub items: HashMap<Uuid, Entity>,
}

#[derive(Component)]
pub struct ProjectItem {
    pub uuid: Uuid,
    pub name: String,
    pub data: ProjectItemData,
}

pub enum ProjectItemData {
    Folder,
    Material {
        source: Option<String>,
        handle: Handle<StandardMaterial>,
        overrides: HashMap<ParsedPath, ReflectOwned>,
    },
    Image {
        source: Option<String>,
        handle: Handle<Image>,
        overrides: HashMap<ParsedPath, ReflectOwned>,
    },
    Mesh {
        source: Option<String>,
        handle: Handle<Mesh>,
        overrides: HashMap<ParsedPath, ReflectOwned>,
    },
    Scene {
        dynamic_scene: Arc<Mutex<DynamicScene>>,
    },
}

#[derive(Component)]
pub struct EditorItem;

#[derive(Clone)]
pub enum ProjectEvent {
    CreateFolder {
        uuid: Uuid,
        name: String,
        parent_uuid: Option<Uuid>,
    },
    CreateScene {
        uuid: Uuid,
        name: String,
        parent_uuid: Option<Uuid>,
    },
    CreateMaterial {
        uuid: Uuid,
        name: String,
        parent_uuid: Option<Uuid>,
    },
    CreateMesh {
        uuid: Uuid,
        name: String,
        parent_uuid: Option<Uuid>,
        handle: Handle<Mesh>,
    },
    CreateImage {
        uuid: Uuid,
        name: String,
        parent_uuid: Option<Uuid>,
        handle: Handle<Image>,
    },
    LoadScene {
        scene_uuid: Uuid,
    },
    StoreScene {
        scene_uuid: Uuid,
    },
}

fn handle_project_events(world: &mut World) {
    let mut project_events_system_state = SystemState::<EventReader<ProjectEvent>>::new(world);
    let project_events: Vec<ProjectEvent> = project_events_system_state
        .get(world)
        .iter()
        .cloned()
        .collect();

    for event in project_events {
        match &event {
            ProjectEvent::CreateFolder {
                uuid,
                name,
                parent_uuid,
            } => {
                let parent = parent_uuid
                    .map(|uuid| world.resource::<ProjectItemRegistry>().items.get(&uuid))
                    .flatten()
                    .cloned();

                let mut entity = world.spawn(ProjectItem {
                    uuid: *uuid,
                    name: name.clone(),
                    data: ProjectItemData::Folder,
                });

                if let Some(parent) = parent {
                    entity.set_parent(parent);
                }

                let entity = entity.id();

                world
                    .resource_mut::<ProjectItemRegistry>()
                    .items
                    .insert(*uuid, entity);
            }

            ProjectEvent::CreateScene {
                uuid,
                name,
                parent_uuid,
            } => {
                let parent = parent_uuid
                    .map(|uuid| world.resource::<ProjectItemRegistry>().items.get(&uuid))
                    .flatten()
                    .cloned();

                let mut entity = world.spawn(ProjectItem {
                    uuid: *uuid,
                    name: name.clone(),
                    data: ProjectItemData::Scene {
                        dynamic_scene: Arc::new(Mutex::from(DynamicScene::default())),
                    },
                });

                if let Some(parent) = parent {
                    entity.set_parent(parent);
                }

                let entity = entity.id();

                world
                    .resource_mut::<ProjectItemRegistry>()
                    .items
                    .insert(*uuid, entity);
            }

            ProjectEvent::CreateMaterial {
                uuid,
                name,
                parent_uuid,
            } => {
                let parent = parent_uuid
                    .map(|uuid| world.resource::<ProjectItemRegistry>().items.get(&uuid))
                    .flatten()
                    .cloned();

                let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

                let material = materials.add(StandardMaterial::default());

                let mut entity = world.spawn(ProjectItem {
                    uuid: *uuid,
                    name: name.clone(),
                    data: ProjectItemData::Material {
                        source: None,
                        handle: material,
                        overrides: default(),
                    },
                });

                if let Some(parent) = parent {
                    entity.set_parent(parent);
                }

                let entity = entity.id();

                world
                    .resource_mut::<ProjectItemRegistry>()
                    .items
                    .insert(*uuid, entity);
            }

            ProjectEvent::CreateMesh {
                uuid,
                name,
                handle,
                parent_uuid,
            } => {
                let parent = parent_uuid
                    .map(|uuid| world.resource::<ProjectItemRegistry>().items.get(&uuid))
                    .flatten()
                    .cloned();

                let mut entity = world.spawn(ProjectItem {
                    uuid: *uuid,
                    name: name.clone(),
                    data: ProjectItemData::Mesh {
                        source: None,
                        handle: handle.clone(),
                        overrides: default(),
                    },
                });

                if let Some(parent) = parent {
                    entity.set_parent(parent);
                }

                let entity = entity.id();

                world
                    .resource_mut::<ProjectItemRegistry>()
                    .items
                    .insert(*uuid, entity);
            }

            ProjectEvent::CreateImage {
                uuid,
                name,
                handle,
                parent_uuid,
            } => {
                let parent = parent_uuid
                    .map(|uuid| world.resource::<ProjectItemRegistry>().items.get(&uuid))
                    .flatten()
                    .cloned();

                let mut entity = world.spawn(ProjectItem {
                    uuid: *uuid,
                    name: name.clone(),
                    data: ProjectItemData::Image {
                        source: None,
                        handle: handle.clone(),
                        overrides: default(),
                    },
                });

                if let Some(parent) = parent {
                    entity.set_parent(parent);
                }

                let entity = entity.id();

                world
                    .resource_mut::<ProjectItemRegistry>()
                    .items
                    .insert(*uuid, entity);
            }

            ProjectEvent::LoadScene { scene_uuid } => {
                let scene_entity = world
                    .resource::<ProjectItemRegistry>()
                    .items
                    .get(&scene_uuid)
                    .unwrap()
                    .clone();

                let project_item = world
                    .query::<&ProjectItem>()
                    .get(world, scene_entity)
                    .unwrap();

                let dynamic_scene = match &project_item.data {
                    ProjectItemData::Scene { dynamic_scene } => dynamic_scene.clone(),
                    _ => panic!(),
                };

                dynamic_scene
                    .lock()
                    .unwrap()
                    .write_to_world(world, &mut EntityMap::default())
                    .unwrap();
            }

            ProjectEvent::StoreScene { scene_uuid } => {
                let scene_entity = world
                    .resource::<ProjectItemRegistry>()
                    .items
                    .get(&scene_uuid)
                    .unwrap()
                    .clone();

                let mut query = world.query_filtered::<Entity, With<EditorItem>>();
                let mut dynamic_scene_builder = DynamicSceneBuilder::from_world(world);
                dynamic_scene_builder.extract_entities(query.iter(world));
                let updated_dynamic_scene = dynamic_scene_builder.build();

                let project_item = world
                    .query::<&ProjectItem>()
                    .get(world, scene_entity)
                    .unwrap();

                let dynamic_scene = match &project_item.data {
                    ProjectItemData::Scene { dynamic_scene } => dynamic_scene.clone(),
                    _ => panic!(),
                };

                let mut arc_dynamic_scene = dynamic_scene.lock().unwrap();
                *arc_dynamic_scene = updated_dynamic_scene;
            }
        }
    }
}

pub struct ProjectPlugin;

impl Plugin for ProjectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ProjectItemRegistry::default())
            .add_event::<ProjectEvent>()
            .add_systems(Update, handle_project_events);
    }
}
