use std::ops::Deref;

use ratatui::widgets::TableState;

#[derive(Debug, Default)]
pub struct SelectableTable<C> {
    pub items: C,
    pub state: TableState,
}

impl<C, T> SelectableTable<C>
where
    C: Deref<Target = Vec<T>>,
{
    pub fn new(items: C) -> Self {
        let mut state = TableState::default();
        state.select_first();
        state.select_first_column();
        Self { state, items }
    }

    pub fn get_selected(&self) -> Option<&T> {
        self.items.get(self.state.selected()?)
    }

    /// Selects the first item.
    pub fn select_first(&mut self) {
        self.state.select(Some(0));
    }

    /// Selects the last item.
    pub fn select_last(&mut self) {
        self.state.select(Some(self.items.len().saturating_sub(1)));
    }

    pub fn select_next(&mut self, amount: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i.saturating_add(amount) >= self.items.len() {
                    0
                } else {
                    i.saturating_add(amount)
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select_previous(&mut self, amount: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len().saturating_sub(1)
                } else {
                    i.saturating_sub(amount)
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
