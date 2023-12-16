use crate::lang::DisplayTranslated;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Openings {
    Any, AlekhineDefense, Benoni, Bird, BishopsOpening, BlackmarDiemerGambit, CaroKann, Catalan,
    Dutch, English, FourKnightsGame, French, GiuocoPiano, Grunfeld, HorwitzDefense, IndianDefense,
    ItalianGame, KingsGambit, KingsGambitAccepted, KingsGambitDeclined, KingsIndianAttack,
    KingsIndianDefense, KingsPawnGame, ModernDefense, NimzoIndianDefense, NimzoLarsenAttack,
    NimzowitschDefense, PhilidorDefense, PircDefense, Ponziani, QueensGambitAccepted,
    QueensGambitDeclined, QueensPawnGame, RapportJobavaSystem, Reti, RussianGame, RuyLopez,
    Scandinavian, ScotchGame, SemiSlav,Sicilian, SlavDefense, ThreeKnightsOpening, Trompowsky,
    ViennaGame, ZukertortOpening
}

impl Openings {
    pub const ALL: [Openings; 46] = [
        Openings::Any, Openings::AlekhineDefense, Openings::Benoni, Openings::Bird,
        Openings::BishopsOpening, Openings::BlackmarDiemerGambit, Openings::CaroKann,
        Openings::Catalan, Openings::Dutch, Openings::English, Openings::FourKnightsGame,
        Openings::French, Openings::GiuocoPiano, Openings::Grunfeld, Openings::HorwitzDefense,
        Openings::IndianDefense, Openings::ItalianGame, Openings::KingsGambit,
        Openings::KingsGambitAccepted, Openings::KingsGambitDeclined, Openings::KingsIndianAttack,
        Openings::KingsIndianDefense, Openings::KingsPawnGame, Openings::ModernDefense,
        Openings::NimzoIndianDefense, Openings::NimzoLarsenAttack, Openings::NimzowitschDefense,
        Openings::PhilidorDefense, Openings::PircDefense, Openings::Ponziani,
        Openings::QueensGambitAccepted, Openings::QueensGambitDeclined, Openings::QueensPawnGame,
        Openings::RapportJobavaSystem, Openings::Reti, Openings::RussianGame, Openings::RuyLopez,
        Openings::Scandinavian, Openings::ScotchGame, Openings::SemiSlav, Openings::Sicilian,
        Openings::SlavDefense, Openings::ThreeKnightsOpening, Openings::Trompowsky,
        Openings::ViennaGame, Openings::ZukertortOpening];

    pub fn get_field_name(&self) -> &str {
        match self {
            Openings::Any => "",
            Openings::Sicilian => "Sicilian_Defense", Openings::French => "French_Defense",
            Openings::QueensPawnGame => "Queens_Pawn_Game", Openings::ItalianGame => "Italian_Game",
            Openings::CaroKann => "Caro-Kann_Defense", Openings::QueensGambitDeclined => "Queens_Gambit_Declined",
            Openings::Scandinavian => "Scandinavian_Defense", Openings::RuyLopez => "Ruy_Lopez",
            Openings::English => "English_Opening", Openings::IndianDefense => "Indian_Defense",
            Openings::ScotchGame => "Scotch_Game", Openings::PhilidorDefense => "Philidor_Defense",
            Openings::RussianGame => "Russian_Game", Openings::ModernDefense => "Modern_Defense",
            Openings::FourKnightsGame => "Four_Knights_Game", Openings::KingsGambitAccepted => "Kings_Gambit_Accepted",
            Openings::SlavDefense => "Slav_Defense", Openings::PircDefense => "Pirc_Defense",
            Openings::ZukertortOpening => "Zukertort_Opening", Openings::BishopsOpening => "Bishops_Opening",
            Openings::KingsPawnGame => "Kings_Pawn_Game", Openings::ViennaGame => "Vienna_Game",
            Openings::KingsIndianDefense => "Kings_Indian_Defense", Openings::QueensGambitAccepted => "Queens_Gambit_Accepted",
            Openings::Benoni => "Benoni_Defense", Openings::AlekhineDefense => "Alekhine_Defense",
            Openings::NimzowitschDefense => "Nimzowitsch_Defense", Openings::HorwitzDefense => "Horwitz_Defense",
            Openings::NimzoLarsenAttack => "Nimzo-Larsen_Attack", Openings::KingsGambitDeclined => "Kings_Gambit_Declined",
            Openings::NimzoIndianDefense => "Nimzo-Indian_Defense", Openings::Bird => "Bird_Opening",
            Openings::Dutch => "Dutch_Defense", Openings::SemiSlav => "Semi-Slav_Defense",
            Openings::GiuocoPiano => "Giuoco_Piano", Openings::Grunfeld => "Grunfeld_Defense",
            Openings::ThreeKnightsOpening => "Three_Knights_Opening", Openings::Ponziani => "Ponziani_Opening",
            Openings::KingsIndianAttack => "Kings_Indian_Attack", Openings::BlackmarDiemerGambit => "Blackmar-Diemer_Gambit",
            Openings::Trompowsky => "Trompowsky_Attack", Openings::KingsGambit => "Kings_Gambit",
            Openings::RapportJobavaSystem => "Rapport-Jobava_System", Openings::Catalan => "Catalan_Opening",
            Openings::Reti => "Reti_Opening"
        }
    }
    pub fn get_tr_key(&self) -> &str {
        match self {
            Openings::Any => "any_opening",
            _ => self.get_field_name(),
        }
    }
}

impl DisplayTranslated for Openings {
    fn to_str_tr(&self) -> &str {
        self.get_tr_key()
    }
}
