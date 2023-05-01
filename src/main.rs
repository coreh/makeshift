use bevy::prelude::*;
use icon::Icon;
use project::{ProjectEvent, ProjectItem, ProjectPlugin};
use tree_view::{TreeView, TreeViewBundle, TreeViewItem, TreeViewPlugin};
use uuid::Uuid;

mod icon;
mod project;
mod tree_view;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ProjectPlugin)
        .add_plugin(TreeViewPlugin::<ProjectItem>::default())
        .add_systems(Startup, create_tree_view)
        .add_systems(Startup, create_sample_items)
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
}

fn create_tree_view(mut commands: Commands) {
    commands.spawn(TreeViewBundle::<ProjectItem> {
        tree_view: TreeView {
            icon_size: icon::IconSize::XSmall,
        },
        style: Style {
            size: Size {
                width: Val::Px(200.0),
                height: Val::Percent(100.0),
            },
            ..default()
        },
        background_color: BackgroundColor::from(Color::DARK_GRAY),
        ..default()
    });

    commands.spawn(Camera3dBundle::default());
}
