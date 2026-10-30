use std::rc::Rc;

use wl_proxy::{
    object::{Object, ObjectRcUtils},
    protocols::{
        wayland::wl_surface::WlSurface,
        xdg_shell::{
            xdg_surface::{XdgSurface, XdgSurfaceHandler},
            xdg_toplevel::{XdgToplevel, XdgToplevelHandler},
            xdg_wm_base::{XdgWmBase, XdgWmBaseHandler},
        },
    },
};

/// App id of every agent window, and the prefix of the app ids that clients set.
/// The KWin window rule of the module matches it.
const APP_ID: &str = "agent";
const TITLE_PREFIX: &str = "[agent]";
const MAX_CLIENT_TEXT_BYTES: usize = 512;

/// Sets the handlers that tag the windows of a global that a client bound.
pub fn tag_windows_of(object: &Rc<dyn Object>) {
    if let Some(wm_base) = object.try_downcast::<XdgWmBase>() {
        wm_base.set_handler(WmBaseTagger);
    }
}

fn tagged_app_id(app_id: &str) -> String {
    if app_id.is_empty() {
        APP_ID.to_owned()
    } else {
        format!(
            "{APP_ID}.{}",
            &app_id[..app_id.floor_char_boundary(MAX_CLIENT_TEXT_BYTES)]
        )
    }
}

fn tagged_title(title: &str) -> String {
    if title.is_empty() {
        TITLE_PREFIX.to_owned()
    } else {
        format!(
            "{TITLE_PREFIX} {}",
            &title[..title.floor_char_boundary(MAX_CLIENT_TEXT_BYTES)]
        )
    }
}

struct WmBaseTagger;

impl XdgWmBaseHandler for WmBaseTagger {
    fn handle_get_xdg_surface(
        &mut self,
        wm_base: &Rc<XdgWmBase>,
        id: &Rc<XdgSurface>,
        surface: &Rc<WlSurface>,
    ) {
        wm_base.send_get_xdg_surface(id, surface);
        id.set_handler(XdgSurfaceTagger);
    }
}

struct XdgSurfaceTagger;

impl XdgSurfaceHandler for XdgSurfaceTagger {
    fn handle_get_toplevel(&mut self, xdg_surface: &Rc<XdgSurface>, id: &Rc<XdgToplevel>) {
        xdg_surface.send_get_toplevel(id);
        id.send_set_app_id(APP_ID);
        id.send_set_title(TITLE_PREFIX);
        id.set_handler(ToplevelTagger);
    }
}

struct ToplevelTagger;

impl XdgToplevelHandler for ToplevelTagger {
    fn handle_set_title(&mut self, toplevel: &Rc<XdgToplevel>, title: &str) {
        toplevel.send_set_title(&tagged_title(title));
    }

    fn handle_set_app_id(&mut self, toplevel: &Rc<XdgToplevel>, app_id: &str) {
        toplevel.send_set_app_id(&tagged_app_id(app_id));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_prefix_a_title() {
        assert_eq!(tagged_title("Bevy App"), "[agent] Bevy App");
    }

    #[test]
    fn should_mark_an_empty_title() {
        assert_eq!(tagged_title(""), "[agent]");
    }

    #[test]
    fn should_cut_a_long_title_at_a_character_boundary() {
        let title = "\u{e9}".repeat(MAX_CLIENT_TEXT_BYTES);
        let tagged = tagged_title(&title);
        assert!(tagged.starts_with("[agent] "));
        assert_eq!(tagged.len(), "[agent] ".len() + MAX_CLIENT_TEXT_BYTES);
    }

    #[test]
    fn should_nest_an_app_id_under_agent() {
        assert_eq!(tagged_app_id("org.bevy.example"), "agent.org.bevy.example");
    }

    #[test]
    fn should_give_an_empty_app_id_the_plain_agent_app_id() {
        assert_eq!(tagged_app_id(""), "agent");
    }
}
