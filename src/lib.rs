use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use regex::Regex;

#[derive(Default)]
pub struct IncludeCheck;

impl IncludeCheck {
    pub fn new() -> IncludeCheck {
        IncludeCheck
    }
}

impl Preprocessor for IncludeCheck {
    fn name(&self) -> &str {
        "include-check"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
        let regex = Regex::new(r"\{\{\s*#(rustdoc_)?include\s+([\.[:word:][:space:]/]+\.rs)")?;

        if let Some(_cfg) = ctx.config.get_preprocessor(self.name()) {
            // No configuration value for the moment.
        }

        for item in book.iter() {
            if let BookItem::Chapter(Chapter { source_path: Some(source_path), content, .. }) = item {
                // TODO check regex for ALL matches
                for captures in regex.captures_iter(content) {
                    let mut md_path = ctx.root.clone();
                    md_path.push(ctx.config.book.src.clone());
                    md_path.push(source_path);

                    let md_time = if let Ok(metadata) = std::fs::metadata(&md_path) {
                        if let Ok(time) = metadata.modified() {
                            time
                        } else {
                            anyhow::bail!("Couldn't get modified time of markdown file");
                        }
                    } else {
                        anyhow::bail!("Markdown file not found: {:?}", md_path);
                    };

                    let mut rust_path = ctx.root.clone();
                    rust_path.push(ctx.config.book.src.clone());
                    rust_path.push(captures.get(2).unwrap().as_str());

                    let rust_time = if let Ok(metadata) = std::fs::metadata(&rust_path) {
                        if let Ok(time) = metadata.modified() {
                            time
                        } else {
                            anyhow::bail!("Couldn't get modified time of rust file");
                        }
                    } else {
                        anyhow::bail!("Rust file not found: {:?}", rust_path);
                    };

                    if rust_time > md_time {
                        anyhow::bail!("Source file({:?}) is more recent than book ({:?}) file, make sure everything is properly synced!", rust_path, md_path);
                    }
                }
            }
        }

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        // Should support all renderers as it's only a background check for included strings
        renderer != "not-supported"
    }
}
