use find_folder;
use gtk::{Box, ContainerExt, IconSize, Image, Label, Menu, MenuItem, MenuItemExt, MenuShellExt,
          Orientation, WidgetExt};
use libappindicator::{AppIndicator, AppIndicatorStatus};

#[derive(Debug)]
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
            TrayStatus::Unknown => "grey.png",
            TrayStatus::Success => "blue.png",
            TrayStatus::Unstable => "yellow.png",
            TrayStatus::Failure => "red.png",
            TrayStatus::NotBuilt => "grey.png",
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

    pub fn add_menu_item<F: Fn() + 'static>(&mut self,
                                            label: &str,
                                            icon_name: Option<&str>,
                                            callback: F) {
        let item = create_menu_item(label, icon_name);
        item.connect_activate(move |_| callback());
        self.menu.append(&item);
    }

    pub fn show_all(&mut self) {
        self.menu.show_all();
    }
}
