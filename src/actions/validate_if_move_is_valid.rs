use crate::components::Piece;

pub fn validate_if_move_is_valid(pieces: &Vec<Piece>, candidate: &Piece) -> bool {
    let coord = candidate.coord();
    let size = candidate.size();

    let same_coord_pieces: Vec<Piece> = pieces
        .iter()
        .filter(|piece| piece.coord() == coord)
        .cloned()
        .collect();

    if same_coord_pieces.is_empty() {
        return true;
    }

    same_coord_pieces.iter().all(|piece| piece.size() < size)
}
