# Plan for Rview

Create a tool that aggregates multiple modified files into one file.

## How would it work?

It will be a CLI tool that gets modified files from "git status" and will read every file that was modified and compile it in a single file named "code-review.txt" that is placed in "docs" directory.
It will output to you what files has it chosen, like "Added constants/colors.ts" and will tell you where the file is located "Code review file created at relative/path/to/file"

### Arguments

It must include the ability to:

- define the output file (rview -o docs2/custom-code-review-name.txt)
- ignore some files and directories like package.json, .gitignore via arguments (rview --exclude={.gitignore ,package.json})
- include a quick help when typying "rview -h" or "rviwe --help" that list our features.


**Example of format of output file**:

```txt
========== main.rs ==========
    *the file content from main.rs*

========== .gitignore ==========
    *the file content from .gitignore*

```

### Future considerations

- [ ] Support Windows apart from Unix like systems.
- [ ] Making it more modular
- [ ] Configuration file support (ENV variable pointing to $XDG_CONFIG_HOME/rview.toml)
- [ ] Configurable separator characters and length (default separator '=' with length 10)
