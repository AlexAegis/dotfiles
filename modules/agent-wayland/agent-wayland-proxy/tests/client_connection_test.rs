mod support;

use agent_wayland_proxy::client_connection::UnofferedBind;
use support::{AgentClient, FakeCompositor};
use wayland_client::protocol::{wl_data_device_manager::WlDataDeviceManager, wl_shm::WlShm};

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
    assert!(ending.is::<UnofferedBind>());
}

#[test]
fn should_end_the_connection_of_a_client_that_binds_an_offered_name_as_another_interface() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    let shm = client.offered_name_of("wl_shm");
    let _: WlDataDeviceManager = client.bind(shm, 1);
    assert!(client.roundtrip().is_err());
    let ending = client.disconnect().expect_err("a refused bind");
    assert!(ending.is::<UnofferedBind>());
}

#[test]
fn should_end_the_connection_of_a_client_that_binds_a_newer_version_than_offered() {
    let compositor = FakeCompositor::start();
    let mut client = AgentClient::connect(&compositor);
    let shm = client.offered_name_of("wl_shm");
    let _: WlShm = client.bind(shm, 2);
    assert!(client.roundtrip().is_err());
    let ending = client.disconnect().expect_err("a refused bind");
    assert!(ending.is::<UnofferedBind>());
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
