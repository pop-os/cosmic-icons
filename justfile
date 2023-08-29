set export
rootdir := ''
prefix := '/usr'
base-dir := absolute_path(clean(rootdir / prefix))
install-dir := base-dir / 'share'
icons := install-dir / 'icons' / 'Cosmic'
export ICONS := icons

default:
    @just --list

install-freedesktop:
    @just freedesktop/install

install-extra:
    @just extra/install

install-index:
    install -Dm0644 ./index.theme {{icons}}/index.theme

# Install everything
install: (install-freedesktop) (install-extra) (install-index)

# Uninstalls everything (requires same arguments as given to install)
uninstall:
    rm -rf {{icons}}

# Show the current git commit
git-rev:
    @git rev-parse --short HEAD
