use super::{tt::TranspositionTableEntry, SearchState};
use shakmaty::{Move, MoveList};

const MO_FACTOR: i32 = 10000;

pub struct MovePicker {
    order: Vec<usize>, // sorted indices into moves
    curr: usize,
}

impl Iterator for MovePicker {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.order.len() {
            return None;
        }

        let curr = self.curr;
        self.curr += 1;

        Some(self.order[curr])
    }
}

impl MovePicker {
    pub fn new(
        moves: &MoveList,
        state: &SearchState,
        entry: &TranspositionTableEntry,
        ply: usize,
    ) -> Self {
        let mut scored_indices: Vec<(usize, i32)> = moves
            .iter()
            .enumerate()
            .map(|(i, m)| (i, Self::move_importance(state, entry, ply, m)))
            .collect();

        scored_indices.sort_by_key(|&(_, score)| -score);

        let order = scored_indices.into_iter().map(|(i, _)| i).collect();

        Self { order, curr: 0 }
    }

    fn move_importance(
        state: &SearchState,
        entry: &TranspositionTableEntry,
        ply: usize,
        m: &Move,
    ) -> i32 {
        if *m == entry._move {
            return state.cfg.mo_tt_entry_value.value * MO_FACTOR;
        }

        if m.is_capture() {
            let piece_value = m.role() as i32;
            let capture_value = m.capture().unwrap() as i32;
            return (state.cfg.mo_capture_value.value * capture_value - piece_value) * MO_FACTOR;
        }

        if state.km.get(ply).contains(m) {
            return state.cfg.mo_killer_value.value * MO_FACTOR;
        }

        if m.is_promotion() {
            return m.promotion().unwrap() as i32;
        }

        state.hist.get(m.role(), m.to())
    }
}
