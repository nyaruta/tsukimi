#[doc(hidden)]
#[macro_export]
macro_rules! fraction {
    ($widget:expr) => {{
        use gtk::prelude::WidgetExt;
        if let Some(root) = $widget.root() {
            if let Some(window) = root.downcast_ref::<$crate::ui::widgets::window::Window>() {
                window.set_progressbar_fade();
            }
        }
    }};
}

#[macro_export]
macro_rules! fraction_reset {
    ($widget:expr) => {{
        use gtk::prelude::WidgetExt;
        if let Some(root) = $widget.root() {
            if let Some(window) = root.downcast_ref::<$crate::ui::widgets::window::Window>() {
                window.set_progressbar_opacity(1.0);
                window.hard_set_fraction(0.0);
                window.set_fraction(1.0);
            }
        }
    }};
}

#[macro_export]
macro_rules! insert_editm_dialog {
    ($widget:expr, $dialog:expr) => {{
        use adw::prelude::*;
        use gtk::prelude::WidgetExt;
        if let Some(root) = $widget.root() {
            if let Some(window) = root.downcast_ref::<$crate::ui::widgets::window::Window>() {
                $dialog.present(Some(window));
            } else {
                panic!("Trying to display a dialog when the parent doesn't support it");
            }
        }
    }};
}

#[macro_export]
macro_rules! bing_song_model {
    ($widget:expr, $active_model:expr, $active_core_song:expr) => {{
        use adw::prelude::*;
        use gtk::{
            glib,
            prelude::WidgetExt,
        };
        use $crate::utils::spawn;

        let root = $widget.root();
        let Some(window) = root.and_downcast_ref::<$crate::ui::widgets::window::Window>() else {
            return;
        };
        spawn(glib::clone!(
            #[strong]
            window,
            #[strong(rename_to = active_core_song)]
            $active_core_song,
            async move {
                window
                    .bind_song_model($active_model, active_core_song)
                    .await;
            }
        ));
    }};
}

#[macro_export]
macro_rules! dyn_event {
    ($lvl:ident, $($arg:tt)+) => {
        match $lvl {
            ::gtk::glib::LogLevel::Debug => ::tracing::debug!($($arg)+),
            ::gtk::glib::LogLevel::Message | ::gtk::glib::LogLevel::Info => ::tracing::info!($($arg)+),
            ::gtk::glib::LogLevel::Warning => ::tracing::warn!($($arg)+),
            ::gtk::glib::LogLevel::Error | ::gtk::glib::LogLevel::Critical  => ::tracing::error!($($arg)+),
        }
    };
}

#[macro_export]
macro_rules! close_on_error {
    ($widget:expr, $des:expr) => {{
        use gtk::prelude::WidgetExt;
        if let Some(root) = $widget.root() {
            if let Some(window) = root.downcast_ref::<$crate::ui::widgets::window::Window>() {
                window.close_on_error($des);
            }
        }
    }};
}

#[macro_export]
macro_rules! alert_dialog {
    ($widget:expr, $dialog:expr) => {{
        use gtk::prelude::WidgetExt;
        if let Some(root) = $widget.root() {
            if let Some(window) = root.downcast_ref::<$crate::ui::widgets::window::Window>() {
                window.alert_dialog($dialog);
            }
        }
    }};
}
