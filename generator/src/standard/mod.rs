// Based on https://specifications.freedesktop.org/icon-naming-spec/latest/ar01s04.html

use std::collections::BTreeMap;

mod actions;
mod animations;
mod apps;
mod categories;
mod devices;
mod emblems;
mod emotes;
mod intl;
mod mimetypes;
mod places;
mod status;

pub fn standard() -> BTreeMap<&'static str, &'static [&'static str]> {
    let mut standard = BTreeMap::new();
    standard.insert("actions", actions::ICONS);
    standard.insert("animations", animations::ICONS);
    standard.insert("apps", apps::ICONS);
    standard.insert("categories", categories::ICONS);
    standard.insert("devices", devices::ICONS);
    standard.insert("emblems", emblems::ICONS);
    standard.insert("emotes", emotes::ICONS);
    standard.insert("intl", intl::ICONS);
    standard.insert("mimetypes", mimetypes::ICONS);
    standard.insert("places", places::ICONS);
    standard.insert("status", status::ICONS);
    standard
}
