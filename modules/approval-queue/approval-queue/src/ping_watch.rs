use std::time::{Duration, Instant};

use ping_throttle::{Cadence, Notifier, Ping};

/// The cadence of the pings for the owner. A prompt that arrives while no
/// prompt waits for a ping pings at once, when the last ping is 30 seconds
/// old. After a ping, no ping comes for 30 seconds. Then 4 prompts that wait
/// for a ping get one, or fewer when the oldest of them waited 2 minutes.
pub const CADENCE: Cadence = Cadence {
    quiet_time: Duration::from_secs(30),
    batch_size: 4,
    longest_wait: Duration::from_secs(120),
};

/// Follows the waiting prompts and pings its subscriber at a cadence, one
/// ping for all prompts that wait at that time.
#[derive(Debug)]
pub struct PingWatch {
    notifier: Notifier<String>,
    known: Vec<String>,
}

impl PingWatch {
    #[must_use]
    pub fn new(cadence: Cadence, subscriber: impl FnMut(&Ping<String>) + Send + 'static) -> Self {
        let mut notifier = Notifier::new(cadence);
        notifier.subscribe(subscriber);
        Self {
            notifier,
            known: Vec::new(),
        }
    }

    /// Takes the IDs of the prompts that wait at `now`, oldest first, and
    /// pings when the cadence allows it. A prompt that left the queue counts
    /// for no ping. While the pings are muted, no ping comes: the prompts
    /// keep their wait and ping when the mute ends.
    pub fn refresh(&mut self, waiting: Vec<String>, now: Instant, is_muted: bool) {
        for id in &self.known {
            if !waiting.contains(id) {
                self.notifier.leave(id);
            }
        }
        for id in &waiting {
            if !self.known.contains(id) {
                self.notifier.arrive(id.clone(), now);
            }
        }
        self.known = waiting;
        if !is_muted {
            self.notifier.poll(now);
        }
    }
}
