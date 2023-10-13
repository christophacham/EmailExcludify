use std::collections::HashSet;
use std::fs;
use regex::Regex;

fn extract_unique_emails(text: &str) -> HashSet<String> {
    let re = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
    re.captures_iter(text)
        .map(|cap| cap[0].to_string())
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the input text and blacklist text from files
    let input_text = fs::read_to_string("input.txt")?;
    let blacklist_text = fs::read_to_string("blacklist.txt")?;

    // Extract unique emails from the input text and blacklist text
    let mut emails = extract_unique_emails(&input_text);
    let blacklist_emails = extract_unique_emails(&blacklist_text);

    // Remove blacklisted emails from the main email set
    emails = emails.difference(&blacklist_emails).cloned().collect();

    // Convert the HashSet to a Vec and sort it
    let mut sorted_emails: Vec<String> = emails.into_iter().collect();
    sorted_emails.sort();

    // Convert the Vec to a single string of comma-separated emails
    let output = sorted_emails.join(";");

    // Write the result to an output file
    fs::write("output.txt", output)?;

    println!("Emails extracted and saved to output.txt");
    Ok(())
}
