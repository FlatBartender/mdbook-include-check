mdBook include checker
----

This is a very simple mdbook preprocessor I made to not generate the mdbook for a project if the documentation book may not be in sync with the source code.

In my case, I need to communicate some algorithmic details to people who do not know Rust and may not want to learn it either.

Documentation like this is susceptible to rotting if the code is updated but not the documentation. This preprocessor provides a way to warn the user if the book isn't up to date.

How it works
----

It scans every chapter in the book for the regex `\{\{\s*#(rustdoc_)?include\s+([\.[:word:][:space:]/]+\.rs)`, extracts capture group 2 (the supposed source file path), and check that file's modified time against the current chapter's source file's modified time

This will match the following regexes:
```
{{#include test.rs}}
{{ #rustdoc_include	filename with spaces.rs }}
{{#include ../src/module/a.rs}}
{{#include ../src/module/with spaces/a.rs}}
```

Caveats
----

Currently I have not tested this tool with a freshly repository. As far as I know, git does not preserve modified time, so it might cause issues with that.

How to use
----

Add the following to `book.toml`:
```toml
[preprocessor.include-checks]
command = "mdbook-include-warn"
before = ["links"]
```

