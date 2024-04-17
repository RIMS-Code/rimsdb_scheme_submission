#![warn(clippy::all, rust_2018_idioms)]

use strum_macros::EnumIter;

mod app;

pub use app::TemplateApp;

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, EnumIter)]
pub enum Elements {
    H,
    He,
    Li,
    Be,
    B,
    C,
    N,
    O,
    F,
    Ne,
    Na,
    Mg,
    Al,
    Si,
    P,
    S,
    Cl,
    Ar,
    K,
    Ca,
    Sc,
    Ti,
    V,
    Cr,
    Mn,
    Fe,
    Co,
    Ni,
    Cu,
    Zn,
    Ga,
    Ge,
    As,
    Se,
    Br,
    Kr,
    Rb,
    Sr,
    Y,
    Zr,
    Nb,
    Mo,
    Tc,
    Ru,
    Rh,
    Pd,
    Ag,
    Cd,
    In,
    Sn,
    Sb,
    Te,
    I,
    Xe,
    Cs,
    Ba,
    La,
    Ce,
    Pr,
    Nd,
    Pm,
    Sm,
    Eu,
    Gd,
    Tb,
    Dy,
    Ho,
    Er,
    Tm,
    Yb,
    Lu,
    Hf,
    Ta,
    W,
    Re,
    Os,
    Ir,
    Pt,
    Au,
    Hg,
    Tl,
    Pb,
    Bi,
    Po,
    At,
    Rn,
    Fr,
    Ra,
    Ac,
    Th,
    Pa,
    U,
    Np,
    Pu,
    Am,
    Cm,
    Bk,
    Cf,
    Es,
    Fm,
    Md,
    No,
    Lr,
    Rf,
    Db,
    Sg,
    Bh,
    Hs,
}

impl Elements {
    pub fn ip(&self) -> f64 {
        match self {
            Elements::H => 109678.77174307,
            Elements::He => 198310.66637,
            Elements::Li => 43487.1142,
            Elements::Be => 75192.64,
            Elements::B => 66928.04,
            Elements::C => 90820.348,
            Elements::N => 117225.7,
            Elements::O => 109837.02,
            Elements::F => 140524.5,
            Elements::Ne => 173929.75,
            Elements::Na => 41449.451,
            Elements::Mg => 61671.05,
            Elements::Al => 48278.48,
            Elements::Si => 65747.76,
            Elements::P => 84580.83,
            Elements::S => 83559.1,
            Elements::Cl => 104591.01,
            Elements::Ar => 127109.842,
            Elements::K => 35009.814,
            Elements::Ca => 49305.924,
            Elements::Sc => 52922.0,
            Elements::Ti => 55072.5,
            Elements::V => 54411.67,
            Elements::Cr => 54575.6,
            Elements::Mn => 59959.56,
            Elements::Fe => 63737.704,
            Elements::Co => 63564.6,
            Elements::Ni => 61619.77,
            Elements::Cu => 62317.46,
            Elements::Zn => 75769.31,
            Elements::Ga => 48387.634,
            Elements::Ge => 63713.24,
            Elements::As => 78950.0,
            Elements::Se => 78658.15,
            Elements::Br => 95284.8,
            Elements::Kr => 112914.433,
            Elements::Rb => 33690.81,
            Elements::Sr => 45932.2036,
            Elements::Y => 50145.6,
            Elements::Zr => 53507.832,
            Elements::Nb => 54513.8,
            Elements::Mo => 57204.3,
            Elements::Tc => 57421.68,
            Elements::Ru => 59366.4,
            Elements::Rh => 60160.1,
            Elements::Pd => 67241.14,
            Elements::Ag => 61106.45,
            Elements::Cd => 72540.05,
            Elements::In => 46670.107,
            Elements::Sn => 59232.69,
            Elements::Sb => 69431.34,
            Elements::Te => 72669.006,
            Elements::I => 84294.9,
            Elements::Xe => 97833.787,
            Elements::Cs => 31406.4677325,
            Elements::Ba => 42034.91,
            Elements::La => 44981.0,
            Elements::Ce => 44672.0,
            Elements::Pr => 44120.0,
            Elements::Nd => 44562.0,
            Elements::Pm => 45020.8,
            Elements::Sm => 45519.69,
            Elements::Eu => 45734.74,
            Elements::Gd => 49601.45,
            Elements::Tb => 47295.0,
            Elements::Dy => 47901.76,
            Elements::Ho => 48567.0,
            Elements::Er => 49262.0,
            Elements::Tm => 49880.57,
            Elements::Yb => 50443.2,
            Elements::Lu => 43762.6,
            Elements::Hf => 55047.9,
            Elements::Ta => 60891.4,
            Elements::W => 63427.7,
            Elements::Re => 63181.6,
            Elements::Os => 68058.9,
            Elements::Ir => 72323.9,
            Elements::Pt => 72257.8,
            Elements::Au => 74409.11,
            Elements::Hg => 84184.15,
            Elements::Tl => 49266.66,
            Elements::Pb => 59819.558,
            Elements::Bi => 58761.65,
            Elements::Po => 67896.31,
            Elements::At => 75150.8,
            Elements::Rn => 86692.5,
            Elements::Fr => 32848.872,
            Elements::Ra => 42573.36,
            Elements::Ac => 43394.52,
            Elements::Th => 50867.0,
            Elements::Pa => 47500.0,
            Elements::U => 49958.4,
            Elements::Np => 50535.0,
            Elements::Pu => 48601.0,
            Elements::Am => 48182.0,
            Elements::Cm => 48330.68,
            Elements::Bk => 49989.0,
            Elements::Cf => 50666.76,
            Elements::Es => 51364.58,
            Elements::Fm => 52400.0,
            Elements::Md => 53100.0,
            Elements::No => 53444.0,
            Elements::Lr => 40005.0,
            Elements::Rf => 48580.0,
            Elements::Db => 55000.0,
            Elements::Sg => 63000.0,
            Elements::Bh => 62000.0,
            Elements::Hs => 61000.0,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GroundState {
    pub level: String,
    pub term_symbol: String,
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Lasers {
    TiSa,
    Dye,
    Both,
}

impl Lasers {
    pub fn to_string(&self) -> String {
        match self {
            Lasers::TiSa => "Ti:Sa".to_string(),
            Lasers::Dye => "Dye".to_string(),
            Lasers::Both => "Ti:Sa and Dye".to_string(),
        }
    }
}

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TransitionUnit {
    NM,
    CM1,
}

impl TransitionUnit {
    pub fn to_string(&self) -> String {
        match self {
            TransitionUnit::NM => "nm".to_string(),
            TransitionUnit::CM1 => "1/cm".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum SaturationCurveUnit {
    WCM2,
    W,
}

impl SaturationCurveUnit {
    pub fn to_string(&self) -> String {
        match self {
            SaturationCurveUnit::WCM2 => "W/cm²".to_string(),
            SaturationCurveUnit::W => "W".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SaturationCurve {
    pub title: String,
    pub notes: String,
    pub units: SaturationCurveUnit,
    pub xdat: Vec<f64>,
    pub xdat_unc: Option<Vec<f64>>,
    pub ydat: Vec<f64>,
    pub ydat_unc: Option<Vec<f64>>,
}

impl SaturationCurve {
    /// Create a new instance of Saturation curve or return an error if not successful.
    pub fn new_from_parts(
        title: &String,
        notes: &String,
        units: &SaturationCurveUnit,
        xdat: &String,
        xunc: &String,
        ydat: &String,
        yunc: &String,
    ) -> Result<Self, String> {
        Ok(Self {
            title: title.to_owned(),
            notes: notes.to_owned(),
            units: SaturationCurveUnit::W,
            xdat: Vec::new(),
            xdat_unc: None,
            ydat: Vec::new(),
            ydat_unc: None,
        })
    }
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Transition {
    pub level: String,
    pub term_symbol: String,
    pub transition_strength: String,
    pub low_lying: bool,
    pub forbidden: bool,
}

impl Transition {
    fn new_empty() -> Self {
        Self {
            level: String::new(),
            term_symbol: String::new(),
            transition_strength: String::new(),
            low_lying: false,
            forbidden: false,
        }
    }
}
