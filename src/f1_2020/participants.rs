use crate::f1_2020::car::TOTAL_CARS;
use crate::f1_2020::driver::Driver;
use crate::f1_2020::header::PacketHeader;
use crate::f1_2020::nationality::Nationality;
use crate::f1_2020::team::Team;
use async_std::io::{Cursor, Error, ErrorKind};
use byteorder_async::ReaderToByteOrder;

const PARTICIPANTS_MIN_SIZE: usize = 1213;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum YourTelemetry {
    Restricted, //0
    Public,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ParticipantData {
    // Whether the vehicle is AI (1) or Human (0) controlled
    pub ai_controlled: bool, // u8,
    // Driver
    pub driver: Driver, //u8,
    // Team
    pub team: Team, //u8,
    // Race number of the car
    pub race_number: u8,
    // Nationality of the driver
    pub nationality: Nationality, // u8,
    // Name of participant in UTF-8 format – null terminated
    // Will be truncated with … (U+2026) if too long
    pub name: String,
    // The player's UDP setting, 0 = restricted, 1 = public
    pub your_telemetry: YourTelemetry, // u8,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PacketParticipantsData {
    // Header
    pub header: PacketHeader,
    // Number of active cars in the data – should match number of
    // cars on HUD
    pub num_active_cars: u8,
    pub participants: Vec<ParticipantData>,
}

pub async fn parse_participants_data(
    cursor: &mut Cursor<Vec<u8>>,
    header: PacketHeader,
    size: usize,
) -> Result<PacketParticipantsData, Error> {
    ensure_participants_size(size)?;

    let num_active_cars = cursor.byte_order().read_u8().await?;

    let mut participants = Vec::with_capacity(TOTAL_CARS);
    for _ in 0..TOTAL_CARS {
        let participant = parse_participant(cursor).await?;
        participants.push(participant);
    }

    Ok(PacketParticipantsData {
        header,
        num_active_cars,
        participants,
    })
}

fn ensure_participants_size(size: usize) -> Result<(), Error> {
    if size < PARTICIPANTS_MIN_SIZE {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Participants size is too small",
        ));
    }

    return Ok(());
}

pub fn parse_your_telemetry(value: u8) -> Result<YourTelemetry, Error> {
    match value {
        0 => Ok(YourTelemetry::Restricted),
        1 => Ok(YourTelemetry::Public),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid telemetry")),
    }
}

async fn parse_participant(cursor: &mut Cursor<Vec<u8>>) -> Result<ParticipantData, Error> {
    let ai_controlled = cursor.byte_order().read_u8().await? == 1;
    let driver = parse_driver(cursor.byte_order().read_u8().await?)?;
    let team = parse_team(cursor.byte_order().read_u8().await?)?;
    let race_number = cursor.byte_order().read_u8().await?;
    let nationality = parse_nationality(cursor.byte_order().read_u8().await?)?;
    let name = parse_name(cursor).await?;
    let your_telemetry = parse_your_telemetry(cursor.byte_order().read_u8().await?)?;

    Ok(ParticipantData {
        ai_controlled,
        driver,
        team,
        race_number,
        nationality,
        name,
        your_telemetry,
    })
}

async fn parse_name(cursor: &mut Cursor<Vec<u8>>) -> Result<String, Error> {
    let cursor_position = cursor.position();
    let mut letters = Vec::with_capacity(48);

    for _ in 0..48 {
        let letter = cursor.byte_order().read_u8().await? as char;

        if letter == '\0' {
            break;
        } else {
            letters.push(letter);
        }
    }

    cursor.set_position(cursor_position + 48);
    Ok(letters.iter().collect())
}

fn parse_driver(value: u8) -> Result<Driver, Error> {
    match value {
        0 => Ok(Driver::CarlosSainz),
        1 => Ok(Driver::DaniilKvyat),
        2 => Ok(Driver::DanielRicciardo),
        6 => Ok(Driver::KimiRaikkonen),
        7 => Ok(Driver::LewisHamilton),
        9 => Ok(Driver::MaxVerstappen),
        10 => Ok(Driver::NicoHulkenburg),
        11 => Ok(Driver::KevinMagnussen),
        12 => Ok(Driver::RomainGrosjean),
        13 => Ok(Driver::SebastianVettel),
        14 => Ok(Driver::SergioPerez),
        15 => Ok(Driver::ValtteriBottas),
        17 => Ok(Driver::EstebanOcon),
        19 => Ok(Driver::LanceStroll),
        20 => Ok(Driver::ArronBarnes),
        21 => Ok(Driver::MartinGiles),
        22 => Ok(Driver::AlexMurray),
        23 => Ok(Driver::LucasRoth),
        24 => Ok(Driver::IgorCorreia),
        25 => Ok(Driver::SophieLevasseur),
        26 => Ok(Driver::JonasSchiffer),
        27 => Ok(Driver::AlainForest),
        28 => Ok(Driver::JayLetourneau),
        29 => Ok(Driver::EstoSaari),
        30 => Ok(Driver::YasarAtiyeh),
        31 => Ok(Driver::CallistoCalabresi),
        32 => Ok(Driver::NaotaIzum),
        33 => Ok(Driver::HowardClarke),
        34 => Ok(Driver::WilhelmKaufmann),
        35 => Ok(Driver::MarieLaursen),
        36 => Ok(Driver::FlavioNieves),
        37 => Ok(Driver::PeterBelousov),
        38 => Ok(Driver::KlimekMichalski),
        39 => Ok(Driver::SantiagoMoreno),
        40 => Ok(Driver::BenjaminCoppens),
        41 => Ok(Driver::NoahVisser),
        42 => Ok(Driver::GertWaldmuller),
        43 => Ok(Driver::JulianQuesada),
        44 => Ok(Driver::DanielJones),
        45 => Ok(Driver::ArtemMarkelov),
        46 => Ok(Driver::TadasukeMakino),
        47 => Ok(Driver::SeanGelael),
        48 => Ok(Driver::NyckDeVries),
        49 => Ok(Driver::JackAitken),
        50 => Ok(Driver::GeorgeRussell),
        51 => Ok(Driver::MaximilianGunther),
        52 => Ok(Driver::NireiFukuzumi),
        53 => Ok(Driver::LucaGhiotto),
        54 => Ok(Driver::LandoNorris),
        55 => Ok(Driver::SergioSetteCamara),
        56 => Ok(Driver::LouisDeletraz),
        57 => Ok(Driver::AntonioFuoco),
        58 => Ok(Driver::CharlesLeclerc),
        59 => Ok(Driver::PierreGasly),
        62 => Ok(Driver::AlexanderAlbon),
        63 => Ok(Driver::NicholasLatifi),
        64 => Ok(Driver::DorianBoccolacci),
        65 => Ok(Driver::NikoKari),
        66 => Ok(Driver::RobertoMerhi),
        67 => Ok(Driver::ArjunMaini),
        68 => Ok(Driver::AlessioLorandi),
        69 => Ok(Driver::RubenMeijer),
        70 => Ok(Driver::RashidNair),
        71 => Ok(Driver::JackTremblay),
        74 => Ok(Driver::AntonioGiovinazzi),
        75 => Ok(Driver::RobertKubica),
        78 => Ok(Driver::NobuharuMatsushita),
        79 => Ok(Driver::NikitaMazepin),
        80 => Ok(Driver::GuanyaZhou),
        81 => Ok(Driver::MickSchumacher),
        82 => Ok(Driver::CallumIlott),
        83 => Ok(Driver::JuanManuelCorrea),
        84 => Ok(Driver::JordanKing),
        85 => Ok(Driver::MahaveerRaghunathan),
        86 => Ok(Driver::TatianaCalderon),
        87 => Ok(Driver::AnthoineHubert),
        88 => Ok(Driver::GuilianoAlesi),
        89 => Ok(Driver::RalphBoschung),
        p if p >= 100 => Ok(Driver::Player),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid driver")),
    }
}

pub fn parse_team(value: u8) -> Result<Team, Error> {
    match value {
        0 => Ok(Team::Mercedes),
        1 => Ok(Team::Ferrari),
        2 => Ok(Team::RedBullRacing),
        3 => Ok(Team::Williams),
        4 => Ok(Team::RacingPoint),
        5 => Ok(Team::Renault),
        6 => Ok(Team::AlphaTauri),
        7 => Ok(Team::Haas),
        8 => Ok(Team::McLaren),
        9 => Ok(Team::AlfaRomeo),
        10 => Ok(Team::McLaren1988),
        11 => Ok(Team::McLaren1991),
        12 => Ok(Team::Williams1992),
        13 => Ok(Team::Ferrari1995),
        14 => Ok(Team::Williams1996),
        15 => Ok(Team::McLaren1998),
        16 => Ok(Team::Ferrari2002),
        17 => Ok(Team::Ferrari2004),
        18 => Ok(Team::Renault2006),
        19 => Ok(Team::Ferrari2007),
        20 => Ok(Team::McLaren2008),
        21 => Ok(Team::RedBull2010),
        22 => Ok(Team::Ferrari1976),
        23 => Ok(Team::ARTGrandPrix),
        24 => Ok(Team::CamposVexatecRacing),
        25 => Ok(Team::Carlin),
        26 => Ok(Team::CharouzRacingSystem),
        27 => Ok(Team::DAMS),
        28 => Ok(Team::RussianTime),
        29 => Ok(Team::MPMotorsport),
        30 => Ok(Team::Pertamina),
        31 => Ok(Team::McLaren1990),
        32 => Ok(Team::Trident),
        33 => Ok(Team::BWTArden),
        34 => Ok(Team::McLaren1976),
        35 => Ok(Team::Lotus1972),
        36 => Ok(Team::Ferrari1979),
        37 => Ok(Team::McLaren1982),
        38 => Ok(Team::Williams2003),
        39 => Ok(Team::Brawn2009),
        40 => Ok(Team::Lotus1978),
        41 => Ok(Team::F1GenericCar),
        42 => Ok(Team::ArtGP2019),
        43 => Ok(Team::Campos2019),
        44 => Ok(Team::Carlin2019),
        45 => Ok(Team::SauberJuniorCharouz2019),
        46 => Ok(Team::Dams2019),
        47 => Ok(Team::UniVirtuosi2019),
        48 => Ok(Team::MPMotorsport2019),
        49 => Ok(Team::Prema2019),
        50 => Ok(Team::Trident2019),
        51 => Ok(Team::Arden2019),
        53 => Ok(Team::Benetton1994),
        54 => Ok(Team::Benetton1995),
        55 => Ok(Team::Ferrari2000),
        56 => Ok(Team::Jordan1991),
        255 => Ok(Team::MyTeam),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid team")),
    }
}

pub fn parse_nationality(value: u8) -> Result<Nationality, Error> {
    match value {
        1 => Ok(Nationality::American),
        2 => Ok(Nationality::Argentinean),
        3 => Ok(Nationality::Australian),
        4 => Ok(Nationality::Austrian),
        5 => Ok(Nationality::Azerbaijani),
        6 => Ok(Nationality::Bahraini),
        7 => Ok(Nationality::Belgian),
        8 => Ok(Nationality::Bolivian),
        9 => Ok(Nationality::Brazilian),
        10 => Ok(Nationality::British),
        11 => Ok(Nationality::Bulgarian),
        12 => Ok(Nationality::Cameroonian),
        13 => Ok(Nationality::Canadian),
        14 => Ok(Nationality::Chilean),
        15 => Ok(Nationality::Chinese),
        16 => Ok(Nationality::Colombian),
        17 => Ok(Nationality::CostaRican),
        18 => Ok(Nationality::Croatian),
        19 => Ok(Nationality::Cypriot),
        20 => Ok(Nationality::Czech),
        21 => Ok(Nationality::Danish),
        22 => Ok(Nationality::Dutch),
        23 => Ok(Nationality::Ecuadorian),
        24 => Ok(Nationality::English),
        25 => Ok(Nationality::Emirian),
        26 => Ok(Nationality::Estonian),
        27 => Ok(Nationality::Finnish),
        28 => Ok(Nationality::French),
        29 => Ok(Nationality::German),
        30 => Ok(Nationality::Ghanaian),
        31 => Ok(Nationality::Greek),
        32 => Ok(Nationality::Guatemalan),
        33 => Ok(Nationality::Honduran),
        34 => Ok(Nationality::HongKonger),
        35 => Ok(Nationality::Hungarian),
        36 => Ok(Nationality::Icelander),
        37 => Ok(Nationality::Indian),
        38 => Ok(Nationality::Indonesian),
        39 => Ok(Nationality::Irish),
        40 => Ok(Nationality::Israeli),
        41 => Ok(Nationality::Italian),
        42 => Ok(Nationality::Jamaican),
        43 => Ok(Nationality::Japanese),
        44 => Ok(Nationality::Jordanian),
        45 => Ok(Nationality::Kuwaiti),
        46 => Ok(Nationality::Latvian),
        47 => Ok(Nationality::Lebanese),
        48 => Ok(Nationality::Lithuanian),
        49 => Ok(Nationality::Luxembourger),
        50 => Ok(Nationality::Malaysian),
        51 => Ok(Nationality::Maltese),
        52 => Ok(Nationality::Mexican),
        53 => Ok(Nationality::Monegasque),
        54 => Ok(Nationality::NewZealander),
        55 => Ok(Nationality::Nicaraguan),
        56 => Ok(Nationality::NorthKorean),
        57 => Ok(Nationality::NorthernIrish),
        58 => Ok(Nationality::Norwegian),
        59 => Ok(Nationality::Omani),
        60 => Ok(Nationality::Pakistani),
        61 => Ok(Nationality::Panamanian),
        62 => Ok(Nationality::Paraguayan),
        63 => Ok(Nationality::Peruvian),
        64 => Ok(Nationality::Polish),
        65 => Ok(Nationality::Portuguese),
        66 => Ok(Nationality::Qatari),
        67 => Ok(Nationality::Romanian),
        68 => Ok(Nationality::Russian),
        69 => Ok(Nationality::Salvadoran),
        70 => Ok(Nationality::Saudi),
        71 => Ok(Nationality::Scottish),
        72 => Ok(Nationality::Serbian),
        73 => Ok(Nationality::Singaporean),
        74 => Ok(Nationality::Slovakian),
        75 => Ok(Nationality::Slovenian),
        76 => Ok(Nationality::SouthKorean),
        77 => Ok(Nationality::SouthAfrican),
        78 => Ok(Nationality::Spanish),
        79 => Ok(Nationality::Swedish),
        80 => Ok(Nationality::Swiss),
        81 => Ok(Nationality::Thai),
        82 => Ok(Nationality::Turkish),
        83 => Ok(Nationality::Uruguayan),
        84 => Ok(Nationality::Ukrainian),
        85 => Ok(Nationality::Venezuelan),
        86 => Ok(Nationality::Welsh),
        87 => Ok(Nationality::Barbadian),
        88 => Ok(Nationality::Vietnamese),
        0 | 255 => Ok(Nationality::Invalid),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid nationality")),
    }
}
