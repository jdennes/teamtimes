extern crate chrono;
extern crate chrono_tz;
#[macro_use] extern crate prettytable;

use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use chrono_tz::Africa::Nairobi;
use chrono_tz::Europe::Amsterdam;
use chrono_tz::America::Toronto;
use chrono_tz::US::Eastern;
use chrono_tz::US::Mountain;
use chrono_tz::US::Pacific;
use prettytable::{Table};

struct TeamMemberTimeZone {
    label: String,
    time_zone: Tz,
}

impl TeamMemberTimeZone {
    fn local_time(&self, utc: DateTime<Utc>) -> DateTime<Tz> {
        utc.with_timezone(&self.time_zone)
    }
}

fn get_timezones() -> [TeamMemberTimeZone; 8] {
    [
        TeamMemberTimeZone {label: "Nairobi".to_string(), time_zone: Nairobi},
        TeamMemberTimeZone {label: "Rotterdam".to_string(), time_zone: Amsterdam},
        TeamMemberTimeZone {label: "Düsseldorf".to_string(), time_zone: Amsterdam},
        TeamMemberTimeZone {label: "Boston".to_string(), time_zone: Eastern},
        TeamMemberTimeZone {label: "Toronto".to_string(), time_zone: Toronto},
        TeamMemberTimeZone {label: "Arizona".to_string(), time_zone: Mountain},
        TeamMemberTimeZone {label: "Seattle".to_string(), time_zone: Pacific},
        TeamMemberTimeZone {label: "California".to_string(), time_zone: Pacific}
    ]
}

fn main() {
    let mut table = Table::new();
    let utc: DateTime<Utc> = Utc::now();
    let timezones: [TeamMemberTimeZone; 8] = get_timezones();

    table.add_row(row!["Location", "Local time", "Offset"]);
    for tz in &timezones {
        let local_time: DateTime<Tz> = tz.local_time(utc);
        table.add_row(row![
            tz.label,
            local_time.format("%Y-%m-%d %H:%M:%S %Z"),
            local_time.format("%:z").to_string()
        ]);
    }
    table.printstd();
}
