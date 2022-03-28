use select::node::Node;
use select::predicate::Attr;
use crate::{Result, VGMError};
use crate::models::MultiLanguageString;

fn parse_month(input: &str) -> Result<u8> {
    // Months
    // Jan, Feb, Mar, Apr, May, Jun, Jul, Aug, Sep, Oct, Nov, Dec
    // January, February, March, April, May, June, July, August, September, October, November, December
    Ok(match input {
        "January" | "Jan" => 1,
        "February" | "Feb" => 2,
        "March" | "Mar" => 3,
        "April" | "Apr" => 4,
        "May" => 5,
        "June" | "Jun" => 6,
        "July" | "Jul" => 7,
        "August" | "Aug" => 8,
        "September" | "Sep" => 9,
        "October" | "Oct" => 10,
        "November" | "Nov" => 11,
        "December" | "Dec" => 12,
        _ => return Err(VGMError::InvalidDate),
    })
}

pub(crate) fn parse_date(input: &str) -> Result<String> {
    let input = input.trim().replace(",", "");
    let parts = input.split(" ").collect::<Vec<&str>>();
    // there are three types of time
    return if parts.len() >= 3 {
        // 1. Month date, Year -> Month date Year
        //                          0    1    2
        Ok(format!("{}-{:02}-{}", parts[2], parse_month(parts[0])?, parts[1]))
    } else if parts.len() == 2 {
        // 2. Month Year
        Ok(format!("{}-{:02}", parts[1], parse_month(parts[0])?))
    } else if parts.len() == 1 {
        // 3. Year
        Ok(parts[0].to_string())
    } else {
        unreachable!()
    };
}

pub(crate) fn parse_multi_language(node: &Node) -> MultiLanguageString {
    let mut title = MultiLanguageString::default();
    for node in node.select(Attr("class", "albumtitle")) {
        let language = node.attr("lang").unwrap();
        let mut text = String::new();
        recur(&node, &mut text);
        fn recur(node: &Node, string: &mut String) {
            if let Some(text) = node.as_text() {
                string.push_str(text);
            } else if let Some("em") = node.name() {
                return;
            }
            for child in node.children() {
                recur(&child, string)
            }
        }
        title.insert(language.to_string(), text);
    }
    title
}

#[cfg(test)]
mod test {
    #[test]
    fn normal_dates() {
        assert_eq!(super::parse_date("Aug 13, 2006").unwrap(), "2006-08-13");
        assert_eq!(super::parse_date("Sep 29, 2021").unwrap(), "2021-09-29");
        assert_eq!(super::parse_date("Jul 12, 2016").unwrap(), "2016-07-12");
        assert_eq!(super::parse_date("Jul 2017").unwrap(), "2017-07");
        assert_eq!(super::parse_date("2014").unwrap(), "2014");
    }
}