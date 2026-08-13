Name: wolfetch
Version: 0.5.0
Release: 0.1.pre4%{?dist}
Summary: Tiny and fast system fetch for Linux and BSD
License: GPL-3.0-only
URL: https://github.com/TrueWulf/wolfetch
Source0: https://github.com/TrueWulf/wolfetch/archive/refs/tags/v%{version}-pre.4.tar.gz

%description
wolfetch prints a concise system summary beside a minimal wolf. It does not
require systemd or a background service.

%prep
%autosetup

%build
cargo build --release --locked

%install
install -Dpm0755 target/release/wolfetch %{buildroot}%{_bindir}/wolfetch
install -Dpm0755 target/release/wfetch %{buildroot}%{_bindir}/wfetch
install -Dpm0644 LICENSE %{buildroot}%{_licensedir}/%{name}/LICENSE
install -Dpm0644 config.example %{buildroot}%{_docdir}/%{name}/config.example
install -Dpm0644 man/wolfetch.1 %{buildroot}%{_mandir}/man1/wolfetch.1
install -Dpm0644 man/wolfetch.5 %{buildroot}%{_mandir}/man5/wolfetch.5

%files
%{_bindir}/wolfetch
%{_bindir}/wfetch
%{_licensedir}/%{name}/LICENSE
%{_docdir}/%{name}/config.example
