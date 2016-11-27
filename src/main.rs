extern crate chrono;

use chrono::*;


enum DDays
{
    Sweetmorn,
    Boomtime,
    PricklePrickle,
    Pungenday,
    SettingOrange,
}

enum Seasons
{
    Chaos,
    Discord,
    Confusion,
    Bureaucracy,
    Aftermath,
}

fn is_leapyear(year: i32) -> bool
{
    let factor = |x| year % x == 0;
    factor(4) && (factor(100) || factor(400))
}

fn get_year(year: i32) -> i32
{
    year + 1166
}

fn get_season(day: u32) -> Seasons
{
    use Seasons::*;

    let index = day / 73;
    match index
    {
        0 => Seasons::Chaos,
        1 => Seasons::Discord,
        2 => Seasons::Confusion,
        3 => Seasons::Bureaucracy,
        4 => Seasons::Aftermath,
        _ => {
            println!("How even???");
            Confusion
        }
    }
}

fn get_day(day: u32) -> (DDays, u32)
{
    use DDays::*;

    let yd = day % 73;
    let index = (yd % 5) + 1;
    let dday = match index
    {
        0 => Sweetmorn,
        1 => Boomtime,
        2 => Pungenday,
        3 => PricklePrickle,
        4 => SettingOrange,
        _ => {
            println!("How delightfully confusing!");
            Boomtime
        }
    };
    (dday, yd)
}

fn convert_season(season: Seasons) -> String
{
    use Seasons::*;

    match season
    {
        Chaos => format!("Chaos"),
        Discord => format!("Discord"),
        Confusion => format!("Confusion"),
        Bureaucracy => format!("Bureaucracy"),
        Aftermath => format!("The Aftermath"),
    }
}

fn convert_day(day: DDays, leapyear: bool, ordinal: u32) -> String
{
    use DDays::*;

    if leapyear
    {
        if ordinal == 59
        {
            return format!("St. Tib's Day");
        }
    }

    match day
    {
        Sweetmorn => format!("Sweetmorn"),
        Boomtime => format!("Boomtime"),
        Pungenday => format!("Pungenday"),
        PricklePrickle => format!("Prickle-Prickle"),
        SettingOrange => format!("Setting Orange"),
    }
}

fn set_holyday(holyday: &mut String, season: Seasons)
{
    use Seasons::*;

    holyday.push_str(match season
    {
        Chaos => "Chaoflux",
        Discord => "Discoflux",
        Confusion => "Confuflux",
        Bureaucracy => "Bureflux",
        Aftermath => "Afflux",
    });
}

fn main()
{
    let dt = Local::now();
    let mut yd = dt.ordinal() - 1;

    if is_leapyear(dt.year()) &&  yd > 59
    {
        yd -= 1;
    }

    let dyear = get_year(dt.year());
    let dseason = convert_season(get_season(yd));
    let (dday, ys) = get_day(yd);
    let mut holyday = String::new();

    if ys == 5
    {
        set_holyday(&mut holyday, get_season(yd));
    }

    let dday = convert_day(dday, is_leapyear(dt.year()), yd);
    let ddate = format!("{}, {} {}, {} YOLD", dday, dseason, ys, dyear);

    println!("Today is: {}", &ddate);
    if holyday.len() > 0
    {
        println!("Celebrate {}!", holyday);
    }
}
