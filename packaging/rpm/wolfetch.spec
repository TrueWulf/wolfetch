Name: wolfetch
Version: 0.5.0
%global pre_release pre.1
Release: 0.1%{?dist}
Summary: Tiny and fast system fetch for Linux and BSD
License: GPL-3.0-only
URL: https://github.com/TrueWulf/wolfetch
Source0: %{name}-%{version}.tar.gz

%description
wolfetch prints a concise system summary beside a minimal wolf. It does not
require systemd or a background service.

%prep
%autosetup

%build
cargo build --release --locked

%install
install -Dpm0755 target/release/wolfetch %{buildroot}%{_bindir}/wolfetch
install -Dpm0644 LICENSE %{buildroot}%{_licensedir}/%{name}/LICENSE
install -Dpm0644 config.example %{buildroot}%{_docdir}/%{name}/config.example

%files
%{_bindir}/wolfetch
%{_licensedir}/%{name}/LICENSE
%{_docdir}/%{name}/config.example
