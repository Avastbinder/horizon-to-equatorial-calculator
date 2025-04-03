use iced::widget::{button, column, text, text_input, Column};
use iced;

#[derive(Default)]
struct State{
    latitude: String,
    longitude: String,
    altitude: String,
    azimuth: String,
    year: String,
    month: String,
    day: String,
    hour: String,
    minute: String,
    second: String,
    ra: f64,
    dec: f64
}

// Iced functions
#[derive(Debug, Clone)]
pub enum Message {
    LatitudeChanged(String),
    LongitudeChanged(String),
    AltitudeChanged(String),
    AzimuthChanged(String),
    YearChanged(String),
    MonthChanged(String),
    DayChanged(String),
    HourChanged(String),
    MinuteChanged(String),
    SecondChanged(String),
    FetchEquatorial
}

impl State {
    // Display updater
    fn update(&mut self, message: Message) -> () {
        match message {
            Message::LatitudeChanged(latitude) => {
                self.latitude = latitude;
            }
            Message::LongitudeChanged(longitude) => {
                self.longitude = longitude;
            }
            Message::AltitudeChanged(altitude) => {
                self.altitude = altitude;
            }
            Message::AzimuthChanged(azimuth) => {
                self.azimuth = azimuth;
            }
            Message::YearChanged(year) => {
                self.year = year;
            }
            Message::MonthChanged(month) => {
                self.month = month;
            }
            Message::DayChanged(day) => {
                self.day = day;
            }
            Message::HourChanged(hour) => {
                self.hour = hour;
            }
            Message::MinuteChanged(minute) => {
                self.minute = minute;
            }
            Message::SecondChanged(second) => {
                self.second = second;
            }
    
            Message::FetchEquatorial => {
                let (latitude, longitude, altitude, azimuth, year, month, day, hour, minute, second) = 
                to_correct(&self.latitude, &self.longitude, &self.altitude, &self.azimuth, &self.year, &self.month, &self.day, &self.hour, &self.minute, &self.second);

                match horizon_to_equatorial(
                latitude, longitude, altitude, azimuth, year, month, day, hour, minute, second) {
                    Ok((_ra, _dec)) => {
                        self.ra = _ra;
                        self.dec = _dec;
                    }
                    Err(_) => {println!("Error converting to equatorial")}
                }

            }
        }
    }

    // Display view
    fn view(&self) -> Column<Message> {
        column!
        [
            text("Type latitude of observation location").size(14),
            text_input("", &self.latitude.to_string())
                .on_input(Message::LatitudeChanged),
            text("Type longitude of observation location").size(14),
            text_input("", &self.longitude.to_string())
                .on_input(Message::LongitudeChanged),
            text("Type altitude of star (not height of location)").size(14),
            text_input("", &self.altitude.to_string())
                .on_input(Message::AltitudeChanged),
            text("Type azimuth of star").size(14),
            text_input("", &self.azimuth.to_string())
                .on_input(Message::AzimuthChanged),
            text("Type year at observation").size(14),
            text_input("", &self.year.to_string())
                .on_input(Message::YearChanged),
            text("Type month at observation").size(14),
            text_input("", &self.month.to_string())
                .on_input(Message::MonthChanged),
            text("Type day of observation").size(14),
            text_input("", &self.day.to_string())
                .on_input(Message::DayChanged),
            text("Type hour of observation in 24 hour format").size(14),
            text_input("", &self.hour.to_string())
                .on_input(Message::HourChanged),
            text("Input UTC time").size(12),
            text("Type minute of observation").size(14),
            text_input("", &self.minute.to_string())
                .on_input(Message::MinuteChanged),
            text("Type second of observation").size(14),
            text_input("", &self.second.to_string())
                .on_input(Message::SecondChanged),
            text("If unsure, type 30").size(12),
            button("Enter").on_press(Message::FetchEquatorial),
            text(format!("RA: {:.6}\nDEC: {:.6}", &self.ra, &self.dec)).size(14)
        ].spacing(5).padding(15).into()

    }
}

fn to_correct(    
latitude: &String,
longitude: &String,
altitude: &String,
azimuth: &String,
year: &String,
month: &String,
day: &String,
hour: &String,
minute: &String,
second: &String) -> (f64, f64, f64, f64, i32, i32, i32, i32, i32, i32)
{
    let mut new_lat = 0.0;
    let mut new_long = 0.0;
    let mut new_alt = 0.0;
    let mut new_azi = 0.0;
    let mut new_year = 0;
    let mut new_month = 0;
    let mut new_day = 0;
    let mut new_hour = 0;
    let mut new_min = 0;
    let mut new_sec = 0;
    match latitude.parse::<f64>() {
        Ok(val) => new_lat = val,
        Err(_) => {}
    }
    match longitude.parse::<f64>() {
        Ok(val) => new_long = val,
        Err(_) => {}
    }
    match altitude.parse::<f64>() {
        Ok(val) => new_alt = val,
        Err(_) => {}
    }
    match azimuth.parse::<f64>() {
        Ok(val) => new_azi = val,
        Err(_) => {}
    }
    match year.parse::<i32>() {
        Ok(val) => new_year = val,
        Err(_) => {}
    }
    match month.parse::<i32>() {
        Ok(val) => new_month = val,
        Err(_) => {}
    }
    match day.parse::<i32>() {
        Ok(val) => new_day = val,
        Err(_) => {}
    }
    match hour.parse::<i32>() {
        Ok(val) => new_hour = val,
        Err(_) => {}
    }
    match minute.parse::<i32>() {
        Ok(val) => new_min = val,
        Err(_) => {}
    }
    match second.parse::<i32>() {
        Ok(val) => new_sec = val,
        Err(_) => {}
    }
    (new_lat, new_long, new_alt, new_azi, new_year, new_month, new_day, new_hour, new_min, new_sec)
}

fn main() {
    iced::run("Horizontal to Equatorial Calculator - Aidan Vastbinder", State::update, State::view)
    .expect("Failed to start application")
}


fn horizon_to_equatorial(
    latitude: f64,
    longitude: f64,
    altitude: f64,
    azimuth: f64,
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32
) -> Result<(f64, f64), Box<dyn std::error::Error>> {

    let lat_rad = latitude.to_radians();
    let alt_rad = altitude.to_radians();
    let az_rad = azimuth.to_radians();
    
    // Compute Julian Day and GMST
    let jd = julian_day(year, month, day, hour, minute, second);
    let gmst_deg = gmst(jd);
    println!("\nJulian Day: {}", jd);
    println!("GMST (deg): {}", gmst_deg);
    
    // Compute Local Sidereal Time (LST)
    let lst_deg = (((gmst_deg / 15.0) + (longitude / 15.0)) * 15.0).rem_euclid(360.0);
    println!("LST (deg): {}", lst_deg);
    
    // Calculate declination (Dec)
    let sin_dec = alt_rad.sin() * lat_rad.sin() + alt_rad.cos() * lat_rad.cos() * az_rad.cos();
    let dec_rad = sin_dec.asin();
    
    // Compute Hour Angle (H)
    let dec_deg = dec_rad.to_degrees();
    
    let sin_dec = altitude.to_radians().sin() * latitude.to_radians().sin() 
        + altitude.to_radians().cos() * latitude.to_radians().cos() * azimuth.to_radians().cos();
    let dec = sin_dec.asin().to_degrees();
    
    let sin_lha = (-azimuth.to_radians().sin() * altitude.to_radians().cos()) / dec.to_radians().cos();
    let cos_lha = (altitude.to_radians().sin() - (latitude.to_radians().sin() * dec.to_radians().sin()))
        / (dec.to_radians().cos() * latitude.to_radians().cos());
    let lha = sin_lha.atan2(cos_lha).to_degrees().rem_euclid(360.0);

    println!("LHA: {}", lha);
    
    let ra_deg = (lst_deg - lha).rem_euclid(360.0);
    
    Ok((ra_deg, dec_deg))
}

/// Compute Julian Day (JD) given a date and time in UTC.
fn julian_day(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> f64 {
    let y = if month <= 2 { year - 1 } else { year };
    let m = if month <= 2 { month + 12 } else { month };
    let a = (y as f64 / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();
    let day_fraction = (hour as f64 + minute as f64 / 60.0 + second as f64 / 3600.0) / 24.0;
    let jd = (365.25 * (y as f64 + 4716.0)).floor()
        + (30.6001 * ((m + 1) as f64)).floor()
        + day as f64
        + b
        - 1524.5
        + day_fraction;
    jd
}

/// Compute Greenwich Mean Sidereal Time (GMST) in degrees.
fn gmst(jd: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0;
    let gmst = 280.46061837 + 360.98564736629 * (jd - 2451545.0) + 0.000387933 * t.powi(2) - (t.powi(3) / 38710000.0);
    gmst.rem_euclid(360.0)
}
