#!/bin/bash
set -ex

python -m pip install -Uq twine urllib3[secure]
ls -la wheelhouse/
python -m twine upload --repository-url https://test.pypi.org/legacy/ wheelhouse/*
