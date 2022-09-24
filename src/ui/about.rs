use adw::prelude::*;
use relm4::*;

use crate::config;

use super::window::AppMsg;

#[derive(Debug)]
pub struct AboutPageModel;

#[relm4::component(pub)]
impl SimpleComponent for AboutPageModel {
    type InitParams = gtk::Window;
    type Input = ();
    type Output = AppMsg;
    type Widgets = AboutPageWidgets;

    view! {
        adw::AboutWindow {
            set_transient_for: Some(&parent_window),
            set_modal: true,
            set_application_name: "NixOS Configuration Editor",
            set_application_icon: config::APP_ID,
            set_developer_name: "Victor Fuentes",
            set_version: config::VERSION,
            set_issue_url: "https://github.com/vlinkz/nixos-conf-editor/issues",
            set_license_type: gtk::License::Gpl30Only,
            set_website: "https://github.com/vlinkz/nixos-conf-editor",
            set_developers: &["Victor Fuentes https://github.com/vlinkz"],
        }
    }

    fn init(
        parent_window: Self::InitParams,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AboutPageModel;

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {}
}
