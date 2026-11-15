use std::{cell::Cell, collections::HashSet, rc::Rc};

use wl_proxy::{
    global_mapper::GlobalMapper,
    object::{Object, ObjectCoreApi, ObjectRcUtils},
    protocols::{
        ObjectInterface,
        wayland::{
            wl_fixes::WlFixes,
            wl_registry::{WlRegistry, WlRegistryHandler},
        },
    },
};

use crate::{
    allowed_globals::is_allowed, client_connection::refuse_unoffered_name,
    removal_ack_translator::RemovalAckTranslator, window_tagging,
};

/// Forwards the allowed globals under new names, so a client cannot address a hidden
/// global by its compositor name. A bind outside what the client saw ends the connection.
pub struct RegistryFilter {
    mapper: GlobalMapper,
    /// Indexed by the name that the client sees; index 0 is never a global.
    advertised: Vec<Option<Offer>>,
    hidden: HashSet<u32>,
    /// Compositor names of the `wl_fixes` globals that accept acknowledgments of removals.
    wl_fixes_v2_names: Vec<u32>,
    destroyed_by_client: bool,
    violation: Rc<Cell<bool>>,
}

#[derive(Clone, Copy)]
struct Offer {
    interface: ObjectInterface,
    version: u32,
    compositor_name: u32,
}

impl RegistryFilter {
    pub fn new(violation: Rc<Cell<bool>>) -> Self {
        Self {
            mapper: GlobalMapper::default(),
            advertised: vec![None],
            hidden: HashSet::new(),
            wl_fixes_v2_names: Vec::new(),
            destroyed_by_client: false,
            violation,
        }
    }

    /// Returns the compositor name of a global that this registry offered to the client.
    #[must_use]
    pub fn compositor_name_of(&self, client_name: u32) -> Option<u32> {
        let offer = self
            .advertised
            .get(client_name as usize)
            .copied()
            .flatten()?;
        Some(offer.compositor_name)
    }

    /// Stops the requests that the proxy makes on this registry. The compositor can still
    /// send events that it queued before it read the destruction.
    pub fn mark_destroyed_by_client(&mut self) {
        self.destroyed_by_client = true;
    }

    /// The client never sees the removal of a hidden global, so the proxy acknowledges it.
    /// Without that, the compositor keeps the global until the registry ends. wl-proxy
    /// reports no global of a protocol that this build does not parse.
    fn acknowledge_hidden_removal(&self, registry: &Rc<WlRegistry>, name: u32) {
        if self.destroyed_by_client {
            return;
        }
        let Some(&wl_fixes_name) = self.wl_fixes_v2_names.last() else {
            return;
        };
        let fixes = registry
            .state()
            .create_object::<WlFixes>(WlFixes::MSG__ACK_GLOBAL_REMOVE__SINCE);
        registry.send_bind(wl_fixes_name, fixes.clone());
        fixes.send_ack_global_remove(registry, name);
        fixes.send_destroy();
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
        if interface == ObjectInterface::WlFixes
            && version >= WlFixes::MSG__ACK_GLOBAL_REMOVE__SINCE
        {
            self.wl_fixes_v2_names.push(name);
        }
        if is_allowed(interface) {
            self.mapper
                .forward_global(registry, name, interface, version);
            self.advertised.push(Some(Offer {
                interface,
                version,
                compositor_name: name,
            }));
        } else {
            self.mapper.ignore_global(name);
            self.hidden.insert(name);
        }
    }

    // The name stays in `advertised`: libwayland accepts a bind until the global is destroyed.
    fn handle_global_remove(&mut self, registry: &Rc<WlRegistry>, name: u32) {
        self.wl_fixes_v2_names
            .retain(|&wl_fixes_name| wl_fixes_name != name);
        if self.hidden.remove(&name) {
            self.acknowledge_hidden_removal(registry, name);
        }
        self.mapper.forward_global_remove(registry, name);
    }

    fn handle_bind(&mut self, registry: &Rc<WlRegistry>, name: u32, id: Rc<dyn Object>) {
        let offered = self.advertised.get(name as usize).copied().flatten();
        let matches_offer = offered.is_some_and(|offer| {
            offer.interface == id.interface() && id.version() <= offer.version
        });
        if !matches_offer {
            tracing::warn!(
                "Client bound {} version {} to name {name}, which it was not offered",
                id.interface().name(),
                id.version(),
            );
            refuse_unoffered_name(registry.client(), &self.violation);
            return;
        }
        self.mapper.forward_bind(registry, name, &id);
        if let Some(fixes) = id.try_downcast::<WlFixes>() {
            fixes.set_handler(RemovalAckTranslator::new(self.violation.clone()));
        }
        window_tagging::tag_windows_of(&id);
    }
}
