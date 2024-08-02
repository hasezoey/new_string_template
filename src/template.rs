//! Module to contain everything needed for [`Template`]

use std::{
    collections::HashMap,
    usize,
};

use crate::error::{
    TemplateError,
    TemplateErrorKind,
};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// The Default Regex Template
    pub static ref DEFAULT_TEMPLATE: Regex = Regex::new(r"(?mi)\{\s*(\S+?)\s*\}").unwrap();
}

/// This is a Struct to Help with caching matches found
/// Stores the beginning and end of the full match and the beginning and end of the value name inside the full match
#[derive(Debug, Clone, PartialEq, Copy)]
struct MatchEntry {
    /// Offset for the beginning of the full match in a parsed Template String
    full_match_start: usize,
    /// Offset for the end of the full match in a parsed Template String
    /// This refers to the character just after the match (using [`regex::Match::end`])
    full_match_end:   usize,

    /// Offset for the beginning of the value name match in a parsed Template String
    value_name_start: usize,
    /// Offset for the end of the value name match in a parsed Template String
    /// This refers to the character just after the match (using [`regex::Match::end`])
    value_name_end:   usize,
}

impl MatchEntry {
    /// Create a new [`MatchEntry`] instance, translating the tuples to inner values
    pub const fn new(full_match: (usize, usize), value_name: (usize, usize)) -> Self {
        return Self {
            full_match_start: full_match.0,
            full_match_end:   full_match.1,

            value_name_start: value_name.0,
            value_name_end:   value_name.1,
        };
    }
}

/// Struct to store the template
#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    /// Template String
    src:     String,
    /// All matches from the Template String
    matches: Vec<MatchEntry>,
}

impl Template {
    /// Create a new Template Instance with the default regex.
    /// # Example
    /// ```rust
    /// # use new_string_template::template::*;
    /// let input_template = "Some { Template }";
    /// let template_instance = Template::new(input_template);
    /// ```
    pub fn new<T: Into<String>>(template: T) -> Self {
        let converted_string = template.into();
        let matches = get_matches(&DEFAULT_TEMPLATE, &converted_string);
        return Self {
            src: converted_string,
            matches,
        };
    }

    /// Change the [`Regex`] that is used to resolve the matches from the template string.  
    /// The [`Regex`] requires to have at least one capture group.
    /// # Example
    /// ```rust
    /// # use new_string_template::template::*;
    /// # use regex::Regex;
    /// # let template_string = "hello";
    /// # let custom_regex = Regex::new(r"(.*)").unwrap();
    /// let templ = Template::new(template_string).with_regex(&custom_regex);
    /// ```
    #[must_use]
    pub fn with_regex(mut self, regex: &Regex) -> Self {
        self.matches = get_matches(regex, &self.src);

        return self;
    }

    /// Render the template with the provided values.
    ///
    /// Internal Helper function for [`Template::render`], [`Template::render_string`] and [`Template::render_nofail`].
    fn render_internal<T: AsRef<str>>(&self, values: &HashMap<&str, T>, fail: bool) -> Result<String, TemplateError> {
        // Early return if there are no matches in the template string
        if self.matches.is_empty() {
            return Ok(self.src.clone());
        }

        // Start with an empty "Vec", but with at least the capacity of "self.matches"
        let mut parts: Vec<&str> = Vec::with_capacity(self.matches.len());
        // Save last index of an match, starting with "0"
        let mut last_index: usize = 0;

        for entry in &self.matches {
            parts.push(&self.src[last_index..entry.full_match_start]); // non-inclusive to only copy up-to just before the starting character of the beginning of the match

            let arg_name = &self.src[entry.value_name_start..entry.value_name_end]; // non-inclusive because regex's "end" referes to the character after the match

            // not using "unwrap_or_else" because of the need to return "Err"
            if let Some(v) = values.get(&arg_name) {
                parts.push(v.as_ref());
            } else {
                if fail {
                    return Err(TemplateError::new(
                        TemplateErrorKind::MissingData,
                        format!("Missing Data for Argument \"{}\"", &arg_name),
                    ));
                }

                // copy the full match in the template into the final string as a fallback if "fail" is "false"
                parts.push(&self.src[entry.full_match_start..entry.full_match_end]);
                // non-inclusive because regex's "end" referes to the character after the match
            }

            last_index = entry.full_match_end;
        }

        // if string is not already fully copied, copy the rest of it
        if last_index < self.src.len() {
            parts.push(&self.src[last_index..self.src.len()]); // non-inclusive because "len" is last index + 1
        }

        return Ok(parts.join(""));
    }

    /// Render the template with the provided values.
    fn render_internal_string<T: AsRef<str>>(
        &self,
        values: &HashMap<String, T>,
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

        for entry in &self.matches {
            parts.push(&self.src[last_index..entry.full_match_start]); // non-inclusive to only copy up-to just before the starting character of the beginning of the match

            let arg_name = &self.src[entry.value_name_start..entry.value_name_end]; // non-inclusive because regex's "end" referes to the character after the match

            // not using "unwrap_or_else" because of the need to return "Err"
            if let Some(v) = values.get(&arg_name.to_string()) {
                parts.push(v.as_ref());
            } else {
                if fail {
                    return Err(TemplateError::new(
                        TemplateErrorKind::MissingData,
                        format!("Missing Data for Argument \"{}\"", &arg_name),
                    ));
                }

                // copy the full match in the template into the final string as a fallback if "fail" is "false"
                parts.push(&self.src[entry.full_match_start..entry.full_match_end]);
                // non-inclusive because regex's "end" referes to the character after the match
            }

            last_index = entry.full_match_end;
        }

        // if string is not already fully copied, copy the rest of it
        if last_index < self.src.len() {
            parts.push(&self.src[last_index..self.src.len()]); // non-inclusive because "len" is last index + 1
        }

        return Ok(parts.join(""));
    }

    /// Render the template with the provided values.
    ///
    /// This function takes a [`HashMap`] where the key is [`str`].
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
    pub fn render<T: AsRef<str>>(&self, values: &HashMap<&str, T>) -> Result<String, TemplateError> {
        return self.render_internal(values, true);
    }

    /// Render the template with the provided values.
    ///
    /// This function takes a [`HashMap`] where the key is [`String`].
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
    ///     map.insert("data1".to_string(), "should");
    ///     map.insert("data2".to_string(), "here");
    ///     map
    /// };
    ///
    /// let rendered = templ.render_string(&data).expect("Expected Result to be Ok");
    /// assert_eq!("Something should be here, and { not here }", rendered);
    /// ```
    pub fn render_string<T: AsRef<str>>(&self, values: &HashMap<String, T>) -> Result<String, TemplateError> {
        return self.render_internal_string(values, true);
    }

    /// Render the template with the provided values.
    ///
    /// This function takes a [`HashMap`] where the key is [`str`].  
    /// This function always returns a [`String`], this function does not error or panic.  
    /// If [`Template::render`] returned a [`Err`], this function will instead return the raw Template string.
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
    #[must_use]
    pub fn render_nofail<T: AsRef<str>>(&self, values: &HashMap<&str, T>) -> String {
        return self
            .render_internal(values, false)
            .unwrap_or_else(|_| return self.src.clone());
    }

    /// Render the template with the provided values.
    ///
    /// This function takes a [`HashMap`] where the key is [`String`].  
    /// This function always returns a [`String`], this function does not error or panic.  
    /// If [`Template::render_string`] returned a [`Err`], this function will instead return the raw Template string.
    /// # Example
    /// ```rust
    /// # use new_string_template::template::*;
    /// # use std::collections::HashMap;
    /// let templ_str = "Something {data1} be {data2}, and { not here }";
    /// let templ = Template::new(templ_str);
    /// let data = {
    ///     let mut map = HashMap::new();
    ///     map.insert("data1".to_string(), "should");
    ///     // map.insert("data2", "here");
    ///     map
    /// };
    ///
    /// let rendered = templ.render_nofail_string(&data);
    /// assert_eq!("Something should be {data2}, and { not here }", rendered);
    /// ```
    #[must_use]
    pub fn render_nofail_string<T: AsRef<str>>(&self, values: &HashMap<String, T>) -> String {
        return self
            .render_internal_string(values, false)
            .unwrap_or_else(|_| return self.src.clone());
    }
}

/// Helper function to execute a [`Regex`] and get all the matches as [`MatchEntry`]
fn get_matches(regex: &Regex, template: &str) -> Vec<MatchEntry> {
    return regex
        .captures_iter(template)
        .map(|found| {
            let full_match = found.get(0).expect("Match Index 0 was None (Full Match)");
            let value_match = found.get(1).expect("Match Index 1 was None (Inner Match)");

            return MatchEntry::new(
                (full_match.start(), full_match.end()),
                (value_match.start(), value_match.end()),
            );
        })
        .collect();
}

#[cfg(test)]
mod test {
    use crate::error::TemplateErrorKind;

    use super::*;

    #[test]
    fn test_regex_end_plus1() {
        // this test tests that "regex::Match::end" returns the index of the next character than the match length
        // see https://github.com/rust-lang/regex/issues/810#issuecomment-948793203
        // see https://github.com/rust-lang/regex/discussions/866
        let string0 = "something { hello } end";

        let cap = DEFAULT_TEMPLATE.captures(string0).expect("expected some captures");
        let f_m = cap.get(0).expect("expected 0");
        let v_m = cap.get(1).expect("expected 1");

        assert_eq!((f_m.start(), f_m.end()), (10, 19));
        assert_eq!((v_m.start(), v_m.end()), (12, 17));
    }

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
    fn test_default_not_greedy() {
        let templ_str = "Something {data1}{data2}, and { not here }";
        let templ = Template::new(templ_str);
        let data = {
            let mut map = HashMap::new();
            map.insert("data1", "20");
            map.insert("data2", "mb");
            map
        };

        let rendered = templ.render(&data).expect("Expected Result to be Ok");
        assert_eq!("Something 20mb, and { not here }", rendered);
    }

    #[test]
    fn test_render_full_no_error_string_value() {
        let templ_str = "Something {data1} be {data2}, and { not here }";
        let templ = Template::new(templ_str);
        let data = {
            let mut map = HashMap::new();
            map.insert("data1", "should".to_string());
            map.insert("data2", "here".to_string());
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
    fn test_render_nofail_string_full_no_error() {
        let templ_str = "Something {data1} be {data2}, and { not here }";
        let templ = Template::new(templ_str);
        let data = {
            let mut map = HashMap::new();
            map.insert("data1".to_string(), "should");
            // map.insert("data2", "here");
            map
        };

        let rendered = templ.render_nofail_string(&data);
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

    #[test]
    fn test_render_full_no_error_string_key() {
        let templ_str = "Something {data1} be {data2}, and { not here }";
        let templ = Template::new(templ_str);
        let data = {
            let mut map = HashMap::new();
            map.insert("data1".to_string(), "should");
            map.insert("data2".to_string(), "here");
            map
        };

        let rendered = templ.render_string(&data).expect("Expected Result to be Ok");
        assert_eq!("Something should be here, and { not here }", rendered);
    }

    #[test]
    fn test_render_full_no_error_string_key_string_value() {
        let templ_str = "Something {data1} be {data2}, and { not here }";
        let templ = Template::new(templ_str);
        let data = {
            let mut map = HashMap::new();
            map.insert("data1".to_string(), "should".to_string());
            map.insert("data2".to_string(), "here".to_string());
            map
        };

        let rendered = templ.render_string(&data).expect("Expected Result to be Ok");
        assert_eq!("Something should be here, and { not here }", rendered);
    }
}
