use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use approval_queue::ping_watch::{CADENCE, PingWatch};

fn seconds(count: u64) -> Duration {
    Duration::from_secs(count)
}

fn ids(names: &[&str]) -> Vec<String> {
    names.iter().map(ToString::to_string).collect()
}

fn recording_watch() -> (PingWatch, Arc<Mutex<Vec<Vec<String>>>>) {
    let pings = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&pings);
    let watch = PingWatch::new(CADENCE, move |ping| {
        seen.lock().unwrap().push(ping.keys.clone());
    });
    (watch, pings)
}

#[test]
fn should_ping_one_time_for_a_prompt_that_stays() {
    let start = Instant::now();
    let (mut watch, pings) = recording_watch();
    watch.refresh(ids(&[]), start, false);
    watch.refresh(ids(&["a"]), start + seconds(1), false);
    watch.refresh(ids(&["a"]), start + seconds(2), false);
    watch.refresh(ids(&["a"]), start + seconds(500), false);
    assert_eq!(*pings.lock().unwrap(), vec![ids(&["a"])]);
}

#[test]
fn should_collect_the_prompts_of_the_quiet_time_into_one_ping() {
    let start = Instant::now();
    let (mut watch, pings) = recording_watch();
    watch.refresh(ids(&["a"]), start, false);
    watch.refresh(ids(&["a", "b"]), start + seconds(5), false);
    watch.refresh(ids(&["a", "b", "c"]), start + seconds(6), false);
    watch.refresh(ids(&["a", "b", "c"]), start + seconds(124), false);
    assert_eq!(pings.lock().unwrap().len(), 1);
    watch.refresh(ids(&["a", "b", "c"]), start + seconds(125), false);
    assert_eq!(*pings.lock().unwrap(), vec![ids(&["a"]), ids(&["b", "c"])]);
}

#[test]
fn should_ping_when_a_batch_waits_after_the_quiet_time() {
    let start = Instant::now();
    let (mut watch, pings) = recording_watch();
    watch.refresh(ids(&["a"]), start, false);
    watch.refresh(ids(&["a", "b", "c", "d", "e"]), start + seconds(5), false);
    watch.refresh(ids(&["a", "b", "c", "d", "e"]), start + seconds(29), false);
    assert_eq!(pings.lock().unwrap().len(), 1);
    watch.refresh(ids(&["a", "b", "c", "d", "e"]), start + seconds(30), false);
    assert_eq!(pings.lock().unwrap()[1], ids(&["b", "c", "d", "e"]));
}

#[test]
fn should_not_ping_for_a_prompt_that_left_the_queue() {
    let start = Instant::now();
    let (mut watch, pings) = recording_watch();
    watch.refresh(ids(&["a"]), start, false);
    watch.refresh(ids(&["a", "b"]), start + seconds(5), false);
    watch.refresh(ids(&["a"]), start + seconds(6), false);
    watch.refresh(ids(&[]), start + seconds(500), false);
    assert_eq!(*pings.lock().unwrap(), vec![ids(&["a"])]);
}

#[test]
fn should_ping_at_once_for_a_prompt_after_the_quiet_time_while_a_pinged_prompt_still_waits() {
    let start = Instant::now();
    let (mut watch, pings) = recording_watch();
    watch.refresh(ids(&["a"]), start, false);
    watch.refresh(ids(&["a", "b"]), start + seconds(40), false);
    assert_eq!(*pings.lock().unwrap(), vec![ids(&["a"]), ids(&["b"])]);
}

#[test]
fn should_not_count_a_pinged_prompt_for_the_batch() {
    let start = Instant::now();
    let (mut watch, pings) = recording_watch();
    watch.refresh(ids(&["a"]), start, false);
    watch.refresh(ids(&["a", "b", "c", "d"]), start + seconds(5), false);
    watch.refresh(ids(&["a", "b", "c", "d"]), start + seconds(124), false);
    assert_eq!(pings.lock().unwrap().len(), 1);
    watch.refresh(ids(&["a", "b", "c", "d"]), start + seconds(125), false);
    assert_eq!(pings.lock().unwrap()[1], ids(&["b", "c", "d"]));
}

#[test]
fn should_ping_for_the_prompts_of_a_mute_when_it_ends() {
    let start = Instant::now();
    let (mut watch, pings) = recording_watch();
    watch.refresh(ids(&["a"]), start, true);
    watch.refresh(ids(&["a", "b"]), start + seconds(300), true);
    assert_eq!(pings.lock().unwrap().len(), 0);
    watch.refresh(ids(&["a", "b"]), start + seconds(600), false);
    assert_eq!(*pings.lock().unwrap(), vec![ids(&["a", "b"])]);
}

#[test]
fn should_not_ping_after_a_mute_for_a_prompt_that_left_in_it() {
    let start = Instant::now();
    let (mut watch, pings) = recording_watch();
    watch.refresh(ids(&["a"]), start, true);
    watch.refresh(ids(&[]), start + seconds(5), true);
    watch.refresh(ids(&[]), start + seconds(600), false);
    assert_eq!(pings.lock().unwrap().len(), 0);
}

#[test]
fn should_not_ping_again_after_a_mute_for_a_prompt_that_had_its_ping() {
    let start = Instant::now();
    let (mut watch, pings) = recording_watch();
    watch.refresh(ids(&["a"]), start, false);
    watch.refresh(ids(&["a"]), start + seconds(5), true);
    watch.refresh(ids(&["a"]), start + seconds(600), false);
    assert_eq!(pings.lock().unwrap().len(), 1);
}
