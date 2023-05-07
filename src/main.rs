use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_mod_picking::prelude::*;
use editor::{EditorItem, EditorPlugin};
use icon::Icon;
use nine_slice::{NineSlice, NineSliceBundle, NineSlicePlugin};
use project::{ProjectEvent, ProjectItem, ProjectPlugin};
use tree_view::{TreeView, TreeViewBundle, TreeViewItem, TreeViewPlugin};
use uuid::Uuid;

mod editor;
mod icon;
mod nine_slice;
mod project;
mod tree_view;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Makeshift Editor".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(ProjectPlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(TreeViewPlugin::<ProjectItem>::default())
        .add_plugin(TreeViewPlugin::<EditorItem>::default())
        .add_plugin(NineSlicePlugin)
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

fn create_tree_view(
    mut commands: Commands,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    ui_scale: Res<UiScale>,
) {
    let text_style_semibold = TextStyle {
        font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
        font_size: 14.0,
        color: Color::WHITE,
    };

    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
        font_size: 14.0,
        color: Color::WHITE,
    };

    // assume one window for time being...
    // TODO: Support window-independent scaling: https://github.com/bevyengine/bevy/issues/5621
    let logical_to_physical_factor = if let Ok(primary_window) = primary_window.get_single() {
        primary_window.resolution.scale_factor()
    } else {
        1.0
    };

    let bg_color = Color::hex("1E1E22").unwrap();

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                right: Val::Px(0.0),
                size: Size {
                    height: Val::Px(32.0),
                    ..default()
                },
                align_items: AlignItems::Stretch,
                ..default()
            },
            background_color: BackgroundColor::from(bg_color),
            ..default()
        })
        .with_children(|children| {
            children
                .spawn(NineSliceBundle {
                    nine_slice: NineSlice {
                        image: asset_server.load("nine_slices/Toolbar@2x.png"),
                        slice: UiRect {
                            top: Val::Px(8.0),
                            left: Val::Px(8.0),
                            bottom: Val::Px(8.0),
                            right: Val::Px(8.0),
                        },
                        size: Size {
                            width: Val::Px(32.0),
                            height: Val::Px(32.0),
                        },
                        ..default()
                    },
                    style: Style {
                        flex_grow: 1.0,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(4.0)),
                        gap: Size::all(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|children| {
                    children
                        .spawn(NineSliceBundle {
                            nine_slice: NineSlice {
                                image: asset_server.load("nine_slices/Button@2x.png"),
                                slice: UiRect {
                                    top: Val::Px(8.0),
                                    left: Val::Px(8.0),
                                    bottom: Val::Px(8.0),
                                    right: Val::Px(8.0),
                                },
                                size: Size {
                                    width: Val::Px(32.0),
                                    height: Val::Px(32.0),
                                },
                                ..default()
                            },
                            style: Style {
                                size: Size {
                                    height: Val::Px(24.0),
                                    ..default()
                                },
                                align_items: AlignItems::Center,
                                padding: UiRect::horizontal(Val::Px(4.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|children| {
                            children.spawn(ImageBundle {
                                image: UiImage {
                                    texture: Icon::named("Bevy").request_icon(
                                        &asset_server,
                                        ui_scale.scale * logical_to_physical_factor,
                                        icon::IconSize::Small,
                                    ),
                                    ..default()
                                },
                                style: Style {
                                    flex_shrink: 0.0,
                                    size: Size {
                                        width: Val::Px(icon::IconSize::Small.into()),
                                        height: Val::Px(icon::IconSize::Small.into()),
                                    },
                                    ..default()
                                },
                                ..default()
                            });
                        });

                    children.spawn(NodeBundle {
                        style: Style {
                            flex_grow: 1.0,
                            ..default()
                        },
                        ..default()
                    });

                    children
                        .spawn(NineSliceBundle {
                            nine_slice: NineSlice {
                                image: asset_server.load("nine_slices/Button@2x.png"),
                                slice: UiRect {
                                    top: Val::Px(8.0),
                                    left: Val::Px(8.0),
                                    bottom: Val::Px(8.0),
                                    right: Val::Px(8.0),
                                },
                                size: Size {
                                    width: Val::Px(32.0),
                                    height: Val::Px(32.0),
                                },
                                ..default()
                            },
                            style: Style {
                                size: Size {
                                    height: Val::Px(24.0),
                                    width: Val::Px(32.0),
                                },
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::horizontal(Val::Px(4.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|children| {
                            children.spawn(ImageBundle {
                                image: UiImage {
                                    texture: Icon::named("Play").request_icon(
                                        &asset_server,
                                        ui_scale.scale * logical_to_physical_factor,
                                        icon::IconSize::XSmall,
                                    ),
                                    ..default()
                                },
                                style: Style {
                                    flex_shrink: 0.0,
                                    size: Size {
                                        width: Val::Px(icon::IconSize::XSmall.into()),
                                        height: Val::Px(icon::IconSize::XSmall.into()),
                                    },
                                    ..default()
                                },
                                ..default()
                            });
                        });

                    children
                        .spawn(NineSliceBundle {
                            nine_slice: NineSlice {
                                image: asset_server.load("nine_slices/Button@2x.png"),
                                slice: UiRect {
                                    top: Val::Px(8.0),
                                    left: Val::Px(8.0),
                                    bottom: Val::Px(8.0),
                                    right: Val::Px(8.0),
                                },
                                size: Size {
                                    width: Val::Px(32.0),
                                    height: Val::Px(32.0),
                                },
                                ..default()
                            },
                            style: Style {
                                size: Size {
                                    height: Val::Px(24.0),
                                    width: Val::Px(32.0),
                                },
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::horizontal(Val::Px(4.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|children| {
                            children.spawn(ImageBundle {
                                image: UiImage {
                                    texture: Icon::named("Pause").request_icon(
                                        &asset_server,
                                        ui_scale.scale * logical_to_physical_factor,
                                        icon::IconSize::XSmall,
                                    ),
                                    ..default()
                                },
                                style: Style {
                                    flex_shrink: 0.0,
                                    size: Size {
                                        width: Val::Px(icon::IconSize::XSmall.into()),
                                        height: Val::Px(icon::IconSize::XSmall.into()),
                                    },
                                    ..default()
                                },
                                ..default()
                            });
                        });

                    children
                        .spawn(NineSliceBundle {
                            nine_slice: NineSlice {
                                image: asset_server.load("nine_slices/Button@2x.png"),
                                slice: UiRect {
                                    top: Val::Px(8.0),
                                    left: Val::Px(8.0),
                                    bottom: Val::Px(8.0),
                                    right: Val::Px(8.0),
                                },
                                size: Size {
                                    width: Val::Px(32.0),
                                    height: Val::Px(32.0),
                                },
                                ..default()
                            },
                            style: Style {
                                size: Size {
                                    height: Val::Px(24.0),
                                    width: Val::Px(32.0),
                                },
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::horizontal(Val::Px(4.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|children| {
                            children.spawn(ImageBundle {
                                image: UiImage {
                                    texture: Icon::named("Stop").request_icon(
                                        &asset_server,
                                        ui_scale.scale * logical_to_physical_factor,
                                        icon::IconSize::XSmall,
                                    ),
                                    ..default()
                                },
                                style: Style {
                                    flex_shrink: 0.0,
                                    size: Size {
                                        width: Val::Px(icon::IconSize::XSmall.into()),
                                        height: Val::Px(icon::IconSize::XSmall.into()),
                                    },
                                    ..default()
                                },
                                ..default()
                            });
                        });

                    children
                        .spawn(NineSliceBundle {
                            nine_slice: NineSlice {
                                image: asset_server.load("nine_slices/Select@2x.png"),
                                slice: UiRect {
                                    top: Val::Px(8.0),
                                    left: Val::Px(8.0),
                                    bottom: Val::Px(8.0),
                                    right: Val::Px(18.0),
                                },
                                size: Size {
                                    width: Val::Px(32.0),
                                    height: Val::Px(32.0),
                                },
                                ..default()
                            },
                            style: Style {
                                size: Size {
                                    height: Val::Px(24.0),
                                    width: Val::Px(120.0),
                                },
                                align_items: AlignItems::Center,
                                padding: UiRect {
                                    left: Val::Px(8.0),
                                    right: Val::Px(18.0),
                                    ..default()
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|children| {
                            children.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Debug".into(),
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

                            children.spawn(ImageBundle {
                                image: UiImage {
                                    texture: Icon::named("Expand").request_icon(
                                        &asset_server,
                                        ui_scale.scale * logical_to_physical_factor,
                                        icon::IconSize::XSmall,
                                    ),
                                    ..default()
                                },
                                style: Style {
                                    flex_shrink: 0.0,
                                    position_type: PositionType::Absolute,
                                    size: Size {
                                        width: Val::Px(icon::IconSize::XSmall.into()),
                                        height: Val::Px(icon::IconSize::XSmall.into()),
                                    },
                                    top: Val::Px(4.0),
                                    right: Val::Px(2.0),
                                    ..default()
                                },
                                ..default()
                            });
                        });

                    children
                        .spawn(NineSliceBundle {
                            nine_slice: NineSlice {
                                image: asset_server.load("nine_slices/Button@2x.png"),
                                slice: UiRect {
                                    top: Val::Px(8.0),
                                    left: Val::Px(8.0),
                                    bottom: Val::Px(8.0),
                                    right: Val::Px(8.0),
                                },
                                size: Size {
                                    width: Val::Px(32.0),
                                    height: Val::Px(32.0),
                                },
                                ..default()
                            },
                            style: Style {
                                size: Size {
                                    height: Val::Px(24.0),
                                    width: Val::Px(32.0),
                                },
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::horizontal(Val::Px(4.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|children| {
                            children.spawn(ImageBundle {
                                image: UiImage {
                                    texture: Icon::named("Launch").request_icon(
                                        &asset_server,
                                        ui_scale.scale * logical_to_physical_factor,
                                        icon::IconSize::XSmall,
                                    ),
                                    ..default()
                                },
                                style: Style {
                                    flex_shrink: 0.0,
                                    size: Size {
                                        width: Val::Px(icon::IconSize::XSmall.into()),
                                        height: Val::Px(icon::IconSize::XSmall.into()),
                                    },
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        });

    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(32.0),
                bottom: Val::Px(0.0),
                size: Size {
                    width: Val::Px(250.0),
                    ..default()
                },
                padding: UiRect::all(Val::Px(4.0)),
                gap: Size::all(Val::Px(4.0)),
                ..default()
            },
            background_color: BackgroundColor::from(bg_color),
            ..default()
        })
        .with_children(|children| {
            children
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        flex_basis: Val::Percent(50.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|children| {
                    children
                        .spawn(NineSliceBundle {
                            nine_slice: NineSlice {
                                image: asset_server.load("nine_slices/Sidebar.Heading@2x.png"),
                                slice: UiRect {
                                    top: Val::Px(8.0),
                                    left: Val::Px(8.0),
                                    bottom: Val::Px(8.0),
                                    right: Val::Px(8.0),
                                },
                                size: Size {
                                    width: Val::Px(32.0),
                                    height: Val::Px(32.0),
                                },
                                ..default()
                            },
                            style: Style {
                                padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|children| {
                            children.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Project".into(),
                                        style: text_style_semibold.clone(),
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

                    children
                        .spawn(NineSliceBundle {
                            nine_slice: NineSlice {
                                image: asset_server.load("nine_slices/Sidebar.Section@2x.png"),
                                slice: UiRect {
                                    top: Val::Px(8.0),
                                    left: Val::Px(8.0),
                                    bottom: Val::Px(8.0),
                                    right: Val::Px(8.0),
                                },
                                size: Size {
                                    width: Val::Px(32.0),
                                    height: Val::Px(32.0),
                                },
                                ..default()
                            },
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                flex_grow: 1.0,
                                flex_shrink: 1.0,
                                size: Size { ..default() },
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|children| {
                            children.spawn(TreeViewBundle::<ProjectItem> {
                                tree_view: TreeView {
                                    icon_size: icon::IconSize::Small,
                                },
                                style: Style {
                                    flex_basis: Val::Px(0.0),
                                    flex_grow: 1.0,
                                    size: Size {
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });

            children
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        flex_basis: Val::Percent(50.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|children| {
                    children
                        .spawn(NineSliceBundle {
                            nine_slice: NineSlice {
                                image: asset_server.load("nine_slices/Sidebar.Heading@2x.png"),
                                slice: UiRect {
                                    top: Val::Px(8.0),
                                    left: Val::Px(8.0),
                                    bottom: Val::Px(8.0),
                                    right: Val::Px(8.0),
                                },
                                size: Size {
                                    width: Val::Px(32.0),
                                    height: Val::Px(32.0),
                                },
                                ..default()
                            },
                            style: Style {
                                padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|children| {
                            children.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Scene".into(),
                                        style: text_style_semibold.clone(),
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

                    children
                        .spawn(NineSliceBundle {
                            nine_slice: NineSlice {
                                image: asset_server.load("nine_slices/Sidebar.Section@2x.png"),
                                slice: UiRect {
                                    top: Val::Px(8.0),
                                    left: Val::Px(8.0),
                                    bottom: Val::Px(8.0),
                                    right: Val::Px(8.0),
                                },
                                size: Size {
                                    width: Val::Px(32.0),
                                    height: Val::Px(32.0),
                                },
                                ..default()
                            },
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                flex_grow: 1.0,
                                size: Size { ..default() },
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|children| {
                            children.spawn(TreeViewBundle::<EditorItem> {
                                tree_view: TreeView {
                                    icon_size: icon::IconSize::XSmall,
                                },
                                style: Style {
                                    flex_grow: 1.0,
                                    flex_basis: Val::Px(0.0),
                                    size: Size { ..default() },
                                    ..default()
                                },
                                ..default()
                            });
                        });
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
