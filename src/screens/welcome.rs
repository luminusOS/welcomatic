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

use gtk::subclass::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/luminus/Welcomatic/ui/welcome.ui")]
    pub struct WelcomaticWelcome {
        #[template_child]
        pub btn_start: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WelcomaticWelcome {
        const NAME: &'static str = "WelcomaticWelcome";
        type Type = super::WelcomaticWelcome;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for WelcomaticWelcome {}
    impl WidgetImpl for WelcomaticWelcome {}
    impl BinImpl for WelcomaticWelcome {}
}

glib::wrapper! {
    pub struct WelcomaticWelcome(ObjectSubclass<imp::WelcomaticWelcome>)
        @extends gtk::Widget, adw::Bin,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl WelcomaticWelcome {
    pub fn new() -> Self {
        glib::Object::new::<Self>()
    }
}
