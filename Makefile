.PHONY: help clean clean-build clean-pyc clean-test clean-docs clean-venv lint coverage test test-all dev venv docs servedocs dist
.DEFAULT_GOAL := help
define BROWSER_PYSCRIPT
import os, webbrowser, sys
try:
	from urllib import pathname2url
except:
	from urllib.request import pathname2url

webbrowser.open("file://" + pathname2url(os.path.abspath(sys.argv[1])))
endef
export BROWSER_PYSCRIPT

define PRINT_HELP_PYSCRIPT
import re, sys

for line in sys.stdin:
	match = re.match(r'^([a-zA-Z_-]+):.*?## (.*)$$', line)
	if match:
		target, help = match.groups()
		print("%-20s %s" % (target, help))
endef
export PRINT_HELP_PYSCRIPT
BROWSER := python -c "$$BROWSER_PYSCRIPT"

help:
	@python -c "$$PRINT_HELP_PYSCRIPT" < $(MAKEFILE_LIST)

clean: clean-build clean-pyc clean-test clean-docs clean-venv  ## remove all builds, artifacts and local virtualenv

clean-build: ## remove build artifacts
	rm -fr target/
	rm -f python/rgeocoder/*.so
	rm -f *.so *.dylib *.dll
	find . -name '*.egg-info' -exec rm -fr {} +
	find . -name '*.egg' -exec rm -fr {} +

clean-pyc: ## remove Python file artifacts
	find . -name '*.pyc' -exec rm -f {} +
	find . -name '*.pyo' -exec rm -f {} +
	find . -name '*~' -exec rm -f {} +
	find . -name '__pycache__' -exec rm -fr {} +

clean-test: ## remove test and coverage artifacts
	rm -rf .pytest_cache
	rm -fr .tox/
	rm -f .coverage
	rm -fr htmlcov/

clean-docs: ## remove html documentation files built by pdoc
	rm -rf _docs

clean-venv: ## remove the local virtual environment
	rm -rf .venv

lint: venv ## check for formatting issues with flake8
	.venv/bin/python -m flake8 python/rgeocoder tests

coverage: venv dev ## check test coverage
	.venv/bin/python -m coverage run --source rgeocoder -m pytest

		.venv/bin/python -m coverage report -m
		.venv/bin/python -m coverage html
		$(BROWSER) htmlcov/index.html

test: venv dev ## run tests
	cargo test
	.venv/bin/python -m pytest

test-all: venv ## run tests on multiple Python versions using tox
	.venv/bin/tox

dev: venv ## build the Rust extension, making it accessible to Python
	.venv/bin/python -m maturin develop

venv:  ## set up a virtualenv and install dependencies
	python -m virtualenv .venv || python -m venv .venv
	.venv/bin/python -m pip install -r requirements_dev.txt

docs: venv dev clean-docs ## generate HTML documentation
	.venv/bin/pdoc --no-show-source --no-search --docformat google --output-dir _docs rgeocoder rgeocoder.exceptions 

servedocs: venv dev ## generate HTML docs and watch for changes
	.venv/bin/pdoc --no-show-source --no-search --docformat google rgeocoder rgeocoder.exceptions

dist: clean venv ## build source and wheel packages
	.venv/bin/python -m maturin sdist
	.venv/bin/python -m maturin build
	.venv/bin/twine check target/wheels/*
	ls -l target/wheels
