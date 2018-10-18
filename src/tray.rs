/*
Copyright (C) 2017  Armin Häberling

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
use find_folder;
use gtk::{
    Box, ContainerExt, IconSize, Image, Label, Menu, MenuItem, MenuItemExt, MenuShellExt,
    Orientation, WidgetExt,
};
use libappindicator::{AppIndicator, AppIndicatorStatus};

#[derive(Debug, Clone, Copy)]
pub enum TrayStatus {
    Unknown,
    Success,
    Unstable,
    Failure,
    NotBuilt,
}

impl TrayStatus {
    fn icon_name(&self) -> String {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let file = match *self {
            TrayStatus::Unknown | TrayStatus::NotBuilt => "grey.png",
            TrayStatus::Success => "blue.png",
            TrayStatus::Unstable => "yellow.png",
            TrayStatus::Failure => "red.png",
        };
        assets.join(file).to_string_lossy().into_owned()
    }

    fn desc(&self) -> &str {
        match *self {
            TrayStatus::Unknown => "Unknown",
            TrayStatus::Success => "Success",
            TrayStatus::Unstable => "Unstable",
            TrayStatus::Failure => "Failure",
            TrayStatus::NotBuilt => "NotBuilt",
        }
    }
}

pub struct Tray {
    indicator: AppIndicator,
    menu: Menu,
}

fn create_menu_item(label: &str, icon_name: Option<&str>) -> MenuItem {
    if let Some(icon_name) = icon_name {
        let icon = Image::new_from_icon_name(icon_name, IconSize::Menu.into());
        let lbl = Label::new(label);
        let b = Box::new(Orientation::Horizontal, 6);
        b.add(&icon);
        b.add(&lbl);
        let mi = MenuItem::new();
        mi.add(&b);
        mi
    } else {
        MenuItem::new_with_label(label)
    }
}

impl Tray {
    pub fn new() -> Tray {
        let mut indicator = AppIndicator::new("Jenkins Tray", "");
        indicator.set_status(AppIndicatorStatus::APP_INDICATOR_STATUS_ACTIVE);
        let status = TrayStatus::Unknown;
        indicator.set_icon_full(&status.icon_name(), status.desc());

        let mut menu = Menu::new();
        indicator.set_menu(&mut menu);

        Tray { indicator, menu }
    }

    pub fn set_status(&mut self, status: TrayStatus) {
        self.indicator
            .set_icon_full(&status.icon_name(), status.desc());
    }

    pub fn add_menu_item<F>(&mut self, label: &str, icon_name: Option<&str>, callback: F)
    where
        F: Fn() + 'static,
    {
        let item = create_menu_item(label, icon_name);
        item.connect_activate(move |_| callback());
        self.menu.append(&item);
        self.menu.show_all();
    }
}
