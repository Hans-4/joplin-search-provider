# Joplin Search Provider

By default, Joplin doesn't integrate into the Gnome Shell search. This simple package, written in Rust, fixes that.

## Installation

### From RPM Package (Fedora)

1. Enable the COPR repository:
   ```bash
    sudo dnf copr enable hans-4/joplin-search-provider
   ```
2. Install the package:
    ```bash
    sudo dnf install joplin-search-provider
    ```
3. You may have to set a [custom db path](#set-db-path)

### Manual Installation
1. Build the project:
    ```bash
    cargo build --release
    ```
2. Install the binary:
    ```bash
    sudo cp target/release/joplin-search-provider /usr/bin/
    ```
3. Install the D-Bus service file:
    ```bash
    sudo cp data/io.github.hans4.JoplinSearchProvider.service /usr/share/dbus-1/services/
    ```
4. Install the search provider configuration:
    ```
    sudo cp data/io.github.hans4.JoplinSearchProvider.search-provider.ini /usr/share/gnome-shell/search-providers/
    ```
5. You may have to set a [custom db path](#set-db-path)

### Set db path
By default, the search provider looks for the Joplin database at:
```
/home/YOUR_USERNAME/.config/joplin-desktop/database.sqlite
```
If your database is in a different location, set the `JOPLIN_DB_PATH` environment variable:
```
export JOPLIN_DB_PATH="/path/to/your/joplin/database.sqlite"
```
To make it permanent, add it to your `~/.bashrc`
