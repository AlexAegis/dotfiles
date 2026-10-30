use std::{cell::Cell, rc::Rc};

use wl_proxy::{
    global_mapper::GlobalMapper,
    object::{Object, ObjectCoreApi},
    protocols::{
        ObjectInterface,
        wayland::wl_registry::{WlRegistry, WlRegistryHandler},
    },
};

use crate::{allowed_globals::is_allowed, window_tagging};

/// Forwards the allowed globals under new names, so a client cannot address a hidden
/// global by its compositor name. A bind outside what the client saw ends the connection.
pub struct RegistryFilter {
    mapper: GlobalMapper,
    /// Indexed by the name that the client sees; index 0 is never a global.
    advertised: Vec<Option<(ObjectInterface, u32)>>,
    violation: Rc<Cell<bool>>,
}

impl RegistryFilter {
    pub fn new(violation: Rc<Cell<bool>>) -> Self {
        Self {
            mapper: GlobalMapper::default(),
            advertised: vec![None],
            violation,
        }
    }
}

impl WlRegistryHandler for RegistryFilter {
    fn handle_global(
        &mut self,
        registry: &Rc<WlRegistry>,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
        if is_allowed(interface) {
            self.mapper
                .forward_global(registry, name, interface, version);
            self.advertised.push(Some((interface, version)));
        } else {
            self.mapper.ignore_global(name);
        }
    }

    fn handle_global_remove(&mut self, registry: &Rc<WlRegistry>, name: u32) {
        self.mapper.forward_global_remove(registry, name);
    }

    fn handle_bind(&mut self, registry: &Rc<WlRegistry>, name: u32, id: Rc<dyn Object>) {
        let offered = self.advertised.get(name as usize).copied().flatten();
        let matches_offer = offered.is_some_and(|(interface, version)| {
            interface == id.interface() && id.version() <= version
        });
        if !matches_offer {
            tracing::warn!(
                "Client bound {} version {} to name {name}, which it was not offered",
                id.interface().name(),
                id.version(),
            );
            self.violation.set(true);
            if let Some(client) = registry.client() {
                client.set_suspended(true);
            }
            return;
        }
        self.mapper.forward_bind(registry, name, &id);
        window_tagging::tag_windows_of(&id);
    }
}
