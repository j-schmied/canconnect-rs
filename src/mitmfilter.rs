//! Implementation of MitmFilter

use regex::Regex;

/// Man-in-the-Middle Filter Type
#[derive(Debug)]
pub(crate) struct MitmFilter {
    pub(crate) filter_operation: char,
    pub(crate) filter_id: u16,
}

impl MitmFilter {
    fn cmp(&self, id: &u16) -> bool {
        if self.filter_id == *id {
            return true;
        }
        false
    }

    fn set_from_string(&mut self, filter: &mut String) {
        if is_valid_filter(&filter) {
            // Filter structure: "< 0x000 or > 0x00 or = 0x0 or *"
            let trimmed = filter.trim();
            let bytes = trimmed.as_bytes();

            // get operation
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    self.filter_operation = (&filter[0..i]).parse().unwrap();
                    // assume that rest of the input string is the id as is_valid_filter passed
                    if self.filter_operation != '*' {
                        let id = filter[i + 1..].trim();
                        self.filter_id = id.parse().unwrap();
                    }
                }
            }
        }
    }

    pub(crate) fn is_valid_filter(filter: &String) -> bool {
        let valid_filter_pattern = Regex::new(r"(^\s*[*]\s*$)|(^\s*[<|>|=]\s*0x[0-7][0-9a-fA-F]{1,2}\s*$)|(^\s*[<|>|=]\s*0x[0-9]\s*$)").unwrap();
        if valid_filter_pattern.is_match(filter) {
            return true;
        }
        false
    }
}
