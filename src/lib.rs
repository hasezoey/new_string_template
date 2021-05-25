use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DEFAULT_TEMPLATE: Regex = Regex::new(r"(?mi)\{\s*(\S+)\s*\}").unwrap();
}

pub struct Template {
    /// Template String
    src: String,
    /// All matches from the Template String
    matches: Vec<(usize, usize)>,
}

impl Template {
    /// Create a new Template Instance with the default regex
    /// # Example
    /// ```rust
    /// # use new_string_template::*;
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
    /// # Example
    /// ```rust
    /// # use new_string_template::*;
    /// # use regex::Regex;
    /// # let template_string = "hello";
    /// # let custom_regex = Regex::new(r"").unwrap();
    /// let templ = Template::new(template_string).with_regex(&custom_regex);
    /// ```
    pub fn with_regex(mut self, regex: &Regex) -> Self {
        self.matches = get_matches(&regex, &self.src);

        return self;
    }

    /// Render the template with the provided values
    pub fn render(&self, _values: &HashMap<&str, &str>) -> String {
        // TODO: implement the "render" function
        panic!("render is not implement yet");
    }
}

fn get_matches(regex: &Regex, template: &str) -> Vec<(usize, usize)> {
    return regex
        .find_iter(&template)
        .map(|found| return (found.start(), found.end()))
        .collect();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_something() {
        let templ = "Something here";
        Template::new(templ);
        println!("{}", templ);
    }
}
