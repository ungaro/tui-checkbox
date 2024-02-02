use std::collections::HashSet;
use ratatui::buffer::Buffer;
use ratatui::layout::{Corner, Rect};
use ratatui::style::Style;
use ratatui::text::Text;
use ratatui::widgets::{Block, StatefulWidget, Widget};
//use unicode_width::UnicodeWidthStr;


#[derive(Debug, Default, Clone)]
pub struct CheckboxState<Identifier> {
    opened: HashSet<Identifier>,
    selected: Identifier,
}

impl<Identifier> TreeState<Identifier>
where
    Identifier: Clone + PartialEq + Eq + core::hash::Hash,
{



    #[must_use]
    pub fn selected(&self) -> Vec<Identifier> {
        self.selected.clone()
    }

    /// Selects the given identifier.
    ///
    /// Returns `true` when the selection changed.
    ///
    /// Clear the selection by passing an empty identifier vector:
    ///
    /// ```rust
    /// # use tui_tree_widget::TreeState;
    /// # let mut state = TreeState::<usize>::default();
    /// state.select(Vec::new());
    /// ```
    pub fn select(&mut self, identifier: Vec<Identifier>) -> bool {
        let changed = self.selected != identifier;
        self.selected = identifier;

        // TODO: ListState does this. Is this relevant?
        if self.selected.is_empty() {
            self.offset = 0;
        }

        changed
    }



    /// Toggles a tree node.
    /// If the node is in opened then it calls `close()`. Otherwise it calls `open()`.
    pub fn toggle(&mut self, identifier: Identifier) {
        if self.opened.contains(&identifier) {
            self.close(&identifier);
        } else {
            self.open(identifier);
        }
    }

    /// Toggles the currently selected tree node.
    /// See also [`toggle`](TreeState::toggle)
    pub fn toggle_selected(&mut self) {
        self.toggle(self.selected());
    }

    pub fn close_all(&mut self) {
        self.opened.clear();
    }

    /// Select the first node.
    ///
    /// Returns `true` when the selection changed.
    pub fn select_first(&mut self, items: &[TreeItem<Identifier>]) -> bool {
        let identifier = items
            .first()
            .map(|o| vec![o.identifier.clone()])
            .unwrap_or_default();
        self.select(identifier)
    }

    /// Select the last visible node.
    ///
    /// Returns `true` when the selection changed.
    pub fn select_last(&mut self, items: &[TreeItem<Identifier>]) -> bool {
        let visible = self.flatten(items);
        let new_identifier = visible
            .last()
            .map(|o| o.identifier.clone())
            .unwrap_or_default();
        self.select(new_identifier)
    }



}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
