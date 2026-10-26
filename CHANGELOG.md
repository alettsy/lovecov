## v1.2.0

Additions:

- Escaped HTML to prevent XSS
- Added colors to better reflect coverage state
- Colored the expansion icon to make it clearer which items are folders or files
- Add HTML progress bars option `--html-use-bars` to replace raw percentage, if desired

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
