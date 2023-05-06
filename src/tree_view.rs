use std::marker::PhantomData;

use bevy::{prelude::*, ui::FocusPolicy, utils::HashMap, window::PrimaryWindow};

use crate::icon::{Icon, IconSize};

pub trait TreeViewItem {
    fn title(&self) -> String;
    fn icon(&self) -> Icon;
    fn is_selected(&self) -> bool;
    fn is_hovered(&self) -> bool;
}

#[derive(Component, Clone, Debug, Default)]
pub struct TreeView {
    pub icon_size: IconSize,
}

#[derive(Component, Clone, Debug)]
pub struct TreeViewState<T: TreeViewItem + Component> {
    content_node: Option<Entity>,
    item_by_node: HashMap<Entity, Entity>,
    node_by_item: HashMap<Entity, Entity>,
    item_by_row: HashMap<Entity, Entity>,
    row_by_item: HashMap<Entity, Entity>,
    disclosure_by_item: HashMap<Entity, Entity>,
    item_by_disclosure: HashMap<Entity, Entity>,
    icon_by_item: HashMap<Entity, Entity>,
    item_by_icon: HashMap<Entity, Entity>,
    label_by_item: HashMap<Entity, Entity>,
    item_by_label: HashMap<Entity, Entity>,
    child_slot_by_item: HashMap<Entity, Entity>,
    item_by_child_slot: HashMap<Entity, Entity>,
    _item: PhantomData<T>,
}

impl<T: TreeViewItem + Component> Default for TreeViewState<T> {
    fn default() -> Self {
        TreeViewState::<T> {
            content_node: None,
            item_by_node: Default::default(),
            node_by_item: Default::default(),
            item_by_row: Default::default(),
            row_by_item: Default::default(),
            disclosure_by_item: Default::default(),
            item_by_disclosure: Default::default(),
            icon_by_item: Default::default(),
            item_by_icon: Default::default(),
            label_by_item: Default::default(),
            item_by_label: Default::default(),
            child_slot_by_item: Default::default(),
            item_by_child_slot: Default::default(),
            _item: Default::default(),
        }
    }
}

#[derive(Component)]
struct TreeViewContent;

#[derive(Component)]
struct TreeViewNode;

#[derive(Component)]
struct TreeViewRow;

#[derive(Component)]
struct TreeViewIcon;

#[derive(Component)]
struct TreeViewLabel;

#[derive(Component)]
struct TreeViewDisclosureButton;

#[derive(Component)]
struct TreeViewChildSlot;

fn update_tree_views<T: TreeViewItem + Component>(
    mut commands: Commands,
    mut tree_views: Query<(Entity, &mut TreeView, &mut TreeViewState<T>)>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    items: Query<&T>,
    changed_items: Query<(Entity, &T, Option<&Children>), Changed<T>>,
    reparented_items: Query<(Entity, &Parent), (With<T>, Changed<Parent>)>,
    rechilded_items: Query<(Entity, &Children), (With<T>, Changed<Children>)>,
    mut orphaned_items: RemovedComponents<Parent>,
    mut removed_items: RemovedComponents<T>,
    asset_server: Res<AssetServer>,
    ui_scale: Res<UiScale>,
) {
    // assume one window for time being...
    // TODO: Support window-independent scaling: https://github.com/bevyengine/bevy/issues/5621
    let logical_to_physical_factor = if let Ok(primary_window) = primary_window.get_single() {
        primary_window.resolution.scale_factor()
    } else {
        1.0
    };

    let text_style = TextStyle {
        font: asset_server.load("fonts/Inter-Regular.ttf"),
        font_size: 14.0,
        color: Color::WHITE,
    };

    for (tree_view_entity, tree_view, mut tree_view_state) in &mut tree_views {
        let content_node = if let Some(content_node) = tree_view_state.content_node {
            content_node
        } else {
            let content_node = commands
                .spawn((
                    TreeViewContent,
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            overflow: Overflow {
                                x: OverflowAxis::Clip,
                                y: OverflowAxis::Clip,
                            },
                            max_size: Size {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                            },
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(tree_view_entity)
                .id();

            tree_view_state.content_node = Some(content_node);

            content_node
        };

        for (item_entity, ref item, item_children) in &changed_items {
            let tree_node = if let Some(entity) = tree_view_state.node_by_item.get(&item_entity) {
                *entity
            } else {
                let mut tree_node = commands.spawn((
                    TreeViewNode,
                    NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Start,
                            overflow: Overflow {
                                x: OverflowAxis::Clip,
                                y: OverflowAxis::Clip,
                            },
                            ..default()
                        },
                        ..default()
                    },
                ));

                let tree_node_id = tree_node.id();

                tree_view_state
                    .node_by_item
                    .insert(item_entity, tree_node_id);
                tree_view_state
                    .item_by_node
                    .insert(tree_node_id, item_entity);

                tree_node.set_parent(content_node);

                tree_node.id()
            };

            // Row
            let row_entity = if let Some(row_entity) = tree_view_state.row_by_item.get(&item_entity)
            {
                commands.entity(*row_entity).despawn_descendants();
                *row_entity
            } else {
                let row_entity = commands
                    .spawn((
                        TreeViewRow,
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(2.0)),
                                gap: Size::all(Val::Px(4.0)),
                                ..default()
                            },
                            ..default()
                        },
                    ))
                    .set_parent(tree_node)
                    .id();

                tree_view_state.row_by_item.insert(item_entity, row_entity);
                tree_view_state.item_by_row.insert(row_entity, item_entity);

                row_entity
            };

            commands
                .entity(row_entity)
                .insert(BackgroundColor::from(if item.is_selected() {
                    Color::rgba(0.0, 0.4, 1.0, 0.3)
                } else if item.is_hovered() {
                    Color::rgba(1.0, 1.0, 1.0, 0.05)
                } else {
                    Color::NONE
                }));

            // Disclosure Button
            let disclosure_entity = commands
                .spawn((
                    TreeViewDisclosureButton,
                    Interaction::None,
                    Button,
                    ImageBundle {
                        image: UiImage {
                            texture: Icon::named("Disclosure.Expanded").request_icon(
                                &asset_server,
                                ui_scale.scale * logical_to_physical_factor,
                                tree_view.icon_size,
                            ),
                            ..default()
                        },
                        style: Style {
                            flex_shrink: 0.0,
                            size: Size {
                                width: Val::Px(tree_view.icon_size.into()),
                                height: Val::Px(tree_view.icon_size.into()),
                            },
                            ..default()
                        },
                        visibility: if item_children.map_or(0, |children| {
                            children
                                .iter()
                                .filter(|child_entity| items.contains(**child_entity))
                                .count()
                        }) == 0
                        {
                            Visibility::Hidden
                        } else {
                            Visibility::Inherited
                        },
                        ..default()
                    },
                ))
                .set_parent(row_entity)
                .id();

            tree_view_state
                .disclosure_by_item
                .insert(item_entity, disclosure_entity);

            tree_view_state
                .item_by_disclosure
                .insert(disclosure_entity, item_entity);

            // Icon
            let icon_entity = commands
                .spawn((
                    TreeViewIcon,
                    ImageBundle {
                        image: UiImage {
                            texture: item.icon().request_icon(
                                &asset_server,
                                ui_scale.scale * logical_to_physical_factor,
                                tree_view.icon_size,
                            ),
                            ..default()
                        },
                        style: Style {
                            flex_shrink: 0.0,
                            size: Size {
                                width: Val::Px(tree_view.icon_size.into()),
                                height: Val::Px(tree_view.icon_size.into()),
                            },
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(row_entity)
                .id();

            tree_view_state
                .icon_by_item
                .insert(item_entity, icon_entity);

            tree_view_state
                .item_by_icon
                .insert(icon_entity, item_entity);

            // Label
            let label_entity = commands
                .spawn((
                    TreeViewLabel,
                    TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: item.title(),
                                style: text_style.clone(),
                            }],
                            ..default()
                        },
                        style: Style {
                            flex_shrink: 0.0,
                            flex_basis: Val::Auto,
                            size: Size {
                                width: Val::Px(30000.0),
                                ..Default::default()
                            },
                            ..default()
                        },
                        ..default()
                    },
                ))
                .set_parent(row_entity)
                .id();

            tree_view_state
                .label_by_item
                .insert(item_entity, label_entity);

            tree_view_state
                .item_by_label
                .insert(label_entity, item_entity);

            // Child Slot
            let child_slot_entity = if let Some(child_slot_entity) =
                tree_view_state.child_slot_by_item.get(&item_entity)
            {
                *child_slot_entity
            } else {
                commands
                    .spawn((
                        TreeViewChildSlot,
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                padding: UiRect {
                                    left: Val::Px(tree_view.icon_size.into()),
                                    ..default()
                                },
                                ..default()
                            },
                            ..default()
                        },
                    ))
                    .set_parent(tree_node)
                    .id()
            };

            tree_view_state
                .child_slot_by_item
                .insert(item_entity, child_slot_entity);

            tree_view_state
                .item_by_child_slot
                .insert(child_slot_entity, item_entity);
        }

        for (item_entity, parent_item_entity) in &reparented_items {
            let node_entity = tree_view_state.node_by_item.get(&item_entity).unwrap();
            let parent_node_entity = tree_view_state
                .child_slot_by_item
                .get(&parent_item_entity.get())
                .unwrap();

            commands
                .entity(*node_entity)
                .set_parent(*parent_node_entity);
        }

        for item_entity in orphaned_items.iter() {
            if let Some(node_entity) = tree_view_state.node_by_item.get(&item_entity) {
                commands.entity(*node_entity).set_parent(content_node);
            }
        }

        for (item_entity, item_children) in &rechilded_items {
            let disclosure_entity = tree_view_state
                .disclosure_by_item
                .get(&item_entity)
                .unwrap();

            if item_children
                .iter()
                .filter(|child_entity| items.contains(**child_entity))
                .count()
                > 0
            {
                commands
                    .entity(*disclosure_entity)
                    .insert(Visibility::Inherited);
            } else {
                commands
                    .entity(*disclosure_entity)
                    .insert(Visibility::Hidden);
            }
        }

        for item_entity in removed_items.iter() {
            // Deregister Child Slot
            let child_slot_entity = tree_view_state
                .child_slot_by_item
                .remove(&item_entity)
                .unwrap();
            tree_view_state
                .item_by_child_slot
                .remove(&child_slot_entity);

            // Deregister Disclosure
            let disclosure_entity = tree_view_state
                .disclosure_by_item
                .remove(&item_entity)
                .unwrap();
            tree_view_state
                .item_by_disclosure
                .remove(&disclosure_entity);

            // Deregister Icon
            let icon_entity = tree_view_state.icon_by_item.remove(&item_entity).unwrap();
            tree_view_state.item_by_icon.remove(&icon_entity);

            // Deregister Label
            let label_entity = tree_view_state.label_by_item.remove(&item_entity).unwrap();
            tree_view_state.item_by_label.remove(&label_entity);

            // Deregister and Despawn Node Entity
            let node_entity = tree_view_state.node_by_item.remove(&item_entity).unwrap();
            tree_view_state.item_by_node.remove(&node_entity);
            commands.entity(node_entity).despawn();
        }
    }
}

fn handle_disclosure_click<T: TreeViewItem + Component>(
    mut tree_views: Query<(&mut TreeView, &TreeViewState<T>)>,
    asset_server: Res<AssetServer>,
    ui_scale: Res<UiScale>,
    mut interacted_disclosures: Query<
        (Entity, &mut UiImage, &Interaction),
        (Changed<Interaction>, With<TreeViewDisclosureButton>),
    >,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut child_slots: Query<&mut Style, With<TreeViewChildSlot>>,
) {
    // assume one window for time being...
    // TODO: Support window-independent scaling: https://github.com/bevyengine/bevy/issues/5621
    let logical_to_physical_factor = if let Ok(primary_window) = primary_window.get_single() {
        primary_window.resolution.scale_factor()
    } else {
        1.0
    };

    for (disclosure_entity, mut disclosure_image, interaction) in &mut interacted_disclosures {
        for (tree_view, tree_view_state) in &mut tree_views {
            if let Some(item_entity) = tree_view_state.item_by_disclosure.get(&disclosure_entity) {
                match interaction {
                    Interaction::Clicked => {
                        let child_slot =
                            tree_view_state.child_slot_by_item.get(item_entity).unwrap();
                        let mut child_slot_style = child_slots.get_mut(*child_slot).unwrap();
                        match child_slot_style.display {
                            Display::None => {
                                child_slot_style.display = Display::Flex;
                                disclosure_image.texture = Icon::named("Disclosure.Expanded")
                                    .request_icon(
                                        &asset_server,
                                        ui_scale.scale * logical_to_physical_factor,
                                        tree_view.icon_size,
                                    );
                            }
                            _ => {
                                child_slot_style.display = Display::None;
                                disclosure_image.texture = Icon::named("Disclosure.Collapsed")
                                    .request_icon(
                                        &asset_server,
                                        ui_scale.scale * logical_to_physical_factor,
                                        tree_view.icon_size,
                                    );
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn sort_child_slot_children<T: TreeViewItem + Component>(
    mut tree_views: Query<&TreeViewState<T>>,
    mut rechilded_child_slots: Query<
        (Entity, &mut Children),
        (
            Or<(With<TreeViewChildSlot>, With<TreeViewContent>)>,
            Changed<Children>,
        ),
    >,
    items: Query<&T>,
) {
    for (child_slot_entity, mut children) in &mut rechilded_child_slots {
        for tree_view_state in &mut tree_views {
            if tree_view_state
                .item_by_child_slot
                .contains_key(&child_slot_entity)
                || tree_view_state.content_node == Some(child_slot_entity)
            {
                children.sort_by_key(|child| {
                    let item_entity = tree_view_state.item_by_node.get(child).unwrap();
                    let item = items.get(*item_entity).unwrap();
                    item.title()
                });
            }
        }
    }
}

pub struct TreeViewPlugin<T: TreeViewItem + Component> {
    _item: PhantomData<T>,
}

impl<T: TreeViewItem + Component> Default for TreeViewPlugin<T> {
    fn default() -> Self {
        TreeViewPlugin {
            _item: Default::default(),
        }
    }
}

impl<T: TreeViewItem + Component> Plugin for TreeViewPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_tree_views::<T>,
                handle_disclosure_click::<T>,
                sort_child_slot_children::<T>,
            ),
        );
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct TreeViewBundle<T: TreeViewItem + Component> {
    pub tree_view: TreeView,
    pub tree_view_state: TreeViewState<T>,
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

impl<T: TreeViewItem + Component> Default for TreeViewBundle<T> {
    fn default() -> Self {
        TreeViewBundle {
            tree_view: TreeView::default(),
            tree_view_state: TreeViewState::default(),
            background_color: Color::NONE.into(),
            node: Default::default(),
            style: Default::default(),
            focus_policy: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            z_index: Default::default(),
        }
    }
}
