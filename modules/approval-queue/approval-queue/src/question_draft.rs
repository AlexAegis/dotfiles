//! The answers to the questions of one prompt while the owner gives them.

use std::collections::{BTreeMap, BTreeSet};

use crate::{answer::MAX_MESSAGE_CHARS, question::Question};

/// The questions of one entry with the answers so far. The owner answers
/// one question after the other.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionDraft {
    id: String,
    questions: Vec<Question>,
    index: usize,
    picked: BTreeSet<usize>,
    answers: BTreeMap<String, String>,
    text: Option<String>,
}

impl QuestionDraft {
    /// Starts at the first question. The entry has at least one, as
    /// `questions_of` reads them.
    #[must_use]
    pub fn new(id: String, questions: Vec<Question>) -> Self {
        Self {
            id,
            questions,
            index: 0,
            picked: BTreeSet::new(),
            answers: BTreeMap::new(),
            text: None,
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The question that the next answer is for.
    #[must_use]
    pub fn current(&self) -> &Question {
        &self.questions[self.index.min(self.questions.len() - 1)]
    }

    /// The number of the current question, from 1, and the count of all.
    #[must_use]
    pub fn position(&self) -> (usize, usize) {
        (self.index + 1, self.questions.len())
    }

    /// True for an option of the current question that the owner picked and
    /// did not confirm yet. Options count from 0.
    #[must_use]
    pub fn is_picked(&self, option: usize) -> bool {
        self.picked.contains(&option)
    }

    /// The answered questions with their answers, in the order of the
    /// prompt.
    pub fn answered(&self) -> impl Iterator<Item = (&str, &str)> {
        self.questions.iter().filter_map(|question| {
            let answer = self.answers.get(&question.question)?;
            Some((question.question.as_str(), answer.as_str()))
        })
    }

    /// The text of an own answer while the owner types it.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Picks the option with the number, from 1. For a question with one
    /// answer, the option is the answer. For a question with several, the
    /// option changes between picked and not picked, and `confirm` answers.
    /// Returns false when the question has no such option.
    pub fn pick(&mut self, number: usize) -> bool {
        let Some(option) = number.checked_sub(1) else {
            return false;
        };
        let Some(choice) = self.current().options.get(option) else {
            return false;
        };
        if self.current().multi_select {
            if !self.picked.remove(&option) {
                self.picked.insert(option);
            }
        } else {
            let label = choice.label.clone();
            self.give(label);
        }
        true
    }

    /// Answers a question with several answers by its picked options, in
    /// the order of the question, joined with a comma and a blank. Returns
    /// false when no option is picked.
    pub fn confirm(&mut self) -> bool {
        if self.picked.is_empty() {
            return false;
        }
        let labels: Vec<&str> = self
            .picked
            .iter()
            .filter_map(|option| self.current().options.get(*option))
            .map(|choice| choice.label.as_str())
            .collect();
        self.give(labels.join(", "));
        true
    }

    /// Opens the text of an own answer to the current question.
    pub fn open_text(&mut self) {
        self.text = Some(String::new());
    }

    pub fn close_text(&mut self) {
        self.text = None;
    }

    /// Adds the character to the open text when the text takes it: no
    /// control character, and no character past the longest answer.
    pub fn push(&mut self, character: char) {
        if let Some(text) = &mut self.text
            && !character.is_control()
            && text.chars().count() < MAX_MESSAGE_CHARS
        {
            text.push(character);
        }
    }

    pub fn pop(&mut self) {
        if let Some(text) = &mut self.text {
            text.pop();
        }
    }

    /// Answers the current question with the open text. Returns false for a
    /// text of only blanks, which stays open.
    pub fn send_text(&mut self) -> bool {
        let Some(text) = self.text.as_deref().map(str::trim) else {
            return false;
        };
        if text.is_empty() {
            return false;
        }
        let text = text.to_owned();
        self.give(text);
        true
    }

    /// The answers when each question has one.
    #[must_use]
    pub fn finished(&self) -> Option<BTreeMap<String, String>> {
        (self.index >= self.questions.len()).then(|| self.answers.clone())
    }

    fn give(&mut self, answer: String) {
        self.answers.insert(self.current().question.clone(), answer);
        self.index += 1;
        self.picked.clear();
        self.text = None;
    }
}
