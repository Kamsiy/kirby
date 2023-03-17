#[cfg(test)]
use crate::*;
use chrono::NaiveDate;

#[test]

fn test_get_ercot_url() {
    let date = NaiveDate::from_ymd_opt(2022, 3, 14).unwrap().and_hms_opt(9, 10, 11).unwrap();
    let expected_url = String::from("https://www.ercot.com/content/cdr/html/20220314_dam_spp.html");
    assert_eq!(get_ercot_url(date), expected_url);
}

#[test]
fn test_get_ercot_page() {
    let result = get_ercot_page();
    assert!(result.unwrap().len() > 0);
}
