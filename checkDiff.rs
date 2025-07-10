// Add these new structs to backend/src/models.rs

/// Represents a single added line of code in a diff.
#[derive(Debug)]
 which is part of a larger project to re-implement Git in pure Rust. It's powerful and will teach us morepub struct LineChange {
    pub line_number: u32,
    pub content: String,
 about the underlying concepts.

*Wait, let's reconsider. `gix-diff` is powerful but very}

/// Represents all the changes within a single file in a diff.
#[derive(Debug)]
pub struct FileChange {
    pub file_path: String,
    pub added_lines: Vec<LineChange>,
}
 low-level. For our learning purposes, a simpler crate like **`patch`** is a much better starting point. It'```

**2. Create a New `diff_parser` Module**

Just as we did for clients, let's keeps designed specifically for parsing `.diff` / `.patch` files.*

**1. Add the `patch` Crate**

*   Open your `backend/Cargo.toml` file.
*   Add the `patch` crate to our code clean by creating a dedicated module for this new responsibility.

1.  In `backend/src`, create a new file named `diff_parser.rs`.
2.  In `backend/src/main.rs`, declare the new your `[dependencies]`.
    ```toml
    [dependencies]
    # ... all your other dependencies
 module:
    ```rust
    // in backend/src/main.rs
    mod clients;
        patch = "0.7"
    ```

**2. Create New Data Structures**

Let's define whatmod diff_parser; // Add this line
    mod handlers;
    // ...
    ```

**3. Implement the Diff Parsing Logic**

Now for the core logic. We'll write a function that iterates through the diff line we want our parsed diff to look like. It's good practice to create our own structs rather than depending on the library's structs everywhere in our code.

*   Open `backend/src/models.rs`.
*    by line and builds up our `Vec<FileChange>` structure.

*   Open the new `backend/src/diff_parser.rs` file.
*   Paste the following code into it:

```rust
// in backend/src/Add these new structs to the file. They will represent a parsed diff.

    ```rust
    // Add these to backend/diff_parser.rs

use crate::models::{FileChange, LineChange};

pub fn parse(diff_text:src/models.rs

    /// Represents a single line of code that was added in a diff.
    #[ &str) -> Vec<FileChange> {
    let mut file_changes = Vec::new();
    let mut currentderive(Debug, Clone)]
    pub struct AddedLine {
        pub file_path: String,
        // The line_file_change: Option<FileChange> = None;
    let mut new_line_number: u32 = number in the *new* version of the file.
        pub line_number: u32,
        // 0;

    for line in diff_text.lines() {
        // --- Detect a new file section The actual text content of the line, without the leading '+'.
        pub content: String,
    }
 ---
        // A line like "diff --git a/src/main.rs b/src/main.rs"    
    /// Represents all the added lines from a parsed diff, grouped by file.
    #[derive(Debug, marks a new file.
        if line.starts_with("diff --git") {
            // If we Clone)]
    pub struct ParsedDiff {
        // A vector containing all the lines that were added across all files.
 were tracking a previous file, save it before starting a new one.
            if let Some(finished_file) =        pub added_lines: Vec<AddedLine>,
    }
    ```

**3. Create the Diff current_file_change.take() {
                if !finished_file.added_lines.is_empty() { Parsing Logic**

Now, let's create a new function that takes the raw diff string and returns our `ParsedDiff`
                    file_changes.push(finished_file);
                }
            }
            
            // Extract the new struct. We'll add this to a new, dedicated module for this kind of logic.

*   In `backend/src file path. It's the part after "b/".
            let path = line.split_whitespace().nth(`, create a new file named `code_parser.rs`.
*   In `backend/src/main.3).unwrap_or("").strip_prefix("b/").unwrap_or("").to_string();
            currentrs`, register the new module:
    ```rust
    // in main.rs
    mod clients;
_file_change = Some(FileChange {
                file_path: path,
                added_lines:    mod code_parser; // <-- Add this
    mod handlers;
    // ...
    ```
*   Now Vec::new(),
            });
            continue; // Move to the next line
        }

        // --- Detect, add the parsing function to `backend/src/code_parser.rs`:

    ```rust
    // in the line number information ---
        // A line like "@@ -1,5 +2,6 @@" tells us backend/src/code_parser.rs

    use crate::models::{AddedLine, ParsedDiff};
 where the changes start.
        if line.starts_with("@@") {
            // Extract the part after    use patch::Patch;

    pub fn parse_diff_to_added_lines(diff_text: & the `+`, e.g., "2,6".
            let parts: Vec<&str> = linestr) -> Result<ParsedDiff, String> {
        // The `patch` crate works with bytes, so we convert.split_whitespace().collect();
            if let Some(line_info) = parts.get(2) { our string.
        let patch_set = Patch::from_bytes(diff_text.as_bytes())
                // The number after '+' and before ',' is the starting line number.
                if let Some(num_str) =
            .map_err(|e| format!("Failed to parse diff: {}", e))?;

        let mut all line_info.strip_prefix('+') {
                    if let Some(num) = num_str.split(',')._added_lines = Vec::new();

        // A diff can contain changes for multiple files. We iterate through each onenext() {
                         // Parse the string into a number. Default to 0 on failure.
                        new_line_number.
        for file_patch in patch_set {
            // Get the path of the file being changed.
             = num.parse::<u32>().unwrap_or(0);
                    }
                }
            }
// We use `new_file` because we care about the state *after* the change.
            let file_path =            continue; // This line is just metadata, so we skip to the next.
        }
        
        // --- Detect added lines and other lines ---
        if line.starts_with('+') && !line.starts_with("+++ file_patch.new.path.to_string_lossy().into_owned();

            // A file patch contains multiple "hunks" or sections of changes.
            for hunk in &file_patch.hunks {
                //") {
            // This is an added line of code.
            if let Some(file) = current_file Each hunk contains multiple lines.
                for line in &hunk.lines {
                    // We only care about lines_change.as_mut() {
                file.added_lines.push(LineChange {
                    line_number: that were ADDED.
                    if let patch::Line::Add(content) = line {
                        // The new_line_number,
                    content: line[1..].to_string(), // The content without the '+'
                });
            }
            new_line_number += 1;
        } else if !line.starts_ line number is part of the hunk's metadata.
                        // We need to calculate the specific line number for thiswith('-') {
            // This is a context line (starts with a space) or a file header.
             added line.
                        // This requires tracking the line number within the hunk.
                        // Let's defer this complex// For any line that isn't a deletion, we increment the line counter.
            new_line_number += calculation for a moment and use a placeholder.
                        // **This is a deliberate simplification for Step 1.**
                        
 1;
        }
    }

    // After the loop, don't forget to save the last file                        all_added_lines.push(AddedLine {
                            file_path: file_path.clone(),
                            // TODO: Implement accurate line number calculation.
                            line_number: hunk.new_range.start as u3 we were processing.
    if let Some(finished_file) = current_file_change.take() {
        if !finished_file.added_lines.is_empty() {
            file_changes.push(2, 
                            content: String::from_utf8_lossy(content).to_string(),
finished_file);
        }
    }

    file_changes
}
