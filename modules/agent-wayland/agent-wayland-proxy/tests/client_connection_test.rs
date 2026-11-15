mod support;

use agent_wayland_proxy::client_connection::UnofferedName;
use support::{AgentClient, FakeCompositor};
use wayland_client::protocol::{
    wl_data_device_manager::WlDataDeviceManager, wl_fixes::WlFixes, wl_output::WlOutput,
    wl_shm::WlShm,
};

#[test]
fn should_offer_the_globals_for_windows_and_input_but_not_the_clipboard() {
    let compositor = FakeCompositor::start();
    let client = AgentClient::connect(&compositor);
    let interfaces: Vec<String> = client
        .offered()
        .into_iter()
        .map(|(_, interface)| interface)
        .collect();
    assert_eq!(
        interfaces,
        ["wl_compositor", "wl_shm", "wl_seat", "xdg_wm_base"]
    );
    client.disconnect().expect("a clean end");
}

#[test]
fn should_number_the_offered_globals_without_gaps_for_hidden_ones() {
    let compositor = FakeCompositor::start();
    let client = AgentClient::connect(&compositor);
    let names: Vec<u32> = client.offered().into_iter().map(|(name, _)| name).collect();
    assert_eq!(names, [1, 2, 3, 4]);
    client.disconnect().expect("a clean end");
}

#[test]
fn should_end_the_connection_of_a_client_that_binds_a_name_it_was_not_offered() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    let unoffered = u32::try_from(client.offered().len() + 1).expect("a small name");
    let _: WlDataDeviceManager = client.bind(unoffered, 3);
    assert!(client.roundtrip().is_err());
    let ending = client.disconnect().expect_err("a refused bind");
    assert!(ending.is::<UnofferedName>());
}

#[test]
fn should_end_the_connection_of_a_client_that_binds_an_offered_name_as_another_interface() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    let shm = client.offered_name_of("wl_shm");
    let _: WlDataDeviceManager = client.bind(shm, 1);
    assert!(client.roundtrip().is_err());
    let ending = client.disconnect().expect_err("a refused bind");
    assert!(ending.is::<UnofferedName>());
}

#[test]
fn should_end_the_connection_of_a_client_that_binds_a_newer_version_than_offered() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    let shm = client.offered_name_of("wl_shm");
    let _: WlShm = client.bind(shm, 2);
    assert!(client.roundtrip().is_err());
    let ending = client.disconnect().expect_err("a refused bind");
    assert!(ending.is::<UnofferedName>());
}

#[test]
fn should_offer_a_global_that_appears_later_under_the_next_name() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    compositor.add_clipboard_global();
    compositor.add_output_global();
    client.roundtrip().expect("a roundtrip");
    assert_eq!(client.offered().last(), Some(&(5, "wl_output".to_owned())));
    client.disconnect().expect("a clean end");
}

#[test]
fn should_forward_the_removal_of_an_offered_global() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    let clipboard = compositor.add_clipboard_global();
    let output = compositor.add_output_global();
    client.roundtrip().expect("a roundtrip");
    compositor.remove_global(clipboard);
    compositor.remove_global(output);
    client.roundtrip().expect("a roundtrip");
    let names: Vec<u32> = client.offered().into_iter().map(|(name, _)| name).collect();
    assert_eq!(names, [1, 2, 3, 4]);
    client.disconnect().expect("a clean end");
}

#[test]
fn should_forward_a_bind_of_a_removed_global_that_the_compositor_still_holds() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    let output = compositor.add_output_global();
    client.roundtrip().expect("a roundtrip");
    let name = client.offered_name_of("wl_output");
    compositor.disable_global(output);
    client.roundtrip().expect("a roundtrip");
    assert!(
        client
            .offered()
            .iter()
            .all(|(_, interface)| interface != "wl_output")
    );
    let _: WlOutput = client.bind(name, 4);
    client.roundtrip().expect("an accepted bind");
    assert_eq!(compositor.received().bound_interfaces, ["wl_output"]);
    client.disconnect().expect("a clean end");
}

#[test]
fn should_forward_the_acknowledgment_of_a_removal_under_the_compositor_name() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    compositor.add_fixes_global(2);
    let output = compositor.add_output_global();
    client.roundtrip().expect("a roundtrip");
    let fixes: WlFixes = client.bind(client.offered_name_of("wl_fixes"), 2);
    let name = client.offered_name_of("wl_output");
    let compositor_name = compositor.compositor_name_of("wl_output");
    compositor.disable_global(output);
    client.roundtrip().expect("a roundtrip");
    client.acknowledge_removal(&fixes, name);
    client.roundtrip().expect("a roundtrip");
    assert_eq!(
        compositor.received().acknowledged_removals,
        [compositor_name]
    );
    client.disconnect().expect("a clean end");
}

#[test]
fn should_end_the_connection_of_a_client_that_acknowledges_a_name_it_was_not_offered() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    compositor.add_fixes_global(2);
    client.roundtrip().expect("a roundtrip");
    let fixes: WlFixes = client.bind(client.offered_name_of("wl_fixes"), 2);
    let unoffered = u32::try_from(client.offered().len() + 1).expect("a small name");
    client.acknowledge_removal(&fixes, unoffered);
    assert!(client.roundtrip().is_err());
    let ending = client.disconnect().expect_err("a refused acknowledgment");
    assert!(ending.is::<UnofferedName>());
    assert_eq!(
        compositor.received().acknowledged_removals,
        Vec::<u32>::new()
    );
}

#[test]
fn should_acknowledge_the_removal_of_a_hidden_global_in_place_of_the_client() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    compositor.add_fixes_global(2);
    let clipboard = compositor.add_clipboard_global();
    client.roundtrip().expect("a roundtrip");
    let compositor_name = compositor.compositor_name_of("wl_data_device_manager");
    compositor.disable_global(clipboard);
    client.roundtrip().expect("a roundtrip");
    assert_eq!(
        compositor
            .received_after_acknowledged_removal()
            .acknowledged_removals,
        [compositor_name]
    );
    client.disconnect().expect("a clean end");
}

#[test]
fn should_not_acknowledge_a_hidden_removal_for_a_compositor_with_wl_fixes_version_1() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    compositor.add_fixes_global(1);
    let clipboard = compositor.add_clipboard_global();
    client.roundtrip().expect("a roundtrip");
    compositor.disable_global(clipboard);
    client.roundtrip().expect("a roundtrip");
    client.roundtrip().expect("a roundtrip");
    assert_eq!(
        compositor.received().acknowledged_removals,
        Vec::<u32>::new()
    );
    client.disconnect().expect("a clean end");
}

#[test]
fn should_not_acknowledge_a_hidden_removal_after_the_compositor_removed_wl_fixes() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    let fixes = compositor.add_fixes_global(2);
    let clipboard = compositor.add_clipboard_global();
    client.roundtrip().expect("a roundtrip");
    compositor.disable_global(fixes);
    compositor.disable_global(clipboard);
    client.roundtrip().expect("a roundtrip");
    client.roundtrip().expect("a roundtrip");
    assert_eq!(
        compositor.received().acknowledged_removals,
        Vec::<u32>::new()
    );
    client.disconnect().expect("a clean end");
}

#[test]
fn should_acknowledge_a_hidden_removal_through_the_remaining_wl_fixes_global() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    compositor.add_fixes_global(2);
    let second_fixes = compositor.add_fixes_global(2);
    let clipboard = compositor.add_clipboard_global();
    client.roundtrip().expect("a roundtrip");
    let compositor_name = compositor.compositor_name_of("wl_data_device_manager");
    compositor.disable_global(second_fixes);
    compositor.disable_global(clipboard);
    client.roundtrip().expect("a roundtrip");
    assert_eq!(
        compositor
            .received_after_acknowledged_removal()
            .acknowledged_removals,
        [compositor_name]
    );
    client.disconnect().expect("a clean end");
}

#[test]
fn should_not_acknowledge_a_hidden_removal_on_a_registry_that_the_client_destroyed() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    compositor.add_fixes_global(2);
    let clipboard = compositor.add_clipboard_global();
    client.roundtrip().expect("a roundtrip");
    let fixes: WlFixes = client.bind(client.offered_name_of("wl_fixes"), 2);
    let destroyed = client.get_another_registry();
    client.roundtrip().expect("a roundtrip");
    fixes.destroy_registry(&destroyed);
    client.roundtrip().expect("a roundtrip");
    let compositor_name = compositor.compositor_name_of("wl_data_device_manager");
    compositor.disable_global(clipboard);
    client.roundtrip().expect("a roundtrip");
    client.roundtrip().expect("a roundtrip");
    let received = compositor.received();
    assert_eq!(received.destroyed_registries, 1);
    assert_eq!(received.acknowledged_removals, [compositor_name]);
    assert_eq!(
        received.acknowledged_removals_on_destroyed_registries,
        Vec::<u32>::new()
    );
    client.disconnect().expect("a clean end");
}

#[test]
fn should_prefix_the_title_and_app_id_that_a_client_sets() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    let window = client.open_window();
    window.set_title("game".to_owned());
    window.set_app_id("bevy".to_owned());
    client.roundtrip().expect("a roundtrip");
    let received = compositor.received();
    assert_eq!(
        received.titles.last().map(String::as_str),
        Some("[agent] game")
    );
    assert_eq!(
        received.app_ids.last().map(String::as_str),
        Some("agent.bevy")
    );
    client.disconnect().expect("a clean end");
}

#[test]
fn should_mark_a_window_whose_client_sets_no_title_or_app_id() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    let _window = client.open_window();
    client.roundtrip().expect("a roundtrip");
    let received = compositor.received();
    assert_eq!(received.titles, ["[agent]"]);
    assert_eq!(received.app_ids, ["agent"]);
    client.disconnect().expect("a clean end");
}

#[test]
fn should_not_forward_the_requests_that_follow_a_refused_bind() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    let window = client.open_window();
    client.roundtrip().expect("a roundtrip");
    let unoffered = u32::try_from(client.offered().len() + 1).expect("a small name");
    let _: WlDataDeviceManager = client.bind(unoffered, 3);
    window.set_title("late".to_owned());
    assert!(client.roundtrip().is_err());
    client.disconnect().expect_err("a refused bind");
    assert_eq!(compositor.received_after_disconnect().titles, ["[agent]"]);
}
