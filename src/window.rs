/* window.rs
 *
 * Copyright 2023 Leandro Rodrigues
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

// use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::screens::welcome::WelcomaticWelcome;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/luminus/Welcomatic/ui/window.ui")]
    pub struct WelcomaticWindow {
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub carousel: TemplateChild<adw::Carousel>,
        #[template_child]
        pub carousel_indicator_dots: TemplateChild<adw::CarouselIndicatorDots>,
        #[template_child]
        pub btn_back: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WelcomaticWindow {
        const NAME: &'static str = "WelcomaticWindow";
        type Type = super::WelcomaticWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for WelcomaticWindow {

        fn constructed(&self) {
            let welcome_screen = WelcomaticWelcome::new();
            self.carousel.append(&welcome_screen);
        }
    }
    impl WidgetImpl for WelcomaticWindow {}
    impl WindowImpl for WelcomaticWindow {}
    impl ApplicationWindowImpl for WelcomaticWindow {}
    impl AdwApplicationWindowImpl for WelcomaticWindow {}
}

glib::wrapper! {
    pub struct WelcomaticWindow(ObjectSubclass<imp::WelcomaticWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl WelcomaticWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
