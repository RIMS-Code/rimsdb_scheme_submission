use std::borrow::ToOwned;
use std::fmt;
use std::str::FromStr;

use serde_json::{json, ser::to_string_pretty, Value};
use strum_macros::EnumIter;

mod app;

pub use app::TemplateApp;

const DB_MAINTAINER_EMAIL: &str = "reto@galactic-forensics.space";

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

impl FromStr for Elements {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "H" => Ok(Elements::H),
            "He" => Ok(Elements::He),
            "Li" => Ok(Elements::Li),
            "Be" => Ok(Elements::Be),
            "B" => Ok(Elements::B),
            "C" => Ok(Elements::C),
            "N" => Ok(Elements::N),
            "O" => Ok(Elements::O),
            "F" => Ok(Elements::F),
            "Ne" => Ok(Elements::Ne),
            "Na" => Ok(Elements::Na),
            "Mg" => Ok(Elements::Mg),
            "Al" => Ok(Elements::Al),
            "Si" => Ok(Elements::Si),
            "P" => Ok(Elements::P),
            "S" => Ok(Elements::S),
            "Cl" => Ok(Elements::Cl),
            "Ar" => Ok(Elements::Ar),
            "K" => Ok(Elements::K),
            "Ca" => Ok(Elements::Ca),
            "Sc" => Ok(Elements::Sc),
            "Ti" => Ok(Elements::Ti),
            "V" => Ok(Elements::V),
            "Cr" => Ok(Elements::Cr),
            "Mn" => Ok(Elements::Mn),
            "Fe" => Ok(Elements::Fe),
            "Co" => Ok(Elements::Co),
            "Ni" => Ok(Elements::Ni),
            "Cu" => Ok(Elements::Cu),
            "Zn" => Ok(Elements::Zn),
            "Ga" => Ok(Elements::Ga),
            "Ge" => Ok(Elements::Ge),
            "As" => Ok(Elements::As),
            "Se" => Ok(Elements::Se),
            "Br" => Ok(Elements::Br),
            "Kr" => Ok(Elements::Kr),
            "Rb" => Ok(Elements::Rb),
            "Sr" => Ok(Elements::Sr),
            "Y" => Ok(Elements::Y),
            "Zr" => Ok(Elements::Zr),
            "Nb" => Ok(Elements::Nb),
            "Mo" => Ok(Elements::Mo),
            "Tc" => Ok(Elements::Tc),
            "Ru" => Ok(Elements::Ru),
            "Rh" => Ok(Elements::Rh),
            "Pd" => Ok(Elements::Pd),
            "Ag" => Ok(Elements::Ag),
            "Cd" => Ok(Elements::Cd),
            "In" => Ok(Elements::In),
            "Sn" => Ok(Elements::Sn),
            "Sb" => Ok(Elements::Sb),
            "Te" => Ok(Elements::Te),
            "I" => Ok(Elements::I),
            "Xe" => Ok(Elements::Xe),
            "Cs" => Ok(Elements::Cs),
            "Ba" => Ok(Elements::Ba),
            "La" => Ok(Elements::La),
            "Ce" => Ok(Elements::Ce),
            "Pr" => Ok(Elements::Pr),
            "Nd" => Ok(Elements::Nd),
            "Pm" => Ok(Elements::Pm),
            "Sm" => Ok(Elements::Sm),
            "Eu" => Ok(Elements::Eu),
            "Gd" => Ok(Elements::Gd),
            "Tb" => Ok(Elements::Tb),
            "Dy" => Ok(Elements::Dy),
            "Ho" => Ok(Elements::Ho),
            "Er" => Ok(Elements::Er),
            "Tm" => Ok(Elements::Tm),
            "Yb" => Ok(Elements::Yb),
            "Lu" => Ok(Elements::Lu),
            "Hf" => Ok(Elements::Hf),
            "Ta" => Ok(Elements::Ta),
            "W" => Ok(Elements::W),
            "Re" => Ok(Elements::Re),
            "Os" => Ok(Elements::Os),
            "Ir" => Ok(Elements::Ir),
            "Pt" => Ok(Elements::Pt),
            "Au" => Ok(Elements::Au),
            "Hg" => Ok(Elements::Hg),
            "Tl" => Ok(Elements::Tl),
            "Pb" => Ok(Elements::Pb),
            "Bi" => Ok(Elements::Bi),
            "Po" => Ok(Elements::Po),
            "At" => Ok(Elements::At),
            "Rn" => Ok(Elements::Rn),
            "Fr" => Ok(Elements::Fr),
            "Ra" => Ok(Elements::Ra),
            "Ac" => Ok(Elements::Ac),
            "Th" => Ok(Elements::Th),
            "Pa" => Ok(Elements::Pa),
            "U" => Ok(Elements::U),
            "Np" => Ok(Elements::Np),
            "Pu" => Ok(Elements::Pu),
            "Am" => Ok(Elements::Am),
            "Cm" => Ok(Elements::Cm),
            "Bk" => Ok(Elements::Bk),
            "Cf" => Ok(Elements::Cf),
            "Es" => Ok(Elements::Es),
            "Fm" => Ok(Elements::Fm),
            "Md" => Ok(Elements::Md),
            "No" => Ok(Elements::No),
            "Lr" => Ok(Elements::Lr),
            "Rf" => Ok(Elements::Rf),
            "Db" => Ok(Elements::Db),
            "Sg" => Ok(Elements::Sg),
            "Bh" => Ok(Elements::Bh),
            "Hs" => Ok(Elements::Hs),
            _ => Err(format!("Element {} not found.", s).to_owned()),
        }
    }
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
            Elements::Pa => 49034.0,
            Elements::U => 49958.4,
            Elements::Np => 50535.0,
            Elements::Pu => 48601.0,
            Elements::Am => 48182.0,
            Elements::Cm => 48330.68,
            Elements::Bk => 49989.0,
            Elements::Cf => 50666.76,
            Elements::Es => 51364.58,
            Elements::Fm => 52422.5,
            Elements::Md => 53230.0,
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

impl GroundState {
    fn get_level(&self) -> Result<String, String> {
        if self.level.is_empty() {
            return Err("Ground state level is empty.".to_owned());
        }

        match self.level.parse::<f64>() {
            Ok(_) => (),
            Err(_) => return Err("Ground state level is not a number.".to_owned()),
        }

        Ok(self.level.clone())
    }
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Lasers {
    TiSa,
    Dye,
    Both,
}

impl fmt::Display for Lasers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lasers::TiSa => write!(f, "Ti:Sa"),
            Lasers::Dye => write!(f, "Dye"),
            Lasers::Both => write!(f, "Ti:Sa and Dye"),
        }
    }
}

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TransitionUnit {
    NM,
    CM1,
}

impl fmt::Display for TransitionUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransitionUnit::NM => write!(f, "nm"),
            TransitionUnit::CM1 => write!(f, "cm¯¹"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum SaturationCurveUnit {
    WCM2,
    W,
}

impl fmt::Display for SaturationCurveUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SaturationCurveUnit::WCM2 => write!(f, "W/cm²"),
            SaturationCurveUnit::W => write!(f, "W"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SaturationCurve {
    pub title: String,
    pub notes: String,
    pub units: SaturationCurveUnit,
    pub fit: bool,
    pub xdat: Vec<f64>,
    pub xdat_unc: Option<Vec<f64>>,
    pub ydat: Vec<f64>,
    pub ydat_unc: Option<Vec<f64>>,
}

impl SaturationCurve {
    /// Create a new instance of Saturation curve or return an error if not successful.
    #[allow(clippy::too_many_arguments)]
    pub fn new_from_parts(
        title: &str,
        notes: &str,
        units: &SaturationCurveUnit,
        fit: bool,
        xdat: &str,
        xunc: &str,
        ydat: &str,
        yunc: &str,
    ) -> Result<Self, String> {
        if xdat.is_empty() {
            return Err("Please enter some data.".to_string());
        }

        let xdat = data_string_to_vec_f64(xdat, "x")?;
        let ydat = data_string_to_vec_f64(ydat, "y")?;

        if xdat.len() != ydat.len() {
            return Err("X and Y data lengths do not match.".to_string());
        }

        let xdat_unc: Option<Vec<f64>> = if xunc.is_empty() {
            None
        } else {
            Some(data_string_to_vec_f64(xunc, "x uncertainty")?)
        };

        if xdat_unc.is_some() && xdat_unc.as_ref().unwrap().len() != xdat.len() {
            return Err("Uncertainty x data length does not match the x data length.".to_string());
        }

        let ydat_unc: Option<Vec<f64>> = if yunc.is_empty() {
            None
        } else {
            Some(data_string_to_vec_f64(yunc, "y uncertainty")?)
        };

        if ydat_unc.is_some() && ydat_unc.as_ref().unwrap().len() != ydat.len() {
            return Err("Uncertainty y data length does not match the y data length.".to_string());
        }

        Ok(Self {
            title: title.to_owned(),
            notes: notes.to_owned(),
            units: units.clone(),
            fit,
            xdat,
            xdat_unc,
            ydat,
            ydat_unc,
        })
    }

    pub fn get_xdat(&self) -> String {
        self.xdat
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn get_xdat_unc(&self) -> String {
        if let Some(xdat_unc) = &self.xdat_unc {
            xdat_unc
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        } else {
            String::new()
        }
    }

    pub fn get_ydat(&self) -> String {
        self.ydat
            .iter()
            .map(|y| y.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn get_ydat_unc(&self) -> String {
        if let Some(ydat_unc) = &self.ydat_unc {
            ydat_unc
                .iter()
                .map(|y| y.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        } else {
            String::new()
        }
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

    fn get_level(&self) -> Result<String, String> {
        if self.level.is_empty() {
            return Ok("".to_owned());
        }

        match self.level.parse::<f64>() {
            Ok(_) => (),
            Err(_) => return Err("Transition level is not a number.".to_owned()),
        }

        Ok(self.level.clone())
    }

    fn get_transition_strength(&self) -> Result<String, String> {
        if self.transition_strength.is_empty() {
            return Ok("".to_owned());
        }

        match self.transition_strength.parse::<f64>() {
            Ok(_) => (),
            Err(_) => return Err("Transition strength is not a number.".to_owned()),
        }

        Ok(self.transition_strength.clone())
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ReferenceEntry {
    id: String,
    authors: String,
    year: usize,
}

impl ReferenceEntry {
    fn new_from_doi(doi: &str) -> Self {
        ReferenceEntry {
            id: doi.into(),
            authors: String::new(),
            year: 0,
        }
    }

    fn new_from_url(url: &str, authors: &str, year: usize) -> Self {
        ReferenceEntry {
            id: url.into(),
            authors: authors.into(),
            year,
        }
    }
    
    fn get_url(&self) -> String {
        if self.authors.is_empty() && self.year == 0 {
            format!("https://doi.org/{}", self.id)
        } else {
            self.id.clone()
        }
    }
    
}

/// Create email content and link and fill it
fn create_email_link(body: &str, element: &Elements) -> String {
    let newline = "%0D%0A";
    let spacer =
        "\n\n======= SCHEME FILE: DO NOT EDIT BELOW THIS LINE =======\n\n".replace('\n', newline);

    let title = format!("Scheme submission: {:?}", element);

    format!(
        "mailto:{}?subject={}&body={}{}",
        DB_MAINTAINER_EMAIL,
        title,
        spacer,
        body.replace('\n', newline)
    )
}

/// Create GitHub issue link that will automatically open the issue submission and fill it.
fn create_gh_issue(body: &str, element: &Elements) -> String {
    let title = format!("Scheme submission: {:?}", element);
    let label = "scheme_submission";

    format!(
        "https://github.com/RIMS-Code/rims-code.github.io/issues/new?labels={}&title={}&body={}",
        label, title, body
    )
}

/// Create a JSON output string from the input data in the mask.
fn create_json_output(app_entries: &TemplateApp) -> Result<String, String> {
    let scheme_unit_json = match app_entries.scheme_unit {
        TransitionUnit::NM => "nm",
        TransitionUnit::CM1 => "cm<sup>-1</sup>",
    };

    // error checking
    if app_entries.scheme_transitions[0].level.is_empty() {
        return Err("No transitions entered: Please add at least one step.".into());
    } else if app_entries.submitted_by.is_empty() {
        return Err("Please enter your name.".into());
    }

    // create the json file
    let mut json_out = json!({
        "notes": app_entries.notes,
        "rims_scheme": {
            "scheme": {
                "element": format!("{:?}", app_entries.scheme_element),
                "lasers": app_entries.scheme_lasers.to_string(),
                "last_step_to_ip": app_entries.scheme_last_step_to_ip,
                "gs_term": app_entries.scheme_gs.term_symbol,
                "gs_level": app_entries.scheme_gs.get_level()?,
                "ip_term": app_entries.scheme_ip_term_symbol,
                "unit": scheme_unit_json,
            },
        },
        "references": app_entries.references,
        "submitted_by": app_entries.submitted_by,
    });

    for (it, val) in app_entries.scheme_transitions.iter().enumerate() {
        let level = val.get_level()?;
        if !level.is_empty() {
            json_out["rims_scheme"]["scheme"][format!("step_level{}", it)] = Value::from(level);
            json_out["rims_scheme"]["scheme"][format!("step_term{}", it)] =
                Value::from(val.term_symbol.clone());
            json_out["rims_scheme"]["scheme"][format!("trans_strength{}", it)] =
                Value::from(val.get_transition_strength()?);
            json_out["rims_scheme"]["scheme"][format!("step_forbidden{}", it)] =
                Value::from(val.forbidden);
            json_out["rims_scheme"]["scheme"][format!("step_lowlying{}", it)] =
                Value::from(val.low_lying);
        }
    }

    let mut sat_curves_arr: Vec<Value> = Vec::new();
    for val in app_entries.saturation_curves.iter() {
        let sat_unit_json = match val.units {
            SaturationCurveUnit::WCM2 => "W * cm^-2",
            SaturationCurveUnit::W => "W",
        };

        let mut json_tmp = json!({
            "title": val.title,
            "notes": val.notes,
            "unit": sat_unit_json,
            "fit": val.fit,
            "data": {
                "x": val.xdat,
                "y": val.ydat,
            }
        });
        match &val.xdat_unc {
            Some(x) => json_tmp["data"]["x_err"] = Value::from(x.clone()),
            None => (),
        }
        match &val.ydat_unc {
            Some(y) => json_tmp["data"]["y_err"] = Value::from(y.clone()),
            None => (),
        }

        sat_curves_arr.push(json_tmp);
    }
    json_out["saturation_curves"] = Value::from(sat_curves_arr);

    match to_string_pretty(&json_out) {
        Ok(json) => Ok(json),
        Err(e) => Err(format!("Error creating JSON output: {}", e).to_string()),
    }
}

/// Take a data string and transfer it to a f64 Vector.
fn data_string_to_vec_f64(data: &str, name: &str) -> Result<Vec<f64>, String> {
    let mut x_data: Vec<f64> = Vec::new();
    let xsplit = split_data_string(data);
    for xd in xsplit {
        match xd.parse::<f64>() {
            Ok(x) => x_data.push(x),
            Err(_) => return Err(format!("None-numeric value fount in {} data.", name).to_string()),
        };
    }
    Ok(x_data)
}

fn load_config_file(app_entries: &mut TemplateApp) -> Result<(), String> {
    let config_json: Value = match serde_json::from_str(&app_entries.rimsschemedrawer_in) {
        Ok(json) => json,
        Err(e) => return Err(format!("Error reading JSON file: {}", e).to_string()),
    };

    // Let's check what file we have here by looking for the "rims_scheme" key.
    let scheme;
    if config_json["rims_scheme"].is_object() {
        scheme = config_json["rims_scheme"]["scheme"].clone();
    } else if config_json["scheme"].is_object() {
        scheme = config_json["scheme"].clone();
    } else {
        return Err("No 'rims_scheme' or 'scheme' key found in the JSON file.".to_string());
    }

    // Load the scheme data
    app_entries.scheme_element = match scheme["element"].as_str() {
        Some(e) => e.parse::<Elements>()?,
        None => return Err("No element found in the JSON file.".to_string()),
    };

    app_entries.scheme_gs = GroundState {
        level: scheme["gs_level"].as_str().unwrap_or("0").to_owned(),
        term_symbol: scheme["gs_term"].as_str().unwrap_or("").to_owned(),
    };

    app_entries.scheme_ip_term_symbol = scheme["ip_term"].as_str().unwrap_or("").into();

    app_entries.scheme_lasers = match scheme["lasers"].as_str() {
        Some(l) => match l {
            "Ti:Sa" => Lasers::TiSa,
            "Dye" => Lasers::Dye,
            "Ti:Sa and Dye" => Lasers::Both,
            _ => return Err("Laser type not found.".to_string()),
        },
        None => return Err("No laser type found in the JSON file.".to_string()),
    };

    app_entries.scheme_last_step_to_ip = scheme["last_step_to_ip"].as_bool().unwrap_or(false);

    let mut scheme_transitions: [Transition; 7] = [
        Transition::new_empty(),
        Transition::new_empty(),
        Transition::new_empty(),
        Transition::new_empty(),
        Transition::new_empty(),
        Transition::new_empty(),
        Transition::new_empty(),
    ];

    for it in 0..scheme_transitions.len() {
        if !scheme[format!("step_level{}", it)].is_null() {
            scheme_transitions[it] = Transition {
                level: scheme[format!("step_level{}", it)]
                    .as_str()
                    .unwrap_or("")
                    .to_owned(),
                term_symbol: scheme[format!("step_term{}", it)]
                    .as_str()
                    .unwrap_or("")
                    .to_owned(),
                transition_strength: scheme[format!("trans_strength{}", it)]
                    .as_str()
                    .unwrap_or("")
                    .to_owned(),
                low_lying: scheme[format!("step_lowlying{}", it)]
                    .as_bool()
                    .unwrap_or(false),
                forbidden: scheme[format!("step_forbidden{}", it)]
                    .as_bool()
                    .unwrap_or(false),
            };
        }
    }
    app_entries.scheme_transitions = scheme_transitions;

    app_entries.scheme_unit = match scheme["unit"].as_str() {
        Some(u) => match u {
            "nm" => TransitionUnit::NM,
            _ => TransitionUnit::CM1,
        },
        None => return Err("No unit found in the JSON file.".to_string()),
    };

    // Load Notes if they are there
    app_entries.notes = config_json["notes"].as_str().unwrap_or("").into();

    // Load References if they are there
    let refs = config_json["references"].as_array();
    let mut references: Vec<ReferenceEntry> = Vec::new();
    if let Some(refs) = refs {
        for r in refs {
            let rid = match r["id"].as_str() {
                Some(s) => s,
                None => continue,
            };
            let rauth = r["authors"].as_str().unwrap_or("");
            let ryear = match r["year"].as_u64() {
                Some(y) => y as usize,
                None => 0_usize,
            };

            if rauth.is_empty() && ryear == 0 {
                references.push(ReferenceEntry::new_from_doi(rid))
            } else {
                references.push(ReferenceEntry::new_from_url(rid, rauth, ryear))
            }
        }
    };
    app_entries.references = references;

    // Load saturation curves if they are there
    let sats = config_json["saturation_curves"].as_array();
    let mut saturation_curves: Vec<SaturationCurve> = Vec::new();
    if let Some(sats) = sats {
        for sat in sats {
            let title = match sat["title"].as_str() {
                Some(t) => t.to_owned(),
                None => continue,
            };
            let notes = sat["notes"].as_str().unwrap_or("").to_owned();
            let units = match sat["unit"].as_str() {
                Some("W") => SaturationCurveUnit::W,
                _ => SaturationCurveUnit::WCM2,
            };
            let fit = sat["fit"].as_bool().unwrap_or(true);
            let xdat = match sat["data"]["x"].as_array() {
                Some(x) => json_array_to_f64(x)?,
                None => {
                    return Err(
                        format!("No saturation x data found in the JSON file for {title}.")
                            .to_string(),
                    )
                }
            };
            let ydat = match sat["data"]["y"].as_array() {
                Some(y) => json_array_to_f64(y)?,
                None => {
                    return Err(
                        format!("No saturation y data found in the JSON file for {title}.")
                            .to_string(),
                    )
                }
            };
            let xunc = match sat["data"]["x_err"].as_array() {
                Some(x) => Some(json_array_to_f64(x)?),
                None => None,
            };
            let yunc = match sat["data"]["y_err"].as_array() {
                Some(y) => Some(json_array_to_f64(y)?),
                None => None,
            };
            saturation_curves.push(SaturationCurve {
                title,
                notes,
                units,
                fit,
                xdat,
                xdat_unc: xunc,
                ydat,
                ydat_unc: yunc,
            });
        }
    }
    app_entries.saturation_curves = saturation_curves;

    // Load Notes if they are there
    app_entries.submitted_by = config_json["submitted_by"].as_str().unwrap_or("").into();

    Ok(())
}

/// Take a data string and split it into a vector of strings according to a list of delimiters.
fn split_data_string(data: &str) -> Vec<String> {
    let xsplit: Vec<String> = data
        .split(&[' ', ',', ':', '-', '\t', '\n', '\r'])
        .filter(|&r| !r.is_empty())
        .map(|r| r.to_string())
        .collect();
    xsplit
}

fn json_array_to_f64(data: &Vec<Value>) -> Result<Vec<f64>, String> {
    let mut x_data: Vec<f64> = Vec::new();
    for xd in data {
        let xf64 = xd.as_f64();
        match xf64 {
            Some(x) => x_data.push(x),
            None => return Err("None-numeric value fount in data.".to_string()),
        };
    }
    Ok(x_data)
}

/// Check if a given string is a doi or not.
/// DOIs contain one slash.
fn is_doi(inp: &str) -> bool {
    inp.chars().filter(|c| *c == '/').count() == 1
}

#[cfg(test)]
#[test]
fn test_parse_element() {
    assert_eq!(Elements::from_str("H").unwrap(), Elements::H);
    assert_eq!(Elements::from_str("He").unwrap(), Elements::He);
    assert_eq!(Elements::from_str("Li").unwrap(), Elements::Li);
    assert_eq!(Elements::from_str("Be").unwrap(), Elements::Be);
    assert_eq!(Elements::from_str("B").unwrap(), Elements::B);
    assert_eq!(Elements::from_str("C").unwrap(), Elements::C);
    assert_eq!(Elements::from_str("N").unwrap(), Elements::N);
    assert_eq!(Elements::from_str("O").unwrap(), Elements::O);
    assert_eq!(Elements::from_str("F").unwrap(), Elements::F);
    assert_eq!(Elements::from_str("Ne").unwrap(), Elements::Ne);
    assert_eq!(Elements::from_str("Na").unwrap(), Elements::Na);
    assert_eq!(Elements::from_str("Mg").unwrap(), Elements::Mg);
    assert_eq!(Elements::from_str("Al").unwrap(), Elements::Al);
    assert_eq!(Elements::from_str("Si").unwrap(), Elements::Si);
    assert_eq!(Elements::from_str("P").unwrap(), Elements::P);
    assert_eq!(Elements::from_str("S").unwrap(), Elements::S);
    assert_eq!(Elements::from_str("Cl").unwrap(), Elements::Cl);
    assert_eq!(Elements::from_str("Ar").unwrap(), Elements::Ar);
    assert_eq!(Elements::from_str("K").unwrap(), Elements::K);
    assert_eq!(Elements::from_str("Ca").unwrap(), Elements::Ca);
    assert_eq!(Elements::from_str("Sc").unwrap(), Elements::Sc);
    assert_eq!(Elements::from_str("Ti").unwrap(), Elements::Ti);
    assert_eq!(Elements::from_str("V").unwrap(), Elements::V);
    assert_eq!(Elements::from_str("Cr").unwrap(), Elements::Cr);
    assert_eq!(Elements::from_str("Mn").unwrap(), Elements::Mn);
    assert_eq!(Elements::from_str("Fe").unwrap(), Elements::Fe);
    assert_eq!(Elements::from_str("Co").unwrap(), Elements::Co);
    assert_eq!(Elements::from_str("Ni").unwrap(), Elements::Ni);
    assert_eq!(Elements::from_str("Cu").unwrap(), Elements::Cu);
}

#[test]
fn test_is_doi() {
    let doi_good = "10.500/123456789";
    assert!(is_doi(doi_good));
    let doi_bad = "https://doi.org/10.500/123456789";
    assert!(!is_doi(doi_bad));
}
