use std::{collections::HashMap, usize};

use crate::error::{TemplateError, TemplateErrorKind};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// The Default Regex Template
    pub static ref DEFAULT_TEMPLATE: Regex = Regex::new(r"(?mi)\{\s*(\S+)\s*\}").unwrap();
}

/// This struct is to simplify usage of 4 "usize"
struct MatchEntry {
    outer_start: usize,
    outer_end: usize,

    inner_start: usize,
    inner_end: usize,
}

impl MatchEntry {
    pub fn new(outer: (usize, usize), inner: (usize, usize)) -> MatchEntry {
        return MatchEntry {
            outer_start: outer.0,
            outer_end: outer.1,

            inner_start: inner.0,
            inner_end: inner.1,
        };
    }
}

/// Struct to store the template
pub struct Template {
    /// Template String
    src: String,
    /// All matches from the Template String
    matches: Vec<MatchEntry>,
}

impl Template {
    /// Create a new Template Instance with the default regex
    /// # Example
    /// ```rust
    /// # use new_string_template::template::*;
    /// let input_template = "Some {{ Template }}";
    /// let template_instance = Template::new(input_template);
    /// ```
    pub fn new<T: Into<String>>(template: T) -> Self {
        let converted_string = template.into();
        let matches = get_matches(&DEFAULT_TEMPLATE, &converted_string);
        return Template {
            src: converted_string,
            matches,
        };
    }

    /// Change used regex to an custom one
    /// The regex needs to include one valid capture group
    /// # Example
    /// ```rust
    /// # use new_string_template::template::*;
    /// # use regex::Regex;
    /// # let template_string = "hello";
    /// # let custom_regex = Regex::new(r"(.*)").unwrap();
    /// let templ = Template::new(template_string).with_regex(&custom_regex);
    /// ```
    pub fn with_regex(mut self, regex: &Regex) -> Self {
        self.matches = get_matches(&regex, &self.src);

        return self;
    }

    /// Render the template with the provided values
    fn render_internal(
        &self,
        values: &HashMap<&str, &str>,
        fail: bool,
    ) -> Result<String, TemplateError> {
        // Early return if there are no matches in the template string
        if self.matches.is_empty() {
            return Ok(self.src.clone());
        }

        // Start with an empty "Vec", but with at least the capacity of "self.matches"
        let mut parts: Vec<&str> = Vec::with_capacity(self.matches.len());
        // Save last index of an match, starting with "0"
        let mut last_index: usize = 0;

        for entry in self.matches.iter() {
            parts.push(&self.src[last_index..entry.outer_start]);

            let arg_name = &self.src[entry.inner_start..entry.inner_end];

            // not using "unwrap_or_else" because of the need to return "Err"
            match values.get(&arg_name) {
                Some(v) => parts.push(*v),
                _ => {
                    if fail {
                        return Err(TemplateError::new(
                            TemplateErrorKind::MissingData,
                            format!("Missing Data for Argument \"{}\"", &arg_name),
                        ));
                    }

                    parts.push(&self.src[entry.outer_start..entry.outer_end]);
                }
            }

            last_index = entry.outer_end;
        }

        // if string is not already fully copied, copy the rest of it
        if last_index < self.src.len() {
            parts.push(&self.src[last_index..self.src.len()]);
        }

        return Ok(parts.join(""));
    }

    /// Render the template with the provided values
    /// # Errors
    /// This function Errors on the first problem encountered
    /// # Example
    /// ```rust
    /// # use new_string_template::template::*;
    /// # use std::collections::HashMap;
    /// let templ_str = "Something {data1} be {data2}, and { not here }";
    /// let templ = Template::new(templ_str);
    /// let data = {
    ///     let mut map = HashMap::new();
    ///     map.insert("data1", "should");
    ///     map.insert("data2", "here");
    ///     map
    /// };
    ///
    /// let rendered = templ.render(&data).expect("Expected Result to be Ok");
    /// assert_eq!("Something should be here, and { not here }", rendered);
    /// ```
    pub fn render(&self, values: &HashMap<&str, &str>) -> Result<String, TemplateError> {
        return self.render_internal(values, true);
    }

    /// Render the template with the provided values
    /// Returns Full Converted String and no Result
    /// # Example
    /// ```rust
    /// # use new_string_template::template::*;
    /// # use std::collections::HashMap;
    /// let templ_str = "Something {data1} be {data2}, and { not here }";
    /// let templ = Template::new(templ_str);
    /// let data = {
    ///     let mut map = HashMap::new();
    ///     map.insert("data1", "should");
    ///     // map.insert("data2", "here");
    ///     map
    /// };
    ///
    /// let rendered = templ.render_nofail(&data);
    /// assert_eq!("Something should be {data2}, and { not here }", rendered);
    /// ```
    pub fn render_nofail(&self, values: &HashMap<&str, &str>) -> String {
        return self
            .render_internal(values, false)
            .unwrap_or_else(|_| return self.src.clone());
    }
}

fn get_matches(regex: &Regex, template: &str) -> Vec<MatchEntry> {
    return regex
        .captures_iter(&template)
        .map(|found| {
            let outer_match = found.get(0).expect("Match Index 0 was None (Full Match)");
            let inner_match = found.get(1).expect("Match Index 1 was None (Inner Match)");

            return MatchEntry::new(
                (outer_match.start(), outer_match.end()),
                (inner_match.start(), inner_match.end()),
            );
        })
        .collect();
}

#[cfg(test)]
mod test {
    use crate::error::TemplateErrorKind;

    use super::*;

    #[test]
    fn test_render_full_no_error() {
        let templ_str = "Something {data1} be {data2}, and { not here }";
        let templ = Template::new(templ_str);
        let data = {
            let mut map = HashMap::new();
            map.insert("data1", "should");
            map.insert("data2", "here");
            map
        };

        let rendered = templ.render(&data).expect("Expected Result to be Ok");
        assert_eq!("Something should be here, and { not here }", rendered);
    }

    #[test]
    fn test_render_should_error_on_missing_data() {
        let templ_str = "Something {data1} be {data2}, and { not here }";
        let templ = Template::new(templ_str);
        let data = {
            let mut map = HashMap::new();
            map.insert("data1", "should");
            // map.insert("data2", "here"); // the missing data
            map
        };

        let rendered = templ.render(&data).expect_err("Expected Result to be Ok");
        assert_eq!(TemplateErrorKind::MissingData, rendered.kind());
    }

    #[test]
    fn test_render_nofail_full_no_error() {
        let templ_str = "Something {data1} be {data2}, and { not here }";
        let templ = Template::new(templ_str);
        let data = {
            let mut map = HashMap::new();
            map.insert("data1", "should");
            // map.insert("data2", "here");
            map
        };

        let rendered = templ.render_nofail(&data);
        assert_eq!("Something should be {data2}, and { not here }", rendered);
    }

    #[test]
    fn test_render_custom_regex_double_brackets() {
        let custom_regex = Regex::new(r"(?mi)\{\{\s+([^\}]+)\s+\}\}").unwrap();
        let templ_str = "Something {{ data1 }} be {{ data2 }}, and {{ data 3 }}";
        let templ = Template::new(templ_str).with_regex(&custom_regex);
        let data = {
            let mut map = HashMap::new();
            map.insert("data1", "should");
            map.insert("data2", "here");
            map.insert("data 3", "here too");
            map
        };

        let rendered = templ.render_nofail(&data);
        assert_eq!("Something should be here, and here too", rendered);
    }

    #[test]
    fn test_render_custom_regex_single() {
        let custom_regex = Regex::new(r"(?mi)#(\S+)").unwrap();
        let templ_str = "Signle character #data1 here";
        let templ = Template::new(templ_str).with_regex(&custom_regex);
        let data = {
            let mut map = HashMap::new();
            map.insert("data1", "can be seen");
            map
        };

        let rendered = templ.render_nofail(&data);
        assert_eq!("Signle character can be seen here", rendered);
    }
}
