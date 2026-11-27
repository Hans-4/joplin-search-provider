Name:           joplin-search-provider
Version:        1.1.0
Release:        1%{?dist}
Summary:        GNOME Shell Search Provider for Joplin notes

License:        MIT
URL:            https://github.com/Hans-4/joplin-search-provider
Source0:        %{url}/archive/refs/tags/v%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  gcc
BuildRequires:  pkgconfig(gio-2.0)
BuildRequires:  pkgconfig(gobject-2.0)
BuildRequires:  pkgconfig(sqlite3)

%description
GNOME Shell Search Provider that integrates Joplin note search into
GNOME Shell's Activities overview.

%prep
%autosetup -p1 -n %{name}-%{version}

%build
cargo build --release

%install
rm -rf %{buildroot}
install -Dm0755 target/release/joplin-search-provider \
    %{buildroot}%{_bindir}/joplin-search-provider
install -Dm0644 data/io.github.hans4.JoplinSearchProvider.search-provider.ini \
    %{buildroot}%{_datadir}/gnome-shell/search-providers/io.github.hans4.JoplinSearchProvider.search-provider.ini
install -Dm0644 data/io.github.hans4.JoplinSearchProvider.service \
    %{buildroot}%{_datadir}/dbus-1/services/io.github.hans4.JoplinSearchProvider.service

%files
%license LICENSE
%{_bindir}/joplin-search-provider
%{_datadir}/dbus-1/services/io.github.hans4.JoplinSearchProvider.service
%{_datadir}/gnome-shell/search-providers/io.github.hans4.JoplinSearchProvider.search-provider.ini

%changelog
* Fr Nov 27 2026 Hans-4 <hannes.dev@gmx.de> - 1.1.0-1
- Add folders and tags to search

* Tue Nov 24 2026 Hans-4 <hannes.dev@gmx.de> - 1.0.0-1
- Initial package