use super::{tt::TranspositionTableEntry, SearchState};
use shakmaty::{Move, MoveList, Role};

const MO_FACTOR: i32 = 10000;

pub struct MovePicker<'a> {
    order: Vec<&'a Move>,
    curr: usize,
}

impl<'a> Iterator for MovePicker<'a> {
    type Item = &'a Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.order.len() {
            return None;
        }

        let move_ref = self.order[self.curr];
        self.curr += 1;

        Some(move_ref)
    }
}

impl<'a> MovePicker<'a> {
    pub fn new(
        moves: &'a MoveList,
        state: &SearchState,
        entry: &TranspositionTableEntry,
        ply: usize,
    ) -> Self {
        let mut scored_moves: Vec<(&'a Move, i32)> = moves
            .iter()
            .map(|m_ref| (m_ref, Self::move_importance(state, entry, ply, m_ref)))
            .collect();

        scored_moves.sort_by_key(|&(_, score)| -score);

        let order: Vec<&'a Move> = scored_moves.into_iter().map(|(m_ref, _)| m_ref).collect();

        Self { order, curr: 0 }
    }

    fn move_importance(
        state: &SearchState,
        entry: &TranspositionTableEntry,
        ply: usize,
        m: &Move,
    ) -> i32 {
        if let Some(tt_move) = &entry._move {
            if *m == *tt_move {
                return state.cfg.mo_tt_entry_value.value * MO_FACTOR;
            }
        }

        if m.is_capture() {
            let moving_piece_value = m.role() as i32;
            let captured_piece_value = m.capture().unwrap_or(Role::Pawn) as i32;
            return (state.cfg.mo_capture_value.value * captured_piece_value - moving_piece_value)
                * MO_FACTOR;
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
