//! The answers to the questions of one prompt while the owner gives them.

use std::collections::{BTreeMap, BTreeSet};

use crate::{answer::MAX_MESSAGE_CHARS, question::Question};

/// What the owner marked for one question: options, and an own answer.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Marks {
    picked: BTreeSet<usize>,
    own: Option<String>,
}

/// What `QuestionDraft::go_on` did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    /// The current question has no answer, or one that the queue does not
    /// take, with the reason.
    Refused(&'static str),
    /// The next question shows.
    Next,
    /// Each question has its answer, by the text of the question.
    Finished(BTreeMap<String, String>),
}

const NOTHING_MARKED: &str = "Mark an option with its number first, or give an own answer with t";
const ANSWER_TOO_LONG: &str =
    "The marked options and the own answer are longer than 500 characters";

/// The questions of one entry with the marks of the owner. The owner goes
/// from one question to the next. No mark is an answer before the last
/// question is done.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionDraft {
    id: String,
    questions: Vec<Question>,
    index: usize,
    marks: Vec<Marks>,
    text: Option<String>,
}

impl QuestionDraft {
    /// Starts at the first question. The entry has at least one, as
    /// `questions_of` reads them.
    #[must_use]
    pub fn new(id: String, questions: Vec<Question>) -> Self {
        let marks = vec![Marks::default(); questions.len()];
        Self {
            id,
            questions,
            index: 0,
            marks,
            text: None,
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The question that the marks are for.
    #[must_use]
    pub fn current(&self) -> &Question {
        &self.questions[self.index]
    }

    /// The number of the current question, from 1, and the count of all.
    #[must_use]
    pub fn position(&self) -> (usize, usize) {
        (self.index + 1, self.questions.len())
    }

    #[must_use]
    pub fn is_at_last(&self) -> bool {
        self.index + 1 == self.questions.len()
    }

    /// True for a marked option of the current question. Options count
    /// from 0.
    #[must_use]
    pub fn is_picked(&self, option: usize) -> bool {
        self.marks[self.index].picked.contains(&option)
    }

    /// The own answer to the current question that the owner kept.
    #[must_use]
    pub fn own_answer(&self) -> Option<&str> {
        self.marks[self.index].own.as_deref()
    }

    /// The questions before the current one with their answers, in the
    /// order of the prompt.
    pub fn answered(&self) -> impl Iterator<Item = (&str, String)> {
        (0..self.index).filter_map(|index| {
            Some((self.questions[index].question.as_str(), self.answer(index)?))
        })
    }

    /// The text of an own answer while the owner types it.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Marks the option with the number, from 1, or takes its mark away. A
    /// question with one answer keeps one mark, and no own answer beside
    /// it. Returns false when the question has no such option.
    pub fn pick(&mut self, number: usize) -> bool {
        let Some(option) = number
            .checked_sub(1)
            .filter(|option| *option < self.current().options.len())
        else {
            return false;
        };
        let has_one_answer = !self.current().multi_select;
        let marks = &mut self.marks[self.index];
        let was_picked = marks.picked.remove(&option);
        if has_one_answer {
            marks.picked.clear();
            marks.own = None;
        }
        if !was_picked {
            marks.picked.insert(option);
        }
        true
    }

    /// Goes to the next question when the current one has an answer. After
    /// the last question, gives the answers.
    pub fn go_on(&mut self) -> Step {
        match self.answer(self.index) {
            None => return Step::Refused(NOTHING_MARKED),
            Some(answer) if answer.chars().count() > MAX_MESSAGE_CHARS => {
                return Step::Refused(ANSWER_TOO_LONG);
            }
            Some(_) => {}
        }
        if !self.is_at_last() {
            self.index += 1;
            return Step::Next;
        }
        let answers = self
            .questions
            .iter()
            .enumerate()
            .filter_map(|(index, question)| Some((question.question.clone(), self.answer(index)?)))
            .collect();
        Step::Finished(answers)
    }

    /// Goes back to the question before. Returns false at the first one.
    pub fn go_back(&mut self) -> bool {
        let is_later = self.index > 0;
        self.index = self.index.saturating_sub(1);
        is_later
    }

    /// Opens the text of an own answer to the current question, with the
    /// own answer that the owner kept before.
    pub fn open_text(&mut self) {
        self.text = Some(self.own_answer().unwrap_or_default().to_owned());
    }

    /// Closes the text without a change of the own answer.
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

    /// Closes the open text and keeps it as the own answer to the current
    /// question. A text of only blanks takes the own answer away. A
    /// question with one answer then has no marked option.
    pub fn keep_text(&mut self) {
        let Some(text) = self.text.take() else {
            return;
        };
        let text = text.trim();
        let has_one_answer = !self.current().multi_select;
        let marks = &mut self.marks[self.index];
        marks.own = (!text.is_empty()).then(|| text.to_owned());
        if has_one_answer && marks.own.is_some() {
            marks.picked.clear();
        }
    }

    /// The answer to the question from its marks: the labels of the marked
    /// options in the order of the question, then the own answer, joined
    /// with a comma and a blank. `None` without a mark.
    fn answer(&self, index: usize) -> Option<String> {
        let marks = &self.marks[index];
        let parts: Vec<&str> = marks
            .picked
            .iter()
            .filter_map(|option| self.questions[index].options.get(*option))
            .map(|choice| choice.label.as_str())
            .chain(marks.own.as_deref())
            .collect();
        (!parts.is_empty()).then(|| parts.join(", "))
    }
}
