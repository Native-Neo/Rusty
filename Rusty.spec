Name:           Rusty
Version:        0.1.0
Release:        1%{?dist}
Summary:        A Rust based Calculator
License:        MIT
Source0:        %{name}-%{version}.tar.gz
BuildRequires:  cargo, rust, cargo-generate-rpm

%description
Rusty

%prep
%setup -q

%build
cargo build --release
cargo-generate-rpm

%install
# Mimic what cargo-generate-rpm does or extract from target/generate-rpm
mkdir -p %{buildroot}/usr/bin
cp target/release/Rusty %{buildroot}/usr/bin/

%files
/usr/bin/Rusty

%changelog
* Sun Jul 05 2026 Native <yashyt6246@gmail.com> - 0.1.0-1
- Made SRPM
