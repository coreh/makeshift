use bevy::prelude::*;
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Copy, Clone, Default, Debug)]
pub enum IconSize {
    #[default]
    XSmall = 16,
    Small = 24,
    Medium = 32,
}

impl From<IconSize> for f32 {
    fn from(value: IconSize) -> Self {
        value as i32 as f32
    }
}

pub trait IconProvider: Debug {
    fn request_icon(
        &self,
        asset_server: &Res<AssetServer>,
        scale: f64,
        size: IconSize,
    ) -> Handle<Image>;
}

impl IconProvider for Handle<Image> {
    fn request_icon(
        &self,
        _asset_server: &Res<AssetServer>,
        _scale: f64,
        _size: IconSize,
    ) -> Handle<Image> {
        self.clone()
    }
}

#[derive(Debug)]
pub struct NamedIcon(pub String);

impl IconProvider for NamedIcon {
    fn request_icon(
        &self,
        asset_server: &Res<AssetServer>,
        scale: f64,
        size: IconSize,
    ) -> Handle<Image> {
        let path = format!(
            "icons/{}.{}{}.png",
            self.0,
            match size {
                IconSize::XSmall => "16x16",
                IconSize::Small => "24x24",
                IconSize::Medium => "32x32",
            },
            if scale <= 1.0 { "" } else { "@2x" },
        );
        asset_server.load(path)
    }
}

pub struct Icon(Box<dyn IconProvider + Send + Sync>);

impl Icon {
    pub fn named(s: impl Into<String>) -> Icon {
        NamedIcon(s.into()).into()
    }
}

impl Deref for Icon {
    type Target = dyn IconProvider;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl From<NamedIcon> for Icon {
    fn from(named_icon: NamedIcon) -> Self {
        Icon(Box::from(named_icon))
    }
}

impl From<Handle<Image>> for Icon {
    fn from(handle: Handle<Image>) -> Self {
        Icon(Box::from(handle))
    }
}
