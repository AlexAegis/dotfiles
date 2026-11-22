use std::{
    collections::HashMap,
    time::{Duration, Instant, SystemTime},
};

use crate::{
    action::Action, answer::Answer, key::Key, listed_entry::ListedEntry,
    message_draft::MessageDraft, question::questions_of, question_draft::QuestionDraft,
};

/// The list takes a key that answers, opens a message, mutes or unmutes the
/// pings or quits only after this time without a key press, pasted text, a
/// new selection by the list or a new focus of its window.
/// A key that the owner pressed for an entry that left then does not answer
/// the next one. Of text that someone types into the list without such a
/// pause, only the first key can act.
const QUIET_TIME: Duration = Duration::from_secs(1);
const SCROLL_STEP: u16 = 10;
const SHORT_MUTE: Duration = Duration::from_mins(10);
const LONG_MUTE: Duration = Duration::from_hours(1);
const NOT_QUIET: &str = "Too early. Press the key after 1 second without a key.";
const ANSWERED: &str = "The prompt has its answer";
const NO_PROMPT: &str = "No prompt waits";
const DRAFT_ENTRY_LEFT: &str = "The prompt left the queue. Esc closes the message.";
const QUESTION_ENTRY_LEFT: &str = "The prompt left the queue. Esc closes the text.";
const QUESTION_KEYS: &str = "A question takes an answer: a number picks an option, t opens a text.";
const NO_SUCH_OPTION: &str = "The question has no option with this number";
const NOTHING_PICKED: &str = "Pick an option with its number first";
const PASTE_IGNORED: &str = "Pasted text is no answer. The m key opens a message.";
const PASTE_IGNORED_BY_QUESTION: &str = "Pasted text is no answer. The t key opens a text.";

/// What the list shows: the waiting entries, the selected one and the text
/// that the owner types.
#[derive(Debug, Default)]
pub struct ListState {
    entries: Vec<ListedEntry>,
    selected: Option<String>,
    quiet_from: Option<Instant>,
    /// The end of the quiet time after the window got the focus. An open
    /// message takes no key before it.
    focus_quiet_from: Option<Instant>,
    answered: HashMap<String, Answer>,
    draft: Option<MessageDraft>,
    /// The answers so far to the questions of each entry, by its ID.
    questions: HashMap<String, QuestionDraft>,
    scroll: u16,
    scroll_limit: u16,
    notice: Option<String>,
    pings_muted_until: Option<SystemTime>,
}

impl ListState {
    /// Takes the current entries of the queue. The selection stays on its
    /// entry. When that entry left, the oldest entry gets the selection. A
    /// message or the text of an own answer stays open when its entry left,
    /// so the keys that the owner still types are text and no answers.
    pub fn refresh(&mut self, entries: Vec<ListedEntry>, now: Instant) {
        self.entries = entries;
        self.answered.retain(|id, _| contains(&self.entries, id));
        self.questions
            .retain(|id, draft| draft.text().is_some() || contains(&self.entries, id));
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
            if self.refuses_after_new_focus(now) {
                return None;
            }
            return self.press_in_message(&key, now);
        }
        if self.question_text().is_some() {
            if self.refuses_after_new_focus(now) {
                return None;
            }
            return self.press_in_question_text(&key, now);
        }
        let is_quiet = self.is_quiet(now);
        self.start_quiet_time(now);
        if let QuestionKey::Taken(action) = self.press_for_question(&key, is_quiet) {
            return action;
        }
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
            Key::Character('a') => return self.answer(Answer::ALLOW, is_quiet),
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
            Key::Character(character @ ('s' | 'S' | 'u')) => {
                if is_quiet {
                    return Some(match character {
                        's' => Action::MutePings(SHORT_MUTE),
                        'S' => Action::MutePings(LONG_MUTE),
                        _ => Action::UnmutePings,
                    });
                }
                self.notice = Some(NOT_QUIET.to_owned());
            }
            Key::Character(' ') => self.name_unused_key("space"),
            Key::Character(character) => self.name_unused_key(&character.to_string()),
            Key::Enter => self.name_unused_key("Enter"),
            Key::Escape => self.name_unused_key("Esc"),
            Key::Backspace => self.name_unused_key("Backspace"),
            Key::Other(name) => self.name_unused_key(&name),
            Key::Interrupt => {}
        }
        None
    }

    /// The window of the list got the focus.
    pub fn gain_focus(&mut self, now: Instant) {
        self.start_quiet_time(now);
        self.focus_quiet_from = Some(now + QUIET_TIME);
    }

    /// Shows a failure that is no refused answer.
    pub fn report(&mut self, failure: &str) {
        self.notice = Some(failure.to_owned());
    }

    /// True while the quiet time after a new focus lasts. An open message
    /// or own answer then takes no key and no pasted text, and the quiet time starts again.
    fn refuses_after_new_focus(&mut self, now: Instant) -> bool {
        let refuses = self.focus_quiet_from.is_some_and(|from| now < from);
        if refuses {
            self.focus_quiet_from = Some(now + QUIET_TIME);
            self.notice = Some(NOT_QUIET.to_owned());
        }
        refuses
    }

    fn name_unused_key(&mut self, name: &str) {
        self.notice = Some(format!("Key {name} has no function"));
    }

    /// Takes text that the terminal marks as pasted: text of the message, and
    /// never keys of the list. A line break becomes a space.
    pub fn paste(&mut self, text: &str, now: Instant) {
        let has_text = self.draft.is_some() || self.question_text().is_some();
        if has_text && self.refuses_after_new_focus(now) {
            return;
        }
        let pasted = text.chars().map(|character| {
            if character.is_control() {
                ' '
            } else {
                character
            }
        });
        if let Some(draft) = &mut self.draft {
            self.notice = None;
            pasted.for_each(|character| draft.push(character));
        } else if let Some(questions) = self
            .questions
            .values_mut()
            .find(|draft| draft.text().is_some())
        {
            self.notice = None;
            pasted.for_each(|character| questions.push(character));
        } else {
            let notice = if self.question_draft().is_some() {
                PASTE_IGNORED_BY_QUESTION
            } else {
                PASTE_IGNORED
            };
            self.notice = Some(notice.to_owned());
            self.start_quiet_time(now);
        }
    }

    /// Takes back an answer that the queue refused, so the owner can answer
    /// again.
    pub fn report_refused_answer(&mut self, id: &str, reason: &str) {
        self.answered.remove(id);
        self.notice = Some(reason.to_owned());
    }

    /// Takes the time until which the pings are muted, for the title.
    pub fn show_mute(&mut self, until: Option<SystemTime>) {
        self.pings_muted_until = until;
    }

    #[must_use]
    pub fn pings_muted_until(&self) -> Option<SystemTime> {
        self.pings_muted_until
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

    /// The questions of the selected entry with the answers so far. `None`
    /// for an entry without questions that the list can answer.
    #[must_use]
    pub fn question_draft(&self) -> Option<QuestionDraft> {
        let listed = self.selected()?;
        match self.questions.get(&listed.id) {
            Some(draft) => Some(draft.clone()),
            None => Some(QuestionDraft::new(
                listed.id.clone(),
                questions_of(&listed.entry)?,
            )),
        }
    }

    /// The text of an own answer to a question while the owner types it.
    #[must_use]
    pub fn question_text(&self) -> Option<&str> {
        self.questions.values().find_map(QuestionDraft::text)
    }

    /// The message of a denial while the owner types it.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.draft.as_ref().map(|draft| draft.text.as_str())
    }

    #[must_use]
    pub fn notice(&self) -> Option<&str> {
        if self.notice.as_deref() == Some(NOT_QUIET) {
            self.notice.as_deref()
        } else if self.draft_entry_left() {
            Some(DRAFT_ENTRY_LEFT)
        } else if self.draft.is_none() && self.question_text_entry_left() {
            Some(QUESTION_ENTRY_LEFT)
        } else {
            self.notice.as_deref()
        }
    }

    #[must_use]
    pub fn scroll(&self) -> u16 {
        self.scroll
    }

    fn press_in_message(&mut self, key: &Key, now: Instant) -> Option<Action> {
        match key {
            Key::Character(character) => self.draft.as_mut()?.push(*character),
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

    /// Takes a key for the questions of the selected entry. `NotOne` for a
    /// key that is none of theirs, and for an entry without questions.
    fn press_for_question(&mut self, key: &Key, is_quiet: bool) -> QuestionKey {
        let Some(mut draft) = self.question_draft() else {
            return QuestionKey::NotOne;
        };
        let number = match key {
            Key::Character(character @ '1'..='9') => character.to_digit(10),
            Key::Enter if !draft.current().multi_select => return QuestionKey::NotOne,
            Key::Character('a' | 't') | Key::Enter | Key::Escape => None,
            _ => return QuestionKey::NotOne,
        };
        if *key == Key::Character('a') {
            self.notice = Some(QUESTION_KEYS.to_owned());
            return QuestionKey::Taken(None);
        }
        if *key == Key::Escape {
            if is_quiet {
                self.questions.remove(draft.id());
                self.scroll = 0;
            } else {
                self.notice = Some(NOT_QUIET.to_owned());
            }
            return QuestionKey::Taken(None);
        }
        if self.target(is_quiet).is_none() {
            return QuestionKey::Taken(None);
        }
        let position = draft.position();
        let refusal = match (key, number) {
            (_, Some(number)) => (!draft.pick(number as usize)).then_some(NO_SUCH_OPTION),
            (Key::Enter, _) => (!draft.confirm()).then_some(NOTHING_PICKED),
            _ => {
                draft.open_text();
                None
            }
        };
        if let Some(refusal) = refusal {
            self.notice = Some(refusal.to_owned());
            return QuestionKey::Taken(None);
        }
        QuestionKey::Taken(self.keep_or_send(draft, position))
    }

    fn press_in_question_text(&mut self, key: &Key, now: Instant) -> Option<Action> {
        let id = self
            .questions
            .values()
            .find(|draft| draft.text().is_some())?
            .id()
            .to_owned();
        let mut draft = self.questions.remove(&id)?;
        let position = draft.position();
        let entry_left = !contains(&self.entries, &id);
        match key {
            Key::Character(character) => draft.push(*character),
            Key::Backspace => draft.pop(),
            Key::Escape | Key::Enter if entry_left => {
                self.start_quiet_time(now);
                return None;
            }
            Key::Escape => {
                draft.close_text();
                self.start_quiet_time(now);
            }
            Key::Enter if draft.send_text() => self.start_quiet_time(now),
            _ => {}
        }
        self.keep_or_send(draft, position)
    }

    /// Keeps the answers so far. With an answer to each question, sends
    /// them as the answer to the entry. The pane starts at its top for
    /// another question than the one at `position`.
    fn keep_or_send(&mut self, draft: QuestionDraft, position: (usize, usize)) -> Option<Action> {
        if draft.position() != position {
            self.scroll = 0;
        }
        if let Some(answers) = draft.finished() {
            let answer = Answer::Allow {
                answers: Some(answers),
            };
            return Some(self.record(draft.id().to_owned(), answer));
        }
        self.questions.insert(draft.id().to_owned(), draft);
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

    fn question_text_entry_left(&self) -> bool {
        self.questions
            .values()
            .any(|draft| draft.text().is_some() && !contains(&self.entries, draft.id()))
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

/// What the questions of the selected entry did with a key.
enum QuestionKey {
    /// The key is none of theirs, or the entry has no questions.
    NotOne,
    Taken(Option<Action>),
}

fn contains(entries: &[ListedEntry], id: &str) -> bool {
    entries.iter().any(|listed| listed.id == id)
}
