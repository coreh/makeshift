use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use editor::{EditorItem, EditorPlugin};
use icon::Icon;
use project::{ProjectEvent, ProjectItem, ProjectPlugin};
use tree_view::{TreeView, TreeViewBundle, TreeViewItem, TreeViewPlugin};
use uuid::Uuid;

mod editor;
mod icon;
mod project;
mod tree_view;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(ProjectPlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(TreeViewPlugin::<ProjectItem>::default())
        .add_plugin(TreeViewPlugin::<EditorItem>::default())
        .add_systems(
            Startup,
            (create_tree_view, create_sample_items, create_3d_scene),
        )
        .run()
}

fn create_sample_items(
    mut project_events: EventWriter<ProjectEvent>,
    asset_server: Res<AssetServer>,
) {
    project_events.send(ProjectEvent::CreateMaterial {
        uuid: Uuid::new_v4(),
        name: "Player Material".into(),
        parent_uuid: None,
    });

    let folder_uuid = Uuid::new_v4();
    project_events.send(ProjectEvent::CreateFolder {
        uuid: folder_uuid,
        name: "Levels".into(),
        parent_uuid: None,
    });

    let scene_uuid = Uuid::new_v4();
    project_events.send(ProjectEvent::CreateScene {
        uuid: scene_uuid,
        name: "Hub World".into(),
        parent_uuid: Some(folder_uuid),
    });

    let yet_another_folder_uuid = Uuid::new_v4();
    project_events.send(ProjectEvent::CreateFolder {
        uuid: yet_another_folder_uuid,
        name: "Level 02".into(),
        parent_uuid: Some(folder_uuid),
    });

    let other_folder_uuid = Uuid::new_v4();
    project_events.send(ProjectEvent::CreateFolder {
        uuid: other_folder_uuid,
        name: "Level 01".into(),
        parent_uuid: Some(folder_uuid),
    });

    project_events.send(ProjectEvent::CreateMaterial {
        uuid: Uuid::new_v4(),
        name: "Grass Material".into(),
        parent_uuid: Some(other_folder_uuid),
    });

    project_events.send(ProjectEvent::CreateMaterial {
        uuid: Uuid::new_v4(),
        name: "Stone Material".into(),
        parent_uuid: Some(other_folder_uuid),
    });

    project_events.send(ProjectEvent::CreateScene {
        uuid: Uuid::new_v4(),
        name: "Map".into(),
        parent_uuid: Some(other_folder_uuid),
    });

    project_events.send(ProjectEvent::CreateMesh {
        uuid: Uuid::new_v4(),
        name: "Player Model".into(),
        handle: asset_server.load("SomeModel.gltf"),
        parent_uuid: None,
    });

    project_events.send(ProjectEvent::CreateImage {
        uuid: Uuid::new_v4(),
        name: "Player Texture".into(),
        handle: asset_server.load("SomeImage.png"),
        parent_uuid: None,
    });
}

impl TreeViewItem for ProjectItem {
    fn title(&self) -> String {
        self.name.clone()
    }

    fn icon(&self) -> Icon {
        match self.data {
            project::ProjectItemData::Folder => Icon::named("Folder"),
            project::ProjectItemData::Material { .. } => Icon::named("Material"),
            project::ProjectItemData::Image { .. } => Icon::named("Image"),
            project::ProjectItemData::Mesh { .. } => Icon::named("Mesh"),
            project::ProjectItemData::Scene { .. } => Icon::named("Scene"),
        }
    }

    fn is_selected(&self) -> bool {
        false
    }

    fn is_hovered(&self) -> bool {
        false
    }
}

impl TreeViewItem for EditorItem {
    fn title(&self) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => match self.inferred_type {
                editor::EditorItemInferredType::None => "(Entity)".into(),
                editor::EditorItemInferredType::Camera => "(Camera)".into(),
                editor::EditorItemInferredType::PointLight => "(Point Light)".into(),
                editor::EditorItemInferredType::SpotLight => "(Spot Light)".into(),
                editor::EditorItemInferredType::DirectionalLight => "(Directional Light)".into(),
                editor::EditorItemInferredType::Mesh => "(Mesh)".into(),
            },
        }
    }

    fn icon(&self) -> Icon {
        match self.inferred_type {
            editor::EditorItemInferredType::None => Icon::named("Entity"),
            editor::EditorItemInferredType::Camera => Icon::named("Camera"),
            editor::EditorItemInferredType::PointLight => Icon::named("Light.Point"),
            editor::EditorItemInferredType::SpotLight => Icon::named("Light.Spot"),
            editor::EditorItemInferredType::DirectionalLight => Icon::named("Light.Directional"),
            editor::EditorItemInferredType::Mesh => Icon::named("Mesh.Entity"),
        }
    }

    fn is_selected(&self) -> bool {
        self.is_selected
    }

    fn is_hovered(&self) -> bool {
        self.is_hovered
    }
}

fn create_tree_view(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/Inter-SemiBold.ttf"),
        font_size: 14.0,
        color: Color::WHITE,
    };

    let list_color = Color::rgb(0.11, 0.11, 0.11);
    let heading_color = Color::rgb(0.11, 0.11, 0.11);
    let bg_color = Color::rgb(0.19, 0.19, 0.19);

    commands
        .spawn(NodeBundle {
            style: Style {
                gap: Size::all(Val::Px(1.0)),
                padding: UiRect {
                    right: Val::Px(1.0),
                    ..default()
                },
                flex_direction: FlexDirection::Column,
                size: Size {
                    width: Val::Px(250.0),
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor::from(bg_color),
            ..default()
        })
        .with_children(|children| {
            children
                .spawn(NodeBundle {
                    style: Style {
                        size: Size { ..default() },
                        padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                        ..default()
                    },
                    background_color: BackgroundColor::from(heading_color),
                    ..default()
                })
                .with_children(|children| {
                    children.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Project".into(),
                                style: text_style.clone(),
                            }],
                            ..default()
                        },
                        style: Style {
                            flex_shrink: 0.0,
                            ..default()
                        },
                        ..default()
                    });
                });

            children.spawn(TreeViewBundle::<ProjectItem> {
                tree_view: TreeView {
                    icon_size: icon::IconSize::Small,
                },
                style: Style {
                    size: Size {
                        height: Val::Percent(50.0),
                        ..default()
                    },
                    ..default()
                },
                background_color: BackgroundColor::from(list_color),
                ..default()
            });

            children
                .spawn(NodeBundle {
                    style: Style {
                        size: Size { ..default() },
                        padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                        ..default()
                    },
                    background_color: BackgroundColor::from(heading_color),
                    ..default()
                })
                .with_children(|children| {
                    children.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Scene".into(),
                                style: text_style.clone(),
                            }],
                            ..default()
                        },
                        style: Style {
                            flex_shrink: 0.0,
                            ..default()
                        },
                        ..default()
                    });
                });

            children.spawn(TreeViewBundle::<EditorItem> {
                tree_view: TreeView {
                    icon_size: icon::IconSize::XSmall,
                },
                style: Style {
                    flex_grow: 1.0,
                    size: Size {
                        height: Val::Percent(50.0),
                        ..default()
                    },
                    ..default()
                },
                background_color: BackgroundColor::from(list_color),
                ..default()
            });
        });
}

fn create_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let icosphere_mesh = meshes.add(
        Mesh::try_from(shape::Icosphere {
            radius: 0.9,
            subdivisions: 7,
        })
        .unwrap(),
    );

    commands
        .spawn((
            EditorItem::default(),
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
            ComputedVisibility::default(),
        ))
        .with_children(|children| {
            // Camera
            children.spawn((
                EditorItem::default(),
                Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 2.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                },
                RaycastPickCamera::default(),
            ));
        });

    commands.spawn((
        EditorItem::default(),
        Name::from("Icosphere"),
        PbrBundle {
            mesh: icosphere_mesh.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.9, 0.2, 0.3, 1.0),
                ..default()
            }),
            ..default()
        },
        PickableBundle::default(),
        RaycastPickTarget::default(),
    ));

    commands.spawn((
        EditorItem::default(),
        Name::from("Icosphere"),
        PbrBundle {
            mesh: icosphere_mesh.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.2, 0.9, 0.3, 1.0),
                ..default()
            }),
            transform: Transform::from_xyz(2.0, 0.0, 0.0),
            ..default()
        },
        PickableBundle::default(),
        RaycastPickTarget::default(),
    ));

    commands.spawn((
        EditorItem::default(),
        Name::from("Icosphere"),
        PbrBundle {
            mesh: icosphere_mesh.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.2, 0.3, 0.9, 1.0),
                ..default()
            }),
            transform: Transform::from_xyz(4.0, 0.0, 0.0),
            ..default()
        },
        PickableBundle::default(),
        RaycastPickTarget::default(),
    ));

    // Light
    commands.spawn((
        EditorItem::default(),
        PointLightBundle {
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
    ));

    // Sun
    commands.spawn((
        EditorItem::default(),
        DirectionalLightBundle {
            transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::X, -PI / 2.0)),
            ..default()
        },
    ));
}
