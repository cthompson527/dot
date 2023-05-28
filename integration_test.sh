#!/usr/bin/env bash

set -xeuo pipefail

cargo build --release
DOT="$(pwd)/target/release/dot"

## Create an integration test by:
##   1. Creating a fake folder structure with some config files
##   2. Comming the fake folder structure as a git repo
##   3. Setting up a new HOME variable
##   4. Cloning the fake repo into $HOME/dotfiles with dot
##   5. Verify all of the symlinks are correct
##   6. Clean up test directory

mkdir integration_test
pushd integration_test


################################################
########  1. Create fake folder  ###############
################################################
mkdir fake_repo
pushd fake_repo
mkdir -p "HOME/.config/fish"
touch "HOME/.config/fish/config.fish"
touch "HOME/.vimrc"
touch "HOME/.bashrc"
touch "HOME/.zshrc"

mkdir -p "HOME/.doom.d/"
touch "HOME/.doom.d/config.el"
touch "HOME/.doom.d/packages.el"
touch "HOME/.doom.d/init.el"


################################################
########  2. Commit as git repo  ###############
################################################
git init
git config user.email "fake@fakeuser.com"
git config user.name "Fake User"
git add .
git commit -m "Initial fake setup"
popd


################################################
########  3. Setup fake HOME ###################
################################################
mkdir home
pushd home
export HOME=$(pwd)
popd


################################################
########  4. Clone fake repo ###################
################################################
$DOT init ./fake_repo


################################################
########  5. Verify all links ##################
################################################
pushd home
[ -L ".config/fish/config.fish" ]
[ -L ".vimrc" ]
[ -L ".bashrc" ]
[ -L ".zshrc" ]
[ -L ".doom.d/config.el" ]
[ -L ".doom.d/packages.el" ]
[ -L ".doom.d/init.el" ]
popd

################################################
########  6. Clean test directory ##############
################################################
popd
rm -rf integration_test
