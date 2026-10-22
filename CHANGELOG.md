## v1.1.0

Additions:

- Added `csv` output type
- Added `--output-path` optional flag for customing output directory
- Added `--ignore` optional flag for ignoring files
- Switch to file write streaming as opposed to string building in memory
- Switch to HashMap for files and folders in `FolderData` for better management

Also refactoring, such as improvements to naming and code organisation.

## v1.0.0

Initial release.

Supports output types of:

- `summary`
- `html`

CLI format: `lovecov <file> <output (optional)>`
