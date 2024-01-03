#!/usr/bin/env just --justfile
set dotenv-load := true

list:
    @just --list

help:
    @just list

generate-opinonated-binary:
    cargo generate --path . --bin --name "template-project" -d license="both" -d project-description="Example binary project using my personal template" -d nightly=true -d protect-main-branch=true -d gh-username=milho

generate:
    @just clean
    @just generate-opinonated-binary

clean:
    rm -rf {{binary-name}}

pre-commit:
    @just generate
    cd {{binary-name}} && just ci
    @just clean

ci:
    @just generate
    cd {{binary-name}} && just init && just ci
