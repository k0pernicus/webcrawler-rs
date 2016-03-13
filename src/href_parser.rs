use regex::Regex;

/// Structure to represents the list of all external references from a web page given by an URL
pub struct ParsedTree<'a> {
    /// The URL of the web page to parse
    url : &'a str,
    /// The list of references
    refs : Vec<String>,
}

/// The implementation of the structure
impl<'a> ParsedTree<'a> {

    pub fn new(url : &'a str) -> ParsedTree {
        ParsedTree {
            url : url,
            refs : Vec::new()
        }
    }

    /// Let's parse the web page!
    pub fn run(&mut self, xml_structure: &str) {
        let mut next_to_tag = false;
        let mut current_searching_for_tag = false;
        let mut current_tag = String::new();
        let mut current_href = false;
        let mut current_href_content = String::new();
        for character in xml_structure.chars() {
            // <a href="lol"> hkjfdlfjdkljfdklfjkfldjfkld </a>
            match character {
                '<' => {
                    if current_href {
                        current_href_content.push('<');
                    }
                    next_to_tag = true;
                    current_searching_for_tag = true;
                },
                '/' => {
                    if next_to_tag {
                        current_href = false;
                        current_searching_for_tag = false;
                        if current_href_content != "" {
                            current_href_content.push_str("/a>");
                            self.refs.push(current_href_content.clone());
                        }
                        current_href_content.clear();
                    }
                    else {
                        if current_href {
                            current_href_content.push('/')
                        }
                    }
                    next_to_tag = false;
                },
                '>' => {
                    next_to_tag = false;
                    if current_href {
                        current_href_content.push('>');
                    }
                    current_tag.clear();
                }
                ' ' => {
                    next_to_tag = false;
                    if current_searching_for_tag {
                        current_searching_for_tag = false;
                        // self.add_tag(current_tag.clone(), String::new());

                        if current_tag == "a" {
                            current_href = true;
                            current_href_content = "<a".to_string();
                        }

                        current_tag.clear();
                    }

                    if current_href {
                        current_href_content.push(' ');
                    }
                },
                _ => {
                    next_to_tag = false;
                    if current_searching_for_tag {
                        current_tag.push(character);
                    }
                    if current_href {
                        current_href_content.push(character);
                    }
                },
            }
        }
    }

    /// A method to get and to filter external references from the web page
    pub fn get_external_references(&mut self) -> Vec<String>{

        let href_regex = Regex::new(r###"href="(?P<reference>[^"]*)"###).unwrap();

        let hrefs = self.refs
            .iter()
            .filter(|s| href_regex.is_match(s))
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        hrefs
            .iter()
            .map(|s| href_regex.captures(&s).unwrap().name("reference").unwrap())
            .filter(|s| s.starts_with("http"))
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
    }

}
