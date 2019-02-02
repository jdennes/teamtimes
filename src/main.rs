extern crate chrono;
extern crate chrono_tz;
#[macro_use] extern crate prettytable;

use chrono::{DateTime, Utc};
use chrono_tz::Africa::Nairobi;
use chrono_tz::Europe::Amsterdam;
use chrono_tz::America::Toronto;
use chrono_tz::US::Eastern;
use chrono_tz::US::Mountain;
use chrono_tz::US::Pacific;
use prettytable::{Table};

fn main() {
    let mut table = Table::new();

    let utc_now: DateTime<Utc> = Utc::now();

    table.add_row(row!["Nairobi", utc_now.with_timezone(&Nairobi).to_string()]);
    table.add_row(row!["Rotterdam", utc_now.with_timezone(&Amsterdam).to_string()]);
    table.add_row(row!["Düsseldorf", utc_now.with_timezone(&Amsterdam).to_string()]);
    table.add_row(row!["Boston", utc_now.with_timezone(&Eastern).to_string()]);
    table.add_row(row!["Toronto", utc_now.with_timezone(&Toronto).to_string()]);
    table.add_row(row!["Arizona", utc_now.with_timezone(&Mountain).to_string()]);
    table.add_row(row!["Seattle", utc_now.with_timezone(&Pacific).to_string()]);
    table.add_row(row!["California", utc_now.with_timezone(&Pacific).to_string()]);

    table.printstd();
}
