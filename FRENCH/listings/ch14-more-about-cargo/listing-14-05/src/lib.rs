// ANCHOR: here
//! # Art
//!
//! Une bibliothèque pour modéliser des concepts artistiques.

pub use self::types::CouleurPrimaire;
pub use self::types::CouleurSecondaire;
pub use self::utilitaires::mixer;

pub mod types {
    // -- partie masquée ici --
    // ANCHOR_END: here
    /// Les couleurs primaires du modèle RJB.
    pub enum CouleurPrimaire {
        Rouge,
        Jaune,
        Bleu,
    }

    /// Les couleurs secondaires du modèle RJB.
    pub enum CouleurSecondaire {
        Orange,
        Vert,
        Violet,
    }
    // ANCHOR: here
}

pub mod utilitaires {
    // -- partie masquée ici --
    // ANCHOR_END: here
    use crate::types::*;

    /// Combine deux couleurs primaires dans les mêmes quantités pour
    /// créer une couleur secondaire.
    pub fn mixer(c1: CouleurPrimaire, c2: CouleurPrimaire) -> CouleurSecondaire {
        CouleurSecondaire::Orange
    }
    // ANCHOR: here
}
// ANCHOR_END: here
