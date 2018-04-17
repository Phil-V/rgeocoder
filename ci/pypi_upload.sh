#!/bin/bash
ls -la wheelhouse/
source /tmp/.venv/3.6.5/bin/activate || virtualenv /tmp/.venv/deploy && source /tmp/.venv/deploy/bin/activate
set -ex
pip install -Uq twine urllib3[secure]
python -m twine upload wheelhouse/*
