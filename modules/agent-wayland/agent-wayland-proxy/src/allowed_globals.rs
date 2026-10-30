use wl_proxy::protocols::ObjectInterface;

/// Returns whether agent clients see a global of this interface.
///
/// The list covers windows, buffers, input, and frame timing. Clipboard, primary
/// selection, layer shell, input methods, idle state, and output control stay hidden.
#[must_use]
pub fn is_allowed(interface: ObjectInterface) -> bool {
    matches!(
        interface,
        ObjectInterface::WlCompositor
            | ObjectInterface::WlFixes
            | ObjectInterface::WlOutput
            | ObjectInterface::WlSeat
            | ObjectInterface::WlShm
            | ObjectInterface::WlSubcompositor
            | ObjectInterface::WpAlphaModifierV1
            | ObjectInterface::WpColorManagerV1
            | ObjectInterface::WpColorRepresentationManagerV1
            | ObjectInterface::WpCommitTimingManagerV1
            | ObjectInterface::WpContentTypeManagerV1
            | ObjectInterface::WpCursorShapeManagerV1
            | ObjectInterface::WpFifoManagerV1
            | ObjectInterface::WpFractionalScaleManagerV1
            | ObjectInterface::WpLinuxDrmSyncobjManagerV1
            | ObjectInterface::WpPointerWarpV1
            | ObjectInterface::WpPresentation
            | ObjectInterface::WpSinglePixelBufferManagerV1
            | ObjectInterface::WpTearingControlManagerV1
            | ObjectInterface::WpViewporter
            | ObjectInterface::XdgWmBase
            | ObjectInterface::ZwpLinuxDmabufV1
            | ObjectInterface::ZwpPointerConstraintsV1
            | ObjectInterface::ZwpPointerGesturesV1
            | ObjectInterface::ZwpRelativePointerManagerV1
            | ObjectInterface::ZxdgDecorationManagerV1
            | ObjectInterface::ZxdgOutputManagerV1
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_hide_the_clipboard() {
        assert!(!is_allowed(ObjectInterface::WlDataDeviceManager));
    }

    #[test]
    fn should_hide_the_security_context_manager() {
        assert!(!is_allowed(ObjectInterface::WpSecurityContextManagerV1));
    }

    #[test]
    fn should_show_the_window_manager_base() {
        assert!(is_allowed(ObjectInterface::XdgWmBase));
    }
}
