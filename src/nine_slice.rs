use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE, ui::FocusPolicy};

#[derive(Component, Clone, Debug)]
pub struct NineSlice {
    pub image: Handle<Image>,
    pub width: Val,
    pub height: Val,
    pub slice: UiRect,
}

impl Default for NineSlice {
    fn default() -> Self {
        NineSlice {
            image: DEFAULT_IMAGE_HANDLE.typed(),
            width: Val::Auto,
            height: Val::Auto,
            slice: UiRect {
                left: Val::Percent(0.0),
                right: Val::Percent(0.0),
                top: Val::Percent(0.0),
                bottom: Val::Percent(0.0),
            },
        }
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct NineSliceState {
    content_node: Option<Entity>,
    slices: Option<NineSliceEntities>,
    images: Option<NineSliceEntities>,
}

#[derive(Clone, Debug)]
pub struct NineSliceEntities {
    top_left: Entity,
    top: Entity,
    top_right: Entity,
    left: Entity,
    center: Entity,
    right: Entity,
    bottom_left: Entity,
    bottom: Entity,
    bottom_right: Entity,
}

#[derive(Component, Clone, Debug)]
struct NineSliceSlice;

#[derive(Bundle, Clone, Debug)]
pub struct NineSliceBundle {
    pub nine_slice: NineSlice,
    pub nine_slice_state: NineSliceState,
    pub node: Node,
    pub style: Style,
    pub background_color: BackgroundColor,
    pub focus_policy: FocusPolicy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub z_index: ZIndex,
}

impl Default for NineSliceBundle {
    fn default() -> Self {
        NineSliceBundle {
            nine_slice: NineSlice::default(),
            nine_slice_state: NineSliceState::default(),
            node: Node::default(),
            style: Style::default(),
            background_color: Color::NONE.into(),
            focus_policy: FocusPolicy::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            z_index: ZIndex::default(),
        }
    }
}

fn update_nine_slices(
    mut commands: Commands,
    mut changed_nine_slices: Query<(Entity, &NineSlice, &mut NineSliceState), Changed<NineSlice>>,
) {
    for (nine_slice_entity, nine_slice, mut state) in &mut changed_nine_slices {
        let content_node = if let Some(content_node) = state.content_node {
            commands.entity(content_node).despawn_descendants();
            content_node
        } else {
            let content_node = commands
                .spawn((NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::clip(),
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.0),
                        left: Val::Px(0.0),
                        right: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        ..default()
                    },
                    z_index: ZIndex::Local(-1),
                    ..default()
                },))
                .set_parent(nine_slice_entity)
                .id();

            state.content_node = Some(content_node);

            content_node
        };

        let left_percent = match &nine_slice.slice.left {
            Val::Px(left_px) => match nine_slice.width {
                Val::Px(width_px) => left_px / width_px * 100.0,
                _ => panic!("Using a pixel value for `left` in `NineSlice::slice` also requires using a pixel value for `width` in `NineSlice::size`"),
            },
            Val::Percent(left_percent) => *left_percent,
            other => panic!("Unsupported value for `left` in `NineSlice::slice`: {:?}", other),
        };

        let right_percent = match &nine_slice.slice.right {
            Val::Px(right_px) => match nine_slice.width {
                Val::Px(width_px) => right_px / width_px * 100.0,
                _ => panic!("Using a pixel value for `right` in `NineSlice::slice` also requires using a pixel value for `width` in `NineSlice::size`"),
            },
            Val::Percent(right_percent) => *right_percent,
            other => panic!("Unsupported value for `right` in `NineSlice::slice`: {:?}", other),
        };

        let top_percent = match &nine_slice.slice.top {
            Val::Px(top_px) => match nine_slice.width {
                Val::Px(width_px) => top_px / width_px * 100.0,
                _ => panic!("Using a pixel value for `top` in `NineSlice::slice` also requires using a pixel value for `height` in `NineSlice::size`"),
            },
            Val::Percent(top_percent) => *top_percent,
            other => panic!("Unsupported value for `top` in `NineSlice::slice`: {:?}", other),
        };

        let bottom_percent = match &nine_slice.slice.bottom {
            Val::Px(bottom_px) => match nine_slice.width {
                Val::Px(width_px) => bottom_px / width_px * 100.0,
                _ => panic!("Using a pixel value for `bottom` in `NineSlice::slice` also requires using a pixel value for `height` in `NineSlice::size`"),
            },
            Val::Percent(bottom_percent) => *bottom_percent,
            other => panic!("Unsupported value for `bottom` in `NineSlice::slice`: {:?}", other),
        };

        let width_percent = 100.0 - right_percent - left_percent;
        let height_percent = 100.0 - top_percent - bottom_percent;

        let slices = NineSliceEntities {
            top_left: commands
                .spawn((
                    NineSliceSlice,
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            overflow: Overflow::clip(),
                            top: Val::Px(0.0),
                            left: Val::Px(0.0),
                            height: nine_slice.slice.top,
                            width: nine_slice.slice.left,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(content_node)
                .id(),
            top: commands
                .spawn((
                    NineSliceSlice,
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            overflow: Overflow::clip(),
                            top: Val::Px(0.0),
                            left: nine_slice.slice.left,
                            right: nine_slice.slice.right,
                            height: nine_slice.slice.top,
                            width: Val::Auto,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(content_node)
                .id(),
            top_right: commands
                .spawn((
                    NineSliceSlice,
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            overflow: Overflow::clip(),
                            top: Val::Px(0.0),
                            right: Val::Px(0.0),
                            height: nine_slice.slice.top,
                            width: nine_slice.slice.right,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(content_node)
                .id(),
            left: commands
                .spawn((
                    NineSliceSlice,
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            overflow: Overflow::clip(),
                            top: nine_slice.slice.top,
                            left: Val::Px(0.0),
                            height: Val::Auto,
                            width: nine_slice.slice.left,
                            bottom: nine_slice.slice.bottom,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(content_node)
                .id(),
            center: commands
                .spawn((
                    NineSliceSlice,
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            overflow: Overflow::clip(),
                            top: nine_slice.slice.top,
                            left: nine_slice.slice.left,
                            bottom: nine_slice.slice.bottom,
                            right: nine_slice.slice.right,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(content_node)
                .id(),
            right: commands
                .spawn((
                    NineSliceSlice,
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            overflow: Overflow::clip(),
                            top: nine_slice.slice.top,
                            right: Val::Px(0.0),
                            height: Val::Auto,
                            width: nine_slice.slice.right,
                            bottom: nine_slice.slice.bottom,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(content_node)
                .id(),
            bottom_left: commands
                .spawn((
                    NineSliceSlice,
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            overflow: Overflow::clip(),
                            bottom: Val::Px(0.0),
                            left: Val::Px(0.0),
                            height: nine_slice.slice.bottom,
                            width: nine_slice.slice.left,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(content_node)
                .id(),
            bottom: commands
                .spawn((
                    NineSliceSlice,
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            overflow: Overflow::clip(),
                            bottom: Val::Px(0.0),
                            left: nine_slice.slice.left,
                            right: nine_slice.slice.right,
                            height: nine_slice.slice.bottom,
                            width: Val::Auto,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(content_node)
                .id(),
            bottom_right: commands
                .spawn((
                    NineSliceSlice,
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            overflow: Overflow::clip(),
                            bottom: Val::Px(0.0),
                            right: Val::Px(0.0),
                            height: nine_slice.slice.bottom,
                            width: nine_slice.slice.right,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(content_node)
                .id(),
        };

        let images = NineSliceEntities {
            top_left: commands
                .spawn((
                    NineSliceSlice,
                    ImageBundle {
                        image: UiImage {
                            texture: nine_slice.image.clone(),
                            ..default()
                        },
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(0.0),
                            left: Val::Px(0.0),
                            width: Val::Percent(1.0 / (left_percent / 100.0) * 100.0),
                            height: Val::Percent(1.0 / (top_percent / 100.0) * 100.0),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(slices.top_left)
                .id(),
            top: commands
                .spawn((
                    NineSliceSlice,
                    ImageBundle {
                        image: UiImage {
                            texture: nine_slice.image.clone(),
                            ..default()
                        },
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(0.0),
                            left: Val::Percent(-left_percent / width_percent * 100.0),
                            right: Val::Percent(-right_percent / width_percent * 100.0),
                            width: Val::Auto,
                            height: Val::Percent(100.0 / top_percent * 100.0),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(slices.top)
                .id(),
            top_right: commands
                .spawn((
                    NineSliceSlice,
                    ImageBundle {
                        image: UiImage {
                            texture: nine_slice.image.clone(),
                            ..default()
                        },
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(0.0),
                            right: Val::Px(0.0),
                            width: Val::Percent(100.0 / right_percent * 100.0),
                            height: Val::Percent(100.0 / top_percent * 100.0),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(slices.top_right)
                .id(),
            left: commands
                .spawn((
                    NineSliceSlice,
                    ImageBundle {
                        image: UiImage {
                            texture: nine_slice.image.clone(),
                            ..default()
                        },
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.0),
                            top: Val::Percent(-top_percent / height_percent * 100.0),
                            bottom: Val::Percent(-bottom_percent / height_percent * 100.0),
                            width: Val::Percent(100.0 / left_percent * 100.0),
                            height: Val::Auto,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(slices.left)
                .id(),
            center: commands
                .spawn((
                    NineSliceSlice,
                    ImageBundle {
                        image: UiImage {
                            texture: nine_slice.image.clone(),
                            ..default()
                        },
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Percent(-top_percent / height_percent * 100.0),
                            bottom: Val::Percent(-bottom_percent / height_percent * 100.0),
                            left: Val::Percent(-left_percent / width_percent * 100.0),
                            right: Val::Percent(-right_percent / width_percent * 100.0),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(slices.center)
                .id(),
            right: commands
                .spawn((
                    NineSliceSlice,
                    ImageBundle {
                        image: UiImage {
                            texture: nine_slice.image.clone(),
                            ..default()
                        },
                        style: Style {
                            position_type: PositionType::Absolute,
                            right: Val::Px(0.0),
                            top: Val::Percent(-top_percent / height_percent * 100.0),
                            bottom: Val::Percent(-bottom_percent / height_percent * 100.0),
                            width: Val::Percent(100.0 / right_percent * 100.0),
                            height: Val::Auto,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(slices.right)
                .id(),
            bottom_left: commands
                .spawn((
                    NineSliceSlice,
                    ImageBundle {
                        image: UiImage {
                            texture: nine_slice.image.clone(),
                            ..default()
                        },
                        style: Style {
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(0.0),
                            left: Val::Px(0.0),
                            width: Val::Percent(1.0 / (left_percent / 100.0) * 100.0),
                            height: Val::Percent(1.0 / (bottom_percent / 100.0) * 100.0),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(slices.bottom_left)
                .id(),
            bottom: commands
                .spawn((
                    NineSliceSlice,
                    ImageBundle {
                        image: UiImage {
                            texture: nine_slice.image.clone(),
                            ..default()
                        },
                        style: Style {
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(0.0),
                            left: Val::Percent(-left_percent / width_percent * 100.0),
                            right: Val::Percent(-right_percent / width_percent * 100.0),
                            width: Val::Auto,
                            height: Val::Percent(100.0 / bottom_percent * 100.0),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(slices.bottom)
                .id(),
            bottom_right: commands
                .spawn((
                    NineSliceSlice,
                    ImageBundle {
                        image: UiImage {
                            texture: nine_slice.image.clone(),
                            ..default()
                        },
                        style: Style {
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(0.0),
                            right: Val::Px(0.0),
                            width: Val::Percent(1.0 / (right_percent / 100.0) * 100.0),
                            height: Val::Percent(1.0 / (bottom_percent / 100.0) * 100.0),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(slices.bottom_right)
                .id(),
        };
        state.slices = Some(slices);
        state.images = Some(images);
    }
}

pub struct NineSlicePlugin;

impl Plugin for NineSlicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_nine_slices);
    }
}
