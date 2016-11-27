extern crate chrono;

use chrono::*;


enum DDays {
    Sweetmorn,
    Boomtime,
    PricklePrickle,
    Pungenday,
    SettingOrange,
}

enum Seasons {
    Chaos,
    Discord,
    Confusion,
    Bureaucracy,
    Aftermath,
}

fn is_leapyear(year: i32) -> bool {
    let factor = |x| year % x == 0;
    factor(4) && (factor(100) || factor(400))
}

fn get_year(year: i32) -> i32 {
    year + 1166
}

fn get_season(day: u32) -> Seasons {
    let index = day / 73;
    match index {
        0 => Seasons::Chaos,
        1 => Seasons::Discord,
        2 => Seasons::Confusion,
        3 => Seasons::Bureaucracy,
        4 => Seasons::Aftermath,
        _ => {
            println!("get_season failed?");
            Seasons::Confusion
        }
    }
}

fn get_day(day: u32) -> (DDays, u32) {
    let yd = day % 73;
    let index = (yd % 5) + 1;
    let dday = match index {
        0 => DDays::Sweetmorn,
        1 => DDays::Boomtime,
        2 => DDays::Pungenday,
        3 => DDays::PricklePrickle,
        4 => DDays::SettingOrange,
        _ => DDays::Sweetmorn,
    };
    (dday, yd)
}

fn convert_season(season: Seasons) -> String {
    match season {
        Seasons::Chaos => format!("Chaos"),
        Seasons::Discord => format!("Discord"),
        Seasons::Confusion => format!("Confusion"),
        Seasons::Bureaucracy => format!("Bureaucracy"),
        Seasons::Aftermath => format!("The Aftermath"),
    }
}

fn convert_day(day: DDays, leapyear: bool, ordinal: u32) -> String {
    if leapyear {
        if ordinal == 59 {
            return format!("St. Tib's Day");
        }
    }

    match day {
        DDays::Sweetmorn => format!("Sweetmorn"),
        DDays::Boomtime => format!("Boomtime"),
        DDays::Pungenday => format!("Pungenday"),
        DDays::PricklePrickle => format!("Prickle-Prickle"),
        DDays::SettingOrange => format!("Setting Orange"),
    }
}

fn get_holyday(ys: u32, season: Seasons) -> Option<&'static str> {
    if ys == 5 {
        match season {
            Seasons::Chaos => Some("Chaoflux"),
            Seasons::Discord => Some("Discoflux"),
            Seasons::Confusion => Some("Confuflux"),
            Seasons::Bureaucracy => Some("Bureflux"),
            Seasons::Aftermath => Some("Afflux"),
        }
    } else {
        None
    }
}

fn main() {
    let dt = Local::now();
    let mut yd = dt.ordinal() - 1;

    if is_leapyear(dt.year()) &&  yd > 59 {
        yd -= 1;
    }

    let dyear = get_year(dt.year());
    let dseason = convert_season(get_season(yd));
    let (dday, ys) = get_day(yd);
    let dday = convert_day(dday, is_leapyear(dt.year()), yd);
    let ddate = format!("{}, {} {}, {} YOLD", dday, dseason, ys, dyear);

    println!("Today is: {}", &ddate);
    if let Some(holyday) = get_holyday(ys, get_season(yd)) {
        println!("Celebrate {}!", holyday);
    }
}
