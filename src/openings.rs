use std::borrow::Cow;

use crate::lang::{self, PickListWrapper, DisplayTranslated};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Openings {
    Any, Sicilian, French, QueensPawnGame, ItalianGame, CaroKann,Scandinavian,QueensGambitDeclined,
    English, RuyLopez, IndianDefense, ScotchGame, RussianGame, PhilidorDefense, ModernDefense,
    FourKnightsGame, KingsGambitAccepted, ZukertortOpening, BishopsOpening, SlavDefense, PircDefense,
    KingsPawnGame, ViennaGame, QueensGambitAccepted, KingsIndianDefense, Benoni, NimzowitschDefense,
    AlekhineDefense, NimzoLarsenAttack, HorwitzDefense, KingsGambitDeclined, OwenDefense, Bird,
    Dutch, NimzoIndianDefense, VantKruijsOpening, SemiSlav, CenterGame, HungarianOpening, ThreeKnightsOpening,
    PonzianiOpening, EnglundGambit, GrunfeldDefense, BlackmarDiemerGambit, ElephantGambit, PolishOpening,
    DanishGambit, KingsIndianAttack, TrompowskyAttack, EnglishDefense, GrobOpening, RapportJobavaSystem,
    TarraschDefense, CatalanOpening, Reti, QueensIndianDefense, LondonSystem
}

impl Openings {
    pub const ALL: [Openings; 57] = [
        Openings::Any, Openings::Sicilian, Openings::French, Openings::QueensPawnGame, Openings::ItalianGame,
        Openings::CaroKann, Openings::Scandinavian, Openings::QueensGambitDeclined,
        Openings::English, Openings::RuyLopez, Openings::IndianDefense, Openings::ScotchGame,
        Openings::RussianGame, Openings::PhilidorDefense, Openings::ModernDefense, Openings::FourKnightsGame,
        Openings::KingsGambitAccepted, Openings::ZukertortOpening, Openings::BishopsOpening,
        Openings::SlavDefense, Openings::PircDefense, Openings::KingsPawnGame, Openings::ViennaGame,
        Openings::QueensGambitAccepted, Openings::KingsIndianDefense, Openings::Benoni,
        Openings::NimzowitschDefense, Openings::AlekhineDefense, Openings::NimzoLarsenAttack,
        Openings::HorwitzDefense, Openings::KingsGambitDeclined, Openings::OwenDefense, Openings::Bird,
        Openings::Dutch, Openings::NimzoIndianDefense, Openings::VantKruijsOpening, Openings::SemiSlav,
        Openings::CenterGame, Openings::HungarianOpening, Openings::ThreeKnightsOpening,
        Openings::PonzianiOpening, Openings::EnglundGambit, Openings::GrunfeldDefense,
        Openings::BlackmarDiemerGambit, Openings::ElephantGambit, Openings::PolishOpening,
        Openings::DanishGambit, Openings::KingsIndianAttack, Openings::TrompowskyAttack,
        Openings::EnglishDefense, Openings::GrobOpening, Openings::RapportJobavaSystem,
        Openings::TarraschDefense, Openings::CatalanOpening, Openings::Reti, Openings::QueensIndianDefense,
        Openings::LondonSystem];

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
            Openings::OwenDefense => "Owen_Defense", Openings::VantKruijsOpening => "Vant_Kruijs_Opening",
            Openings::CenterGame => "Center_Game", Openings::HungarianOpening => "Hungarian_Opening",
            Openings::ThreeKnightsOpening => "Three_Knights_Opening", Openings::PonzianiOpening => "Ponziani_Opening",
            Openings::EnglundGambit => "Englund_Gambit", Openings::GrunfeldDefense => "Grunfeld_Defense",
            Openings::BlackmarDiemerGambit => "Blackmar-Diemer_Gambit", Openings::ElephantGambit => "Elephant_Gambit",
            Openings::PolishOpening => "Polish_Opening", Openings::DanishGambit => "Danish_Gambit",
            Openings::KingsIndianAttack => "Kings_Indian_Attack", Openings::TrompowskyAttack => "Trompowsky_Attack",
            Openings::EnglishDefense => "English_Defense", Openings::GrobOpening => "Grob_Opening",
            Openings::RapportJobavaSystem => "Rapport-Jobava_System", Openings::TarraschDefense => "Tarrasch_Defense",
            Openings::CatalanOpening => "Catalan_Opening", Openings::Reti => "Reti_Opening",
            Openings::QueensIndianDefense => "Queens_Indian_Defense", Openings::LondonSystem => "London_System"
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

impl PickListWrapper<Openings> {
    pub fn get_openings(lang: lang::Language) -> Vec<PickListWrapper<Openings>> {
        let mut openings_wrapper = Vec::new();
        for opening in Openings::ALL {
            openings_wrapper.push(
                PickListWrapper::<Openings> {
                    lang: lang,
                    item: opening,
                }
            );
        }
        openings_wrapper
    }

    pub fn new_opening(lang: lang::Language, opening: Openings) -> Self {
        Self { lang, item: opening}
    }
}

// Variations

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Variation {
    pub name: Cow<'static, str>,
    pub family: Openings,
}

impl Variation {
    pub const ANY_STR: &'static str = "Any_Variation";
    pub const ANY: Variation = Variation {name: Cow::Borrowed("Any_Variation"), family: Openings::Any};

    const SICILIAN_VARIATIONS: [Variation; 31] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Bowdler_Attack"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Old_Sicilian"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Closed"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Najdorf_Variation"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Alapin_Variation"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Modern_Variations"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Smith-Morra_Gambit"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_McDonnell_Attack"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Open"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Taimanov_Variation"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_French_Variation"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Nyezhmetdinov-Rossolimo_Attack"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Accelerated_Dragon"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Dragon_Variation"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Kan_Variation"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Lowenthal_Variation"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Lasker-Pelikan_Variation"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Four_Knights_Variation"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_OKelly_Variation"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Smith-Morra_Gambit_Accepted"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Grand_Prix_Attack"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Hyperaccelerated_Fianchetto"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Hyperaccelerated_Dragon"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Delayed_Alapin_Variation"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Wing_Gambit"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Classical_Variation"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Staunton-Cochrane_Variation"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Marshall_Gambit"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Scheveningen_Variation"), family: Openings::Sicilian},
        Variation {name: Cow::Borrowed("Sicilian_Defense_Smith-Morra_Gambit_Declined"), family: Openings::Sicilian},
    ];
    const FRENCH_VARIATIONS: [Variation; 14] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::French},
        Variation {name: Cow::Borrowed("French_Defense_Advance_Variation"), family: Openings::French},
        Variation {name: Cow::Borrowed("French_Defense_Knight_Variation"), family: Openings::French},
        Variation {name: Cow::Borrowed("French_Defense_Exchange_Variation"), family: Openings::French},
        Variation {name: Cow::Borrowed("French_Defense_Tarrasch_Variation"), family: Openings::French},
        Variation {name: Cow::Borrowed("French_Defense_Normal_Variation"), family: Openings::French},
        Variation {name: Cow::Borrowed("French_Defense_Winawer_Variation"), family: Openings::French},
        Variation {name: Cow::Borrowed("French_Defense_Rubinstein_Variation"), family: Openings::French},
        Variation {name: Cow::Borrowed("French_Defense_Classical_Variation"), family: Openings::French},
        Variation {name: Cow::Borrowed("French_Defense_La_Bourdonnais_Variation"), family: Openings::French},
        Variation {name: Cow::Borrowed("French_Defense_Queens_Knight"), family: Openings::French},
        Variation {name: Cow::Borrowed("French_Defense_Kings_Indian_Attack"), family: Openings::French},
        Variation {name: Cow::Borrowed("French_Defense_Steiner_Variation"), family: Openings::French},
        Variation {name: Cow::Borrowed("French_Defense_Two_Knights_Variation"), family: Openings::French},
    ];

    const QUEENS_PAWN_GAME_VARIATIONS: [Variation; 10] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::QueensPawnGame},
        Variation {name: Cow::Borrowed("Queens_Pawn_Game_Accelerated_London_System"), family: Openings::QueensPawnGame},
        Variation {name: Cow::Borrowed("Queens_Pawn_Game_Chigorin_Variation"), family: Openings::QueensPawnGame},
        Variation {name: Cow::Borrowed("Queens_Pawn_Game_Zukertort_Variation"), family: Openings::QueensPawnGame},
        Variation {name: Cow::Borrowed("Queens_Pawn_Game_London_System"), family: Openings::QueensPawnGame},
        Variation {name: Cow::Borrowed("Queens_Pawn_Game_Colle_System"), family: Openings::QueensPawnGame},
        Variation {name: Cow::Borrowed("Queens_Pawn_Game_Symmetrical_Variation"), family: Openings::QueensPawnGame},
        Variation {name: Cow::Borrowed("Queens_Pawn_Game_Levitsky_Attack"), family: Openings::QueensPawnGame},
        Variation {name: Cow::Borrowed("Queens_Pawn_Game_Krause_Variation"), family: Openings::QueensPawnGame},
        Variation {name: Cow::Borrowed("Queens_Pawn_Game_Torre_Attack"), family: Openings::QueensPawnGame},
    ];

    const ITALIAN_VARIATIONS: [Variation; 10] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::ItalianGame},
        Variation {name: Cow::Borrowed("Italian_Game_Two_Knights_Defense"), family: Openings::ItalianGame},
        Variation {name: Cow::Borrowed("Italian_Game_Classical_Variation"), family: Openings::ItalianGame},
        Variation {name: Cow::Borrowed("Italian_Game_Giuoco_Pianissimo"), family: Openings::ItalianGame},
        Variation {name: Cow::Borrowed("Italian_Game_Anti-Fried_Liver_Defense"), family: Openings::ItalianGame},
        Variation {name: Cow::Borrowed("Italian_Game_Giuoco_Piano"), family: Openings::ItalianGame},
        Variation {name: Cow::Borrowed("Italian_Game_Evans_Gambit"), family: Openings::ItalianGame},
        Variation {name: Cow::Borrowed("Italian_Game_Scotch_Gambit"), family: Openings::ItalianGame},
        Variation {name: Cow::Borrowed("Italian_Game_Blackburne-Kostic_Gambit"), family: Openings::ItalianGame},
        Variation {name: Cow::Borrowed("Italian_Game_Hungarian_Defense"), family: Openings::ItalianGame},
    ];

    const CARO_KANN_VARIATIONS: [Variation; 11] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::CaroKann},
        Variation {name: Cow::Borrowed("Caro-Kann_Defense_Advance_Variation"), family: Openings::CaroKann},
        Variation {name: Cow::Borrowed("Caro-Kann_Defense_Exchange_Variation"), family: Openings::CaroKann},
        Variation {name: Cow::Borrowed("Caro-Kann_Defense_Classical_Variation"), family: Openings::CaroKann},
        Variation {name: Cow::Borrowed("Caro-Kann_Defense_Two_Knights_Attack"), family: Openings::CaroKann},
        Variation {name: Cow::Borrowed("Caro-Kann_Defense_Panov_Attack"), family: Openings::CaroKann},
        Variation {name: Cow::Borrowed("Caro-Kann_Defense_Hillbilly_Attack"), family: Openings::CaroKann},
        Variation {name: Cow::Borrowed("Caro-Kann_Defense_Accelerated_Panov_Attack"), family: Openings::CaroKann},
        Variation {name: Cow::Borrowed("Caro-Kann_Defense_Maroczy_Variation"), family: Openings::CaroKann},
        Variation {name: Cow::Borrowed("Caro-Kann_Defense_Tartakower_Variation"), family: Openings::CaroKann},
        Variation {name: Cow::Borrowed("Caro-Kann_Defense_Karpov_Variation"), family: Openings::CaroKann},
    ];

    const SCANDINAVIAN_VARIATIONS: [Variation; 7] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::Scandinavian},
        Variation {name: Cow::Borrowed("Scandinavian_Defense_Mieses-Kotroc_Variation"), family: Openings::Scandinavian},
        Variation {name: Cow::Borrowed("Scandinavian_Defense_Modern_Variation"), family: Openings::Scandinavian},
        Variation {name: Cow::Borrowed("Scandinavian_Defense_Main_Line"), family: Openings::Scandinavian},
        Variation {name: Cow::Borrowed("Scandinavian_Defense_Gubinsky-Melts_Defense"), family: Openings::Scandinavian},
        Variation {name: Cow::Borrowed("Scandinavian_Defense_Icelandic-Palme_Gambit"), family: Openings::Scandinavian},
        Variation {name: Cow::Borrowed("Scandinavian_Defense_Panov_Transfer"), family: Openings::Scandinavian},
    ];

    const QUEENS_GAMBIT_DECLINED_VARIATIONS: [Variation; 14] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::QueensGambitDeclined},
        Variation {name: Cow::Borrowed("Queens_Gambit_Declined_Marshall_Defense"), family: Openings::QueensGambitDeclined},
        Variation {name: Cow::Borrowed("Queens_Gambit_Declined_Albin_Countergambit"), family: Openings::QueensGambitDeclined},
        Variation {name: Cow::Borrowed("Queens_Gambit_Declined_Exchange_Variation"), family: Openings::QueensGambitDeclined},
        Variation {name: Cow::Borrowed("Queens_Gambit_Declined_Modern_Variation"), family: Openings::QueensGambitDeclined},
        Variation {name: Cow::Borrowed("Queens_Gambit_Declined_Chigorin_Defense"), family: Openings::QueensGambitDeclined},
        Variation {name: Cow::Borrowed("Queens_Gambit_Declined_Ragozin_Defense"), family: Openings::QueensGambitDeclined},
        Variation {name: Cow::Borrowed("Queens_Gambit_Declined_Queens_Knight_Variation"), family: Openings::QueensGambitDeclined},
        Variation {name: Cow::Borrowed("Queens_Gambit_Declined_Baltic_Defense"), family: Openings::QueensGambitDeclined},
        Variation {name: Cow::Borrowed("Queens_Gambit_Declined_Normal_Defense"), family: Openings::QueensGambitDeclined},
        Variation {name: Cow::Borrowed("Queens_Gambit_Declined_Three_Knights_Variation"), family: Openings::QueensGambitDeclined},
        Variation {name: Cow::Borrowed("Queens_Gambit_Declined_Harrwitz_Attack"), family: Openings::QueensGambitDeclined},
        Variation {name: Cow::Borrowed("Queens_Gambit_Declined_Semi-Tarrasch_Defense"), family: Openings::QueensGambitDeclined},
        Variation {name: Cow::Borrowed("Queens_Gambit_Declined_Tarrasch_Defense"), family: Openings::QueensGambitDeclined},
    ];

    const ENGLISH_VARIATIONS: [Variation; 7] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::English},
        Variation {name: Cow::Borrowed("English_Opening_Kings_English_Variation"), family: Openings::English},
        Variation {name: Cow::Borrowed("English_Opening_Symmetrical_Variation"), family: Openings::English},
        Variation {name: Cow::Borrowed("English_Opening_Anglo-Indian_Defense"), family: Openings::English},
        Variation {name: Cow::Borrowed("English_Opening_Agincourt_Defense"), family: Openings::English},
        Variation {name: Cow::Borrowed("English_Opening_Anglo-Scandinavian_Defense"), family: Openings::English},
        Variation {name: Cow::Borrowed("English_Opening_Caro-Kann_Defensive_System"), family: Openings::English},
    ];

    const RUY_LOPEZ_VARIATIONS: [Variation; 12] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::RuyLopez},
        Variation {name: Cow::Borrowed("Ruy_Lopez_Morphy_Defense"), family: Openings::RuyLopez},
        Variation {name: Cow::Borrowed("Ruy_Lopez_Berlin_Defense"), family: Openings::RuyLopez},
        Variation {name: Cow::Borrowed("Ruy_Lopez_Steinitz_Defense"), family: Openings::RuyLopez},
        Variation {name: Cow::Borrowed("Ruy_Lopez_Classical_Variation"), family: Openings::RuyLopez},
        Variation {name: Cow::Borrowed("Ruy_Lopez_Exchange_Variation"), family: Openings::RuyLopez},
        Variation {name: Cow::Borrowed("Ruy_Lopez_Bird_Variation"), family: Openings::RuyLopez},
        Variation {name: Cow::Borrowed("Ruy_Lopez_Cozio_Defense"), family: Openings::RuyLopez},
        Variation {name: Cow::Borrowed("Ruy_Lopez_Schliemann_Defense"), family: Openings::RuyLopez},
        Variation {name: Cow::Borrowed("Ruy_Lopez_Closed"), family: Openings::RuyLopez},
        Variation {name: Cow::Borrowed("Ruy_Lopez_Open"), family: Openings::RuyLopez},
        Variation {name: Cow::Borrowed("Ruy_Lopez_Marshall_Attack"), family: Openings::RuyLopez},
    ];

    const INDIAN_DEFENSE_VARIATIONS: [Variation; 4] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::IndianDefense},
        Variation {name: Cow::Borrowed("Indian_Defense_Budapest_Defense"), family: Openings::IndianDefense},
        Variation {name: Cow::Borrowed("Indian_Defense_Normal_Variation"), family: Openings::IndianDefense},
        Variation {name: Cow::Borrowed("Indian_Defense_London_System"), family: Openings::IndianDefense},
    ];

    const SCOTCH_GAME_VARIATIONS: [Variation; 5] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::ScotchGame},
        Variation {name: Cow::Borrowed("Scotch_Game_Scotch_Gambit"), family: Openings::ScotchGame},
        Variation {name: Cow::Borrowed("Scotch_Game_Classical_Variation"), family: Openings::ScotchGame},
        Variation {name: Cow::Borrowed("Scotch_Game_Schmidt_Variation"), family: Openings::ScotchGame},
        Variation {name: Cow::Borrowed("Scotch_Game_Goring_Gambit"), family: Openings::ScotchGame},
    ];

    const RUSSIAN_GAME_VARIATIONS: [Variation; 6] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::RussianGame},
        Variation {name: Cow::Borrowed("Russian_Game_Stafford_Gambit"), family: Openings::RussianGame},
        Variation {name: Cow::Borrowed("Russian_Game_Italian_Variation"), family: Openings::RussianGame},
        Variation {name: Cow::Borrowed("Russian_Game_Three_Knights_Game"), family: Openings::RussianGame},
        Variation {name: Cow::Borrowed("Russian_Game_Modern_Attack"), family: Openings::RussianGame},
        Variation {name: Cow::Borrowed("Russian_Game_Classical_Attack"), family: Openings::RussianGame},
    ];

    const KINGS_GAMBIT_ACCEPTED_VARIATIONS: [Variation; 8] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::KingsGambitAccepted},
        Variation {name: Cow::Borrowed("Kings_Gambit_Accepted_MacLeod_Defense"), family: Openings::KingsGambitAccepted},
        Variation {name: Cow::Borrowed("Kings_Gambit_Accepted_Fischer_Defense"), family: Openings::KingsGambitAccepted},
        Variation {name: Cow::Borrowed("Kings_Gambit_Accepted_Kings_Knights_Gambit"), family: Openings::KingsGambitAccepted},
        Variation {name: Cow::Borrowed("Kings_Gambit_Accepted_Bishops_Gambit"), family: Openings::KingsGambitAccepted},
        Variation {name: Cow::Borrowed("Kings_Gambit_Accepted_Cunningham_Defense"), family: Openings::KingsGambitAccepted},
        Variation {name: Cow::Borrowed("Kings_Gambit_Accepted_Modern_Defense"), family: Openings::KingsGambitAccepted},
        Variation {name: Cow::Borrowed("Kings_Gambit_Accepted_Schallopp_Defense"), family: Openings::KingsGambitAccepted},
    ];

    const SLAV_DEFENSE_VARIATIONS: [Variation; 5] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::SlavDefense},
        Variation {name: Cow::Borrowed("Slav_Defense_Exchange_Variation"), family: Openings::SlavDefense},
        Variation {name: Cow::Borrowed("Slav_Defense_Three_Knights_Variation"), family: Openings::SlavDefense},
        Variation {name: Cow::Borrowed("Slav_Defense_Modern_Line"), family: Openings::SlavDefense},
        Variation {name: Cow::Borrowed("Slav_Defense_Quiet_Variation"), family: Openings::SlavDefense},
    ];

    const VIENNA_VARIATIONS: [Variation; 4] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::ViennaGame},
        Variation {name: Cow::Borrowed("Vienna_Game_Vienna_Gambit"), family: Openings::ViennaGame},
        Variation {name: Cow::Borrowed("Vienna_Game_Stanley_Variation"), family: Openings::ViennaGame},
        Variation {name: Cow::Borrowed("Vienna_Game_Max_Lange_Defense"), family: Openings::ViennaGame},
    ];

    const QUEENS_GAMBIT_ACCEPTED_VARIATIONS: [Variation; 3] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::QueensGambitAccepted},
        Variation {name: Cow::Borrowed("Queens_Gambit_Accepted_Old_Variation"), family: Openings::QueensGambitAccepted},
        Variation {name: Cow::Borrowed("Queens_Gambit_Accepted_Central_Variation"), family: Openings::QueensGambitAccepted},
    ];

    const KINGS_INDIAN_DEFENSE_VARIATIONS: [Variation; 6] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::KingsIndianDefense},
        Variation {name: Cow::Borrowed("Kings_Indian_Defense_Normal_Variation"), family: Openings::KingsIndianDefense},
        Variation {name: Cow::Borrowed("Kings_Indian_Defense_Orthodox_Variation"), family: Openings::KingsIndianDefense},
        Variation {name: Cow::Borrowed("Kings_Indian_Defense_Fianchetto_Variation"), family: Openings::KingsIndianDefense},
        Variation {name: Cow::Borrowed("Kings_Indian_Defense_Samisch_Variation"), family: Openings::KingsIndianDefense},
        Variation {name: Cow::Borrowed("Kings_Indian_Defense_Four_Pawns_Attack"), family: Openings::KingsIndianDefense},
    ];

    const BENONI_VARIATIONS: [Variation; 5] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::Benoni},
        Variation {name: Cow::Borrowed("Benoni_Defense_Old_Benoni"), family: Openings::Benoni},
        Variation {name: Cow::Borrowed("Benoni_Defense_Benoni_Gambit_Accepted"), family: Openings::Benoni},
        Variation {name: Cow::Borrowed("Benoni_Defense_French_Benoni"), family: Openings::Benoni},
        Variation {name: Cow::Borrowed("Benoni_Defense_Modern_Variation"), family: Openings::Benoni},
    ];

    const KINGS_GAMBIT_DECLINED_VARIATIONS: [Variation; 4] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::KingsGambitDeclined},
        Variation {name: Cow::Borrowed("Kings_Gambit_Declined_Queens_Knight_Defense"), family: Openings::KingsGambitDeclined},
        Variation {name: Cow::Borrowed("Kings_Gambit_Declined_Falkbeer_Countergambit"), family: Openings::KingsGambitDeclined},
        Variation {name: Cow::Borrowed("Kings_Gambit_Declined_Classical_Variation"), family: Openings::KingsGambitDeclined},
    ];

    const GRUNFELD_VARIATIONS: [Variation; 3] = [
        Variation {name: Cow::Borrowed(Variation::ANY_STR), family: Openings::GrunfeldDefense},
        Variation {name: Cow::Borrowed("Grunfeld_Defense_Exchange_Variation"), family: Openings::GrunfeldDefense},
        Variation {name: Cow::Borrowed("Grunfeld_Defense_Three_Knights_Variation"), family: Openings::GrunfeldDefense},
    ];

}

impl DisplayTranslated for Variation {
    fn to_str_tr(&self) -> &str {
        &self.name
    }
}

impl PickListWrapper<Variation> {
    pub fn get_variations(lang: lang::Language, family: Option<&Openings>) -> Vec<PickListWrapper<Variation>> {
        let mut openings_wrapper = Vec::new();
        if let Some(family) = family {
            let variations: Vec<Variation> = match family {
                Openings::Sicilian => Variation::SICILIAN_VARIATIONS.to_vec(),
                Openings::French => Variation::FRENCH_VARIATIONS.to_vec(),
                Openings::QueensPawnGame => Variation::QUEENS_PAWN_GAME_VARIATIONS.to_vec(),
                Openings::ItalianGame => Variation::ITALIAN_VARIATIONS.to_vec(),
                Openings::CaroKann => Variation::CARO_KANN_VARIATIONS.to_vec(),
                Openings::Scandinavian => Variation::SCANDINAVIAN_VARIATIONS.to_vec(),
                Openings::QueensGambitDeclined => Variation::QUEENS_GAMBIT_DECLINED_VARIATIONS.to_vec(),
                Openings::English => Variation::ENGLISH_VARIATIONS.to_vec(),
                Openings::RuyLopez => Variation::RUY_LOPEZ_VARIATIONS.to_vec(),
                Openings::IndianDefense => Variation::INDIAN_DEFENSE_VARIATIONS.to_vec(),
                Openings::ScotchGame => Variation::SCOTCH_GAME_VARIATIONS.to_vec(),
                Openings::RussianGame => Variation::RUSSIAN_GAME_VARIATIONS.to_vec(),
                Openings::KingsGambitAccepted => Variation::KINGS_GAMBIT_ACCEPTED_VARIATIONS.to_vec(),
                Openings::SlavDefense => Variation::SLAV_DEFENSE_VARIATIONS.to_vec(),
                Openings::ViennaGame => Variation::VIENNA_VARIATIONS.to_vec(),
                Openings::QueensGambitAccepted => Variation::QUEENS_GAMBIT_ACCEPTED_VARIATIONS.to_vec(),
                Openings::KingsIndianDefense => Variation::KINGS_INDIAN_DEFENSE_VARIATIONS.to_vec(),
                Openings::Benoni => Variation::BENONI_VARIATIONS.to_vec(),
                Openings::KingsGambitDeclined => Variation::KINGS_GAMBIT_DECLINED_VARIATIONS.to_vec(),
                Openings::GrunfeldDefense => Variation::GRUNFELD_VARIATIONS.to_vec(),
                _ => vec![Variation::ANY],
            };
            for variation in variations {
                openings_wrapper.push(
                    PickListWrapper::<Variation> {
                        lang: lang,
                        item: variation,
                    }
                );
            }
        }
        openings_wrapper
    }

    pub fn new_variation(lang: lang::Language, var: Variation) -> Self {
        Self {
            lang: lang,
            item: var,
        }
    }
}
