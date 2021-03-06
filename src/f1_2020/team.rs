#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Team {
    Mercedes,
    Ferrari,
    RedBullRacing,
    Williams,
    RacingPoint,
    Renault,
    ToroRosso,
    Haas,
    McLaren,
    AlfaRomeo,
    McLaren1988,
    McLaren1991,
    Williams1992,
    Ferrari1995,
    Williams1996,
    McLaren1998,
    Ferrari2002,
    Ferrari2004,
    Renault2006,
    Ferrari2007,
    RedBull2010,
    Ferrari1976,
    ARTGrandPrix,
    CamposVexatecRacing,
    Carlin,
    CharouzRacingSystem,
    DAMS,
    RussianTime,
    MPMotorsport,
    Pertamina,
    McLaren1990,
    Trident,
    BWTArden,
    McLaren1976,
    Lotus1972,
    Ferrari1979,
    McLaren1982,
    Williams2003,
    Brawn2009,
    Lotus1978,
    ArtGP2019,
    Campos2019,
    Carlin2019,
    SauberJuniorCharouz2019,
    Dams2019,
    UniVirtuosi2019,
    MPMotorsport2019,
    Prema2019,
    Trident2019,
    Arden2019,
    Ferrari1990,
    McLaren2010,
    Ferrari2010,
    AlphaTauri,
    McLaren2008,
    F1GenericCar,
    Benetton1994,
    Benetton1995,
    Ferrari2000,
    Jordan1991,
    MyTeam,
    Unknown,
}

impl Team {
    pub fn name(self) -> &'static str {
        match self {
            Team::Mercedes => "Mercedes",
            Team::Ferrari => "Ferrari",
            Team::RedBullRacing => "Red Bull Racing",
            Team::RacingPoint => "Racing Point",
            Team::Renault => "Renault",
            Team::ToroRosso => "Toro Rosso",
            Team::Haas => "Haas",
            Team::McLaren => "McLaren",
            Team::AlfaRomeo => "Alfa Romeo",
            Team::AlphaTauri => "Alpha Tauri",
            Team::Williams => "Williams",
            _ => "[N/A]",
        }
    }

    pub fn hex_colors(self) -> &'static str {
        match self {
            Team::Mercedes => "#00D2BE",
            Team::Ferrari => "#C00000",
            Team::RedBullRacing => "#0600EF",
            Team::RacingPoint => "#F596C8",
            Team::Renault => "#FFF500",
            Team::Haas => "#787878",
            Team::McLaren => "#FF8700",
            Team::AlfaRomeo => "#960000",
            Team::AlphaTauri => "#C8C8C8",
            Team::Williams => "#0082FA",
            _ => "#000000",
        }
    }

    pub fn rgb(self) -> (u8, u8, u8) {
        match self {
            Team::Mercedes => (0, 210, 190),
            Team::Ferrari => (192, 0, 0),
            Team::RedBullRacing => (6, 0, 239),
            Team::RacingPoint => (245, 150, 200),
            Team::Renault => (255, 245, 0),
            Team::Haas => (120, 120, 120),
            Team::McLaren => (255, 135, 0),
            Team::AlfaRomeo => (150, 0, 0),
            Team::AlphaTauri => (200, 200, 200),
            Team::Williams => (0, 130, 250),
            _ => (0, 0, 0),
        }
    }
}
