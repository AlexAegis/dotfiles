use std::time::SystemTime;

use crate::{action::Action, list_state::ListState, queue_folder::QueueFolder};

/// Carries out what the list does after a key press. A failure becomes a
/// notice of the list. Returns true when the list quits.
pub fn carry_out(
    action: Action,
    queue: &QueueFolder,
    state: &mut ListState,
    now: SystemTime,
) -> bool {
    match action {
        Action::Quit => return true,
        Action::Answer { id, answer } => {
            if let Err(error) = queue.answer(&id, &answer) {
                state.report_refused_answer(&id, &error.to_string());
            }
        }
        Action::MutePings(time) => {
            if let Err(error) = queue.mute_pings_until(now + time) {
                state.report(&format!("{error:#}"));
            }
        }
        Action::UnmutePings => {
            if let Err(error) = queue.unmute_pings() {
                state.report(&format!("{error:#}"));
            }
        }
    }
    state.show_mute(queue.pings_muted_until());
    false
}
