use bevy::prelude::*;

#[derive(Component, Default)]
pub struct EditorItem {
    pub name: Option<String>,
    pub inferred_type: EditorItemInferredType,
}

#[derive(Default)]
pub enum EditorItemInferredType {
    #[default]
    None,
    PointLight,
    SpotLight,
    DirectionalLight,
    Camera,
    Mesh,
}

fn update_editor_items(
    mut editor_items: Query<
        (
            &mut EditorItem,
            Option<&Name>,
            Option<&PointLight>,
            Option<&SpotLight>,
            Option<&DirectionalLight>,
            Option<&Camera>,
            Option<&Handle<Mesh>>,
        ),
        Or<(
            Changed<EditorItem>,
            Changed<Name>,
            Changed<PointLight>,
            Changed<SpotLight>,
            Changed<DirectionalLight>,
            Changed<Camera>,
            Changed<Handle<Mesh>>,
        )>,
    >,
) {
    for (mut editor_item, name, point_light, spot_light, directional_light, camera, mesh) in
        &mut editor_items
    {
        editor_item.name = name.map(|name| name.into());

        editor_item.inferred_type = if point_light.is_some() {
            EditorItemInferredType::PointLight
        } else if spot_light.is_some() {
            EditorItemInferredType::SpotLight
        } else if directional_light.is_some() {
            EditorItemInferredType::DirectionalLight
        } else if camera.is_some() {
            EditorItemInferredType::Camera
        } else if mesh.is_some() {
            EditorItemInferredType::Mesh
        } else {
            EditorItemInferredType::None
        }
    }
}

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_editor_items);
    }
}
