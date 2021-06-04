//! # stateful_tree
//!
//! Helper for a stateful tree
//!

/**
 * MIT License
 *
 * tui-realm-treeview - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use super::{TuiTreeItem, TuiTreeState};

use tui_tree_widget::{flatten, identifier};

/// ## StatefulTree
///
/// A wrapper around a `TuiTree` to handle its state
#[derive(Debug)]
pub struct StatefulTree<'a> {
    pub state: TuiTreeState,
    pub items: Vec<TuiTreeItem<'a>>,
}

enum MoveDirection {
    Up,
    Down,
}

impl<'a> StatefulTree<'a> {
    /// ### new
    ///
    /// Instantiates a new Stateful tree
    pub fn new() -> Self {
        Self {
            state: TuiTreeState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(mut self, items: Vec<TuiTreeItem<'a>>) -> Self {
        self.items = items;
        self
    }

    /// ### next
    ///
    /// Move cursor to the next element (down)
    pub fn next(&mut self) {
        self.move_up_down(MoveDirection::Down);
    }

    /// ### previous
    ///
    /// Move cursor to the previous element (up)
    pub fn previous(&mut self) {
        self.move_up_down(MoveDirection::Up);
    }

    /// ### move_up_down
    ///
    /// Move the cursor up or down
    fn move_up_down(&mut self, direction: MoveDirection) {
        let visible = flatten(&self.state.get_all_opened(), &self.items);
        let current_identifier = self.state.selected();
        let current_index = visible
            .iter()
            .position(|o| o.identifier == current_identifier);
        let new_index = current_index.map_or(0, |current_index| {
            match direction {
                MoveDirection::Down => current_index.saturating_add(1),
                MoveDirection::Up => current_index.saturating_sub(1),
            }
            .min(visible.len() - 1)
        });
        let new_identifier = visible.get(new_index).unwrap().identifier.clone();
        self.state.select(new_identifier);
    }

    /// ### close
    ///
    /// Close selected tree node
    pub fn close(&mut self) {
        let selected = self.state.selected();
        if !self.state.close(&selected) {
            let (head, _) = identifier::get_without_leaf(&selected);
            self.state.select(head);
        }
    }

    /// ### open
    ///
    /// Open selected tree node
    pub fn open(&mut self) {
        self.state.open(self.state.selected());
    }
}