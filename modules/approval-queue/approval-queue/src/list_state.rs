use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{
    action::Action, answer::Answer, key::Key, listed_entry::ListedEntry,
    message_draft::MessageDraft,
};

/// The list takes a key that answers, opens a message or quits only after
/// this time without a key press, pasted text or a new selection by the list.
/// A key that the owner pressed for an entry that left then does not answer
/// the next one. Of text that someone types into the list without such a
/// pause, only the first key can act.
const QUIET_TIME: Duration = Duration::from_secs(1);
const SCROLL_STEP: u16 = 10;
const NOT_QUIET: &str = "Too early. Press the key after 1 second without a key.";
const ANSWERED: &str = "The prompt has its answer";
const NO_PROMPT: &str = "No prompt waits";
const DRAFT_ENTRY_LEFT: &str = "The prompt left the queue. Esc closes the message.";
const PASTE_IGNORED: &str = "Pasted text is no answer. The m key opens a message.";

/// What the list shows: the waiting entries, the selected one and the text
/// that the owner types.
#[derive(Debug, Default)]
pub struct ListState {
    entries: Vec<ListedEntry>,
    selected: Option<String>,
    quiet_from: Option<Instant>,
    answered: HashMap<String, Answer>,
    draft: Option<MessageDraft>,
    scroll: u16,
    scroll_limit: u16,
    notice: Option<String>,
}

impl ListState {
    /// Takes the current entries of the queue. The selection stays on its
    /// entry. When that entry left, the oldest entry gets the selection. A
    /// message stays open when its entry left, so the keys that the owner
    /// still types are text and no answers.
    pub fn refresh(&mut self, entries: Vec<ListedEntry>, now: Instant) {
        self.entries = entries;
        self.answered.retain(|id, _| contains(&self.entries, id));
        if self
            .selected
            .as_deref()
            .is_some_and(|id| contains(&self.entries, id))
        {
            return;
        }
        self.selected = self.entries.first().map(|listed| listed.id.clone());
        self.scroll = 0;
        if self.selected.is_some() {
            self.start_quiet_time(now);
        }
    }

    pub fn press(&mut self, key: Key, now: Instant) -> Option<Action> {
        if key == Key::Interrupt {
            return Some(Action::Quit);
        }
        self.notice = None;
        if self.draft.is_some() {
            return self.press_in_message(key, now);
        }
        let is_quiet = self.is_quiet(now);
        self.start_quiet_time(now);
        match key {
            Key::Up | Key::Character('k') => self.select_neighbor(false),
            Key::Down | Key::Character('j') => self.select_neighbor(true),
            Key::PageUp => self.scroll = self.scroll.saturating_sub(SCROLL_STEP),
            Key::PageDown => {
                self.scroll = self
                    .scroll
                    .saturating_add(SCROLL_STEP)
                    .min(self.scroll_limit);
            }
            Key::Character('a') => return self.answer(Answer::Allow, is_quiet),
            Key::Character('d') => {
                return self.answer(Answer::Deny { message: None }, is_quiet);
            }
            Key::Character('m') => {
                self.draft = self.target(is_quiet).map(|id| MessageDraft {
                    id,
                    text: String::new(),
                });
            }
            Key::Character('q') => {
                if is_quiet {
                    return Some(Action::Quit);
                }
                self.notice = Some(NOT_QUIET.to_owned());
            }
            _ => {}
        }
        None
    }

    /// Takes text that the terminal marks as pasted: text of the message, and
    /// never keys of the list. A line break becomes a space.
    pub fn paste(&mut self, text: &str, now: Instant) {
        if let Some(draft) = &mut self.draft {
            for character in text.chars() {
                draft.push(if character.is_control() {
                    ' '
                } else {
                    character
                });
            }
        } else {
            self.notice = Some(PASTE_IGNORED.to_owned());
            self.start_quiet_time(now);
        }
    }

    /// Takes back an answer that the queue refused, so the owner can answer
    /// again.
    pub fn report_refused_answer(&mut self, id: &str, reason: &str) {
        self.answered.remove(id);
        self.notice = Some(reason.to_owned());
    }

    /// Takes the count of the lines that the selected entry has below its
    /// pane, which is how far the pane scrolls.
    pub fn limit_scroll(&mut self, limit: u16) {
        self.scroll_limit = limit;
        self.scroll = self.scroll.min(limit);
    }

    #[must_use]
    pub fn entries(&self) -> &[ListedEntry] {
        &self.entries
    }

    #[must_use]
    pub fn selected_index(&self) -> Option<usize> {
        let selected = self.selected.as_deref()?;
        self.entries.iter().position(|listed| listed.id == selected)
    }

    #[must_use]
    pub fn selected(&self) -> Option<&ListedEntry> {
        self.selected_index().map(|index| &self.entries[index])
    }

    /// The answer that the owner gave to the entry and that its service
    /// process did not take yet.
    #[must_use]
    pub fn answer_of(&self, id: &str) -> Option<&Answer> {
        self.answered.get(id)
    }

    /// The message of a denial while the owner types it.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.draft.as_ref().map(|draft| draft.text.as_str())
    }

    #[must_use]
    pub fn notice(&self) -> Option<&str> {
        if self.draft_entry_left() {
            Some(DRAFT_ENTRY_LEFT)
        } else {
            self.notice.as_deref()
        }
    }

    #[must_use]
    pub fn scroll(&self) -> u16 {
        self.scroll
    }

    fn press_in_message(&mut self, key: Key, now: Instant) -> Option<Action> {
        match key {
            Key::Character(character) => self.draft.as_mut()?.push(character),
            Key::Backspace => {
                self.draft.as_mut()?.text.pop();
            }
            Key::Escape => self.close_draft(now),
            Key::Enter => {
                if self.draft_entry_left() {
                    self.close_draft(now);
                } else {
                    let draft = self.draft.take()?;
                    self.start_quiet_time(now);
                    let text = draft.text.trim();
                    let message = (!text.is_empty()).then(|| text.to_owned());
                    return Some(self.record(draft.id, Answer::Deny { message }));
                }
            }
            _ => {}
        }
        None
    }

    fn close_draft(&mut self, now: Instant) {
        self.draft = None;
        self.start_quiet_time(now);
    }

    fn draft_entry_left(&self) -> bool {
        self.draft
            .as_ref()
            .is_some_and(|draft| !contains(&self.entries, &draft.id))
    }

    fn select_neighbor(&mut self, later: bool) {
        let Some(index) = self.selected_index() else {
            return;
        };
        let neighbor = if later {
            (index + 1).min(self.entries.len() - 1)
        } else {
            index.saturating_sub(1)
        };
        self.selected = Some(self.entries[neighbor].id.clone());
        self.scroll = 0;
    }

    fn answer(&mut self, answer: Answer, is_quiet: bool) -> Option<Action> {
        let id = self.target(is_quiet)?;
        Some(self.record(id, answer))
    }

    fn start_quiet_time(&mut self, now: Instant) {
        self.quiet_from = Some(now + QUIET_TIME);
    }

    fn is_quiet(&self, now: Instant) -> bool {
        self.quiet_from.is_none_or(|from| now >= from)
    }

    /// The selected entry when a key can act on it: it has no answer, and the
    /// key came after the quiet time. Else sets the notice.
    fn target(&mut self, is_quiet: bool) -> Option<String> {
        let selected = self.selected().map(|listed| listed.id.clone());
        let refusal = match &selected {
            None => NO_PROMPT,
            Some(id) if self.answered.contains_key(id) => ANSWERED,
            Some(_) if !is_quiet => NOT_QUIET,
            Some(_) => return selected,
        };
        self.notice = Some(refusal.to_owned());
        None
    }

    fn record(&mut self, id: String, answer: Answer) -> Action {
        self.answered.insert(id.clone(), answer.clone());
        Action::Answer { id, answer }
    }
}

fn contains(entries: &[ListedEntry], id: &str) -> bool {
    entries.iter().any(|listed| listed.id == id)
}
