//! Canvas undo/redo history — 32-entry bounded stack.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.
//!
//! Ports `WorkspaceHistory` from `webizen-studio::canvas_editor`.
//! Each frame captures a snapshot of container positions, sizes, and
//! manifold seed state so the user can undo/redo layout changes.

use std::collections::VecDeque;

use crate::tool_chest::core::registry::ManifoldSeed;

/// Maximum frames retained in history.
const MAX_FRAMES: usize = 32;

/// A single snapshot of canvas state at a point in time.
#[derive(Clone, Debug)]
pub struct CanvasFrame {
    /// The manifold seed at this point.
    pub seed: ManifoldSeed,
    /// Human-readable label for this history entry.
    pub label: String,
}

/// Bounded undo/redo stack.
pub struct WorkspaceHistory {
    /// Past states (most recent at the back).
    past: VecDeque<CanvasFrame>,
    /// Future states (most recent at the front).
    future: VecDeque<CanvasFrame>,
    /// Current state.
    current: CanvasFrame,
}

impl WorkspaceHistory {
    /// Create a new history with an initial frame.
    pub fn new(initial: CanvasFrame) -> Self {
        Self {
            past: VecDeque::with_capacity(MAX_FRAMES),
            future: VecDeque::with_capacity(MAX_FRAMES),
            current: initial,
        }
    }

    /// Push a new frame, clearing the redo stack.
    pub fn push(&mut self, frame: CanvasFrame) {
        self.past.push_back(self.current.clone());
        if self.past.len() > MAX_FRAMES {
            self.past.pop_front();
        }
        self.future.clear();
        self.current = frame;
    }

    /// Undo — move current to future, pop past to current.
    /// Returns `true` if undo was possible.
    pub fn undo(&mut self) -> bool {
        if let Some(prev) = self.past.pop_back() {
            self.future.push_front(self.current.clone());
            self.current = prev;
            true
        } else {
            false
        }
    }

    /// Redo — move current to past, pop future to current.
    /// Returns `true` if redo was possible.
    pub fn redo(&mut self) -> bool {
        if let Some(next) = self.future.pop_front() {
            self.past.push_back(self.current.clone());
            self.current = next;
            true
        } else {
            false
        }
    }

    /// Can undo?
    pub fn can_undo(&self) -> bool {
        !self.past.is_empty()
    }

    /// Can redo?
    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }

    /// Get the current frame.
    pub fn current(&self) -> &CanvasFrame {
        &self.current
    }

    /// Replace the current frame without pushing history.
    pub fn replace_current(&mut self, frame: CanvasFrame) {
        self.current = frame;
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool_chest::core::registry::ManifoldSeed;

    fn test_seed(id: &str) -> ManifoldSeed {
        ManifoldSeed {
            id: id.into(),
            label: id.into(),
            icon: "test".into(),
            ontology_prefix: "test".into(),
            description: "test".into(),
            containers: vec![],
            connections: vec![],
            panels: vec![],
        }
    }

    #[test]
    fn test_undo_redo_basic() {
        let initial = CanvasFrame { seed: test_seed("a"), label: "initial".into() };
        let mut hist = WorkspaceHistory::new(initial);

        hist.push(CanvasFrame { seed: test_seed("b"), label: "edit 1".into() });
        hist.push(CanvasFrame { seed: test_seed("c"), label: "edit 2".into() });

        assert_eq!(hist.current().seed.id, "c");
        assert!(hist.can_undo());
        assert!(!hist.can_redo());

        assert!(hist.undo());
        assert_eq!(hist.current().seed.id, "b");
        assert!(hist.can_redo());

        assert!(hist.undo());
        assert_eq!(hist.current().seed.id, "a");
        assert!(!hist.can_undo());

        assert!(hist.redo());
        assert_eq!(hist.current().seed.id, "b");
    }

    #[test]
    fn test_push_clears_redo() {
        let initial = CanvasFrame { seed: test_seed("a"), label: "initial".into() };
        let mut hist = WorkspaceHistory::new(initial);

        hist.push(CanvasFrame { seed: test_seed("b"), label: "edit 1".into() });
        hist.undo();
        assert!(hist.can_redo());

        hist.push(CanvasFrame { seed: test_seed("d"), label: "edit 2".into() });
        assert!(!hist.can_redo());
    }

    #[test]
    fn test_max_frames_bounded() {
        let initial = CanvasFrame { seed: test_seed("0"), label: "initial".into() };
        let mut hist = WorkspaceHistory::new(initial);

        for i in 1..=40 {
            hist.push(CanvasFrame {
                seed: test_seed(&i.to_string()),
                label: format!("edit {}", i),
            });
        }

        // Should only be able to undo 32 times
        let mut undo_count = 0;
        while hist.undo() {
            undo_count += 1;
        }
        assert_eq!(undo_count, 32);
    }
}
