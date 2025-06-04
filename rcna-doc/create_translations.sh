#!/bin/bash

# Script to create missing translation files for Zola multilingual site
# Supports: cz, de, es, fr, it, pl, uk language codes

# Define the content directory
CONTENT_DIR="./content"
# Define supported languages
LANGUAGES=("cz" "de" "es" "fr" "it" "pl" "uk")

# Function to create missing translation files
create_missing_translations() {
  local dir="$1"

  # Find all markdown files in the default language (without language suffix)
  find "$dir" -type f -name "*.md" | grep -v "\.[a-z][a-z]\.md$" | while read -r default_file; do
    dir_path=$(dirname "$default_file")
    filename=$(basename "$default_file")
    filename_base="${filename%.md}"

    # For each supported language
    for lang in "${LANGUAGES[@]}"; do
      # Check if translation file exists
      translation_file="${dir_path}/${filename_base}.${lang}.md"

      if [ ! -f "$translation_file" ]; then
        echo "Creating missing translation file: $translation_file"

        # Copy the content from the default file
        cp "$default_file" "$translation_file"
      fi
    done
  done

  # Process subdirectories recursively
  find "$dir" -type d | while read -r subdir; do
    if [ "$subdir" != "$dir" ]; then
      create_missing_translations "$subdir"
    fi
  done
}

# Main execution
echo "Starting translation file creation process..."
create_missing_translations "$CONTENT_DIR"
echo "Translation file creation complete!"

# Count the number of files created
echo "Summary:"
for lang in "${LANGUAGES[@]}"; do
  count=$(find "$CONTENT_DIR" -name "*.${lang}.md" | wc -l)
  echo "  - ${lang}: $count files"
done

echo "Default language: $(find "$CONTENT_DIR" -type f -name "*.md" | grep -c "\.[a-z][a-z]\.md$") files"
