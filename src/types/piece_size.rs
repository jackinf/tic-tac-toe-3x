use std::cmp::Ordering;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum PieceSize {
    Small,
    Medium,
    Large,
}
//
// impl PartialEq<Self> for PieceSize {
//     fn eq(&self, other: &Self) -> bool {
//         use PieceSize::*;
//         match (self, other) {
//             (Small, Small) => true,
//             (Medium, Medium) => true,
//             (Large, Large) => true,
//             _ => false,
//         }
//     }
// }
//
// impl PartialOrd<PieceSize> for PieceSize {
//     fn partial_cmp(&self, other: &PieceSize) -> Option<Ordering> {
//         use PieceSize::*;
//         match (self, other) {
//             (Small, Small) => Some(Ordering::Equal),
//             (Small, Medium) => Some(Ordering::Less),
//             (Small, Large) => Some(Ordering::Less),
//             (Medium, Small) => Some(Ordering::Greater),
//             (Medium, Medium) => Some(Ordering::Equal),
//             (Medium, Large) => Some(Ordering::Less),
//             (Large, Small) => Some(Ordering::Greater),
//             (Large, Medium) => Some(Ordering::Greater),
//             (Large, Large) => Some(Ordering::Equal),
//         }
//     }
// }
