extern crate chrono;
extern crate chrono_tz;
#[macro_use] extern crate prettytable;

use chrono::{DateTime, Utc};
use chrono_tz::Europe::Amsterdam;
use chrono_tz::US::Pacific;

use prettytable::{Table};

fn main() {
    let mut table = Table::new();

    let utc_now: DateTime<Utc> = Utc::now();

    // [<label>, <local time>]
    table.add_row(row!["Amsterdam", utc_now.with_timezone(&Amsterdam).to_string()]);
    table.add_row(row!["Pacific", utc_now.with_timezone(&Pacific).to_string()]);

    table.printstd();
}
