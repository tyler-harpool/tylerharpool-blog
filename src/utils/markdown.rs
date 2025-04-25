use pulldown_cmark::{Parser, Options, html};


pub fn markdown_to_html(content: &str) -> String {
    // Set up options for GitHub-flavored markdown
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    // Pre-process Obsidian wiki links
    let processed_content = process_obsidian_syntax(content);

    // Parse the markdown
    let parser = Parser::new_ext(&processed_content, options);

    // Write to string buffer
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}
fn html_escape(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}
fn process_obsidian_syntax(content: &str) -> String {
    let mut result = String::new();

    // Process line by line
    for line in content.lines() {
        let processed_line = process_wiki_links(line);
        result.push_str(&processed_line);
        result.push('\n');
    }

    result
}
fn process_wiki_links(line: &str) -> String {
    let mut result = String::new();
    let mut current_pos = 0;

    // Scan for [[ patterns
    while let Some(start_pos) = line[current_pos..].find("[[") {
        // Add text before the wiki link
        result.push_str(&line[current_pos..current_pos + start_pos]);

        // Move past the [[ characters
        let link_start = current_pos + start_pos + 2;

        // Find the closing ]]
        if let Some(end_pos) = line[link_start..].find("]]") {
            let link_content = &line[link_start..link_start + end_pos];

            // Check for pipe character for display text
            if let Some(pipe_pos) = link_content.find('|') {
                let target = &link_content[0..pipe_pos];
                let display = &link_content[pipe_pos + 1..];

                // Create a regular markdown link
                result.push_str(&format!("[{}](/notes/{})",
                    display,
                    target.replace(' ', "-").to_lowercase()));
            } else {
                // No display text, use the link content as is
                result.push_str(&format!("[{}](/notes/{})",
                    link_content,
                    link_content.replace(' ', "-").to_lowercase()));
            }

            // Move past the ]] characters
            current_pos = link_start + end_pos + 2;
        } else {
            // No closing ]], just add the [[ and continue
            result.push_str("[[");
            current_pos = link_start;
        }
    }

    // Add any remaining text
    result.push_str(&line[current_pos..]);

    result
}

fn process_obsidian_links(text: &str) -> String {
    let mut result = String::with_capacity(text.len() * 2);
    let mut chars = text.char_indices();
    let mut last_end = 0;

    while let Some((i, c)) = chars.next() {
        if c == '[' && i + 1 < text.len() && text[i+1..].starts_with("[") {
            // Add text before the link
            result.push_str(&text[last_end..i]);

            let start = i + 2;

            // Find the end of the link (closing brackets)
            let mut end_index = None;
            let mut bracket_chars = text[start..].char_indices();
            while let Some((j, ch)) = bracket_chars.next() {
                if ch == ']' && j + 1 < text[start..].len() && text[start+j+1..].starts_with("]") {
                    end_index = Some(start + j);
                    break;
                }
            }

            if let Some(end) = end_index {
                // Extract the link text
                let link_text = &text[start..end];

                // Check if there's a pipe for display text
                if let Some(pipe_pos) = link_text.find('|') {
                    let link = &link_text[0..pipe_pos];
                    let display = &link_text[pipe_pos+1..];

                    // Create an HTML link
                    result.push_str(&format!("<a href=\"/notes/{}\">{}</a>",
                        link.replace(' ', "-").to_lowercase(),
                        display));
                } else {
                    // No pipe, use the link text as display text
                    result.push_str(&format!("<a href=\"/notes/{}\">{}</a>",
                        link_text.replace(' ', "-").to_lowercase(),
                        link_text));
                }

                // Skip ahead past the closing brackets
                last_end = end + 2;
                // Find and skip characters until we reach last_end
                while let Some((i, _)) = chars.next() {
                    if i >= last_end {
                        break;
                    }
                }
            } else {
                // No closing brackets found, just add the opening bracket as-is
                result.push('[');
                last_end = i + 1;
            }
        }
    }

    // Add any remaining text
    if last_end < text.len() {
        result.push_str(&text[last_end..]);
    }

    result
}

fn process_inline_formatting(text: &str) -> String {
    // If text might contain HTML or special characters, escape it first
    let escaped_text = html_escape(text);

    // Process markdown-style formatting
    let text_with_links = replace_markdown_links(&escaped_text);
    let text_with_bold = replace_bold(&text_with_links);
    let text_with_italic = replace_italic(&text_with_bold);

    text_with_italic
}

fn replace_markdown_links(text: &str) -> String {
    let mut result = String::with_capacity(text.len() * 2);
    let mut chars = text.char_indices();
    let mut last_end = 0;

    while let Some((i, c)) = chars.next() {
        if c == '[' {
            // Find the closing bracket
            let mut text_end = None;
            let mut bracket_depth = 1;
            let mut bracket_chars = text[i+1..].char_indices();

            while let Some((j, ch)) = bracket_chars.next() {
                if ch == '[' {
                    bracket_depth += 1;
                } else if ch == ']' {
                    bracket_depth -= 1;
                    if bracket_depth == 0 {
                        text_end = Some(i + 1 + j);
                        break;
                    }
                }
            }

            if let Some(text_end) = text_end {
                // Check if followed by (url)
                if text_end + 1 < text.len() && text.chars().nth(text_end + 1) == Some('(') {
                    // Find the closing parenthesis
                    let mut url_end = None;
                    let mut paren_depth = 1;
                    let mut paren_chars = text[text_end+2..].char_indices();

                    while let Some((j, ch)) = paren_chars.next() {
                        if ch == '(' {
                            paren_depth += 1;
                        } else if ch == ')' {
                            paren_depth -= 1;
                            if paren_depth == 0 {
                                url_end = Some(text_end + 2 + j);
                                break;
                            }
                        }
                    }

                    if let Some(url_end) = url_end {
                        // Add text before the link
                        result.push_str(&text[last_end..i]);

                        // Extract the link text and URL
                        let link_text = &text[i+1..text_end];
                        let url = &text[text_end+2..url_end];

                        // Create HTML link
                        result.push_str(&format!("<a href=\"{}\">{}</a>", url, link_text));

                        // Update position
                        last_end = url_end + 1;
                        // Skip characters we've processed
                        while let Some((idx, _)) = chars.next() {
                            if idx >= last_end {
                                break;
                            }
                        }
                        continue;
                    }
                }
            }
        }

        // If we get here, we didn't process a link at the current position
        if last_end <= i {
            result.push(c);
        }
    }

    // Add any remaining text (if we ended without processing a link)
    if last_end < text.len() && last_end > 0 {
        result.push_str(&text[last_end..]);
    }

    // If we didn't process any links, return the original text
    if result.is_empty() {
        text.to_string()
    } else {
        result
    }
}

fn replace_bold(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut in_bold = false;
    let mut i = 0;

    while i < text.len() {
        // Check for "**" pattern
        if i + 1 < text.len() && &text[i..i+2] == "**" {
            if in_bold {
                result.push_str("</strong>");
                in_bold = false;
                i += 2;
            } else {
                result.push_str("<strong>");
                in_bold = true;
                i += 2;
            }
        } else if i < text.len() {
            // Get the character at index i
            let c = text[i..].chars().next().unwrap();
            result.push(c);
            i += c.len_utf8();
        } else {
            break;
        }
    }

    // Close any open tags
    if in_bold {
        result.push_str("</strong>");
    }

    result
}

fn replace_italic(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut in_italic = false;
    let mut i = 0;

    while i < text.len() {
        // Check for single "*" pattern that's not "**"
        if i < text.len() && text[i..].starts_with("*") &&
           !(i + 1 < text.len() && text[i+1..].starts_with("*")) {
            if in_italic {
                result.push_str("</em>");
                in_italic = false;
                i += 1;
            } else {
                result.push_str("<em>");
                in_italic = true;
                i += 1;
            }
        } else if i < text.len() {
            // Get the character at index i
            let c = text[i..].chars().next().unwrap();
            result.push(c);
            i += c.len_utf8();
        } else {
            break;
        }
    }

    // Close any open tags
    if in_italic {
        result.push_str("</em>");
    }

    result
}
