use std::{cell::Cell, rc::Rc};

use wl_proxy::{
    object::{ObjectCoreApi, ObjectUtils},
    protocols::wayland::{
        wl_fixes::{WlFixes, WlFixesHandler},
        wl_registry::WlRegistry,
    },
};

use crate::{client_connection::refuse_unoffered_name, registry_filter::RegistryFilter};

/// Forwards the acknowledgment of a removed global under the compositor name of that global.
/// Untranslated, the compositor would read the name that the client saw as one of its own.
/// An acknowledgment of a name outside the offer ends the connection.
pub struct RemovalAckTranslator {
    violation: Rc<Cell<bool>>,
}

impl RemovalAckTranslator {
    pub fn new(violation: Rc<Cell<bool>>) -> Self {
        Self { violation }
    }
}

impl WlFixesHandler for RemovalAckTranslator {
    fn handle_destroy_registry(&mut self, fixes: &Rc<WlFixes>, registry: &Rc<WlRegistry>) {
        if let Ok(mut filter) = registry.try_get_handler_mut::<RegistryFilter>() {
            filter.mark_destroyed_by_client();
        }
        fixes.send_destroy_registry(registry);
    }

    fn handle_ack_global_remove(
        &mut self,
        fixes: &Rc<WlFixes>,
        registry: &Rc<WlRegistry>,
        name: u32,
    ) {
        let compositor_name = match registry.try_get_handler_ref::<RegistryFilter>() {
            Ok(filter) => filter.compositor_name_of(name),
            Err(error) => {
                tracing::warn!("Registry of an acknowledged removal has no filter: {error}");
                refuse_unoffered_name(fixes.client(), &self.violation);
                return;
            }
        };
        let Some(compositor_name) = compositor_name else {
            tracing::warn!(
                "Client acknowledged the removal of name {name}, which it was not offered"
            );
            refuse_unoffered_name(fixes.client(), &self.violation);
            return;
        };
        fixes.send_ack_global_remove(registry, compositor_name);
    }
}
