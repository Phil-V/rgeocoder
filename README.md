RGeocoder: a reverse geocoding module for Python
================================================

[![PyPI version](https://img.shields.io/pypi/v/rgeocoder.svg)](https://pypi.python.org/pypi/rgeocoder)
[![Build Status](https://img.shields.io/travis/Phil-V/rgeocoder.svg)](https://travis-ci.org/Phil-V/rgeocoder)
[![Updates](https://pyup.io/repos/github/Phil-V/rgeocoder/shield.svg)](https://pyup.io/repos/github/Phil-V/rgeocoder/)

A lightweight offline reverse geocoder implemented in Rust with
[pyo3](https://github.com/PyO3/pyo3) python bindings.

Based on [llambda/rust-reverse-geocoder](https://github.com/llambda/rust-reverse-geocoder) and
[thampiman/reverse-geocoder](https://github.com/thampiman/reverse-geocoder).

**Note:** *Still in an early, pre-release state.*

Basic usage
-----------

```sh
pip install rgeocoder
```

```py
>>> from rgeocoder import ReverseGeocoder
>>> rg = ReverseGeocoder()
>>> r = rg.nearest(41.891929,12.511331)  # lat, lon
>>> print(r.name, r.cc)
'Rome IT'
>>> print(r.lat, r.lon)
41.89193 12.51133
```

Some locations include the first-
and second-level administrative divisions:
```py
>>> print(r.admin1)
'Latium'
>>> print(r.admin2)
'Citta metropolitana di Roma Capitale'
```

See http://download.geonames.org/export/dump/readme.txt for more
information on the dataset.


License
-------

[MIT License](LICENSE)


Credits
-------

This package was created with
[audreyr/cookiecutter](https://github.com/audreyr/cookiecutter)
 and the [mckaymatt/cookiecutter-pypackage-rust-cross-platform-publish](https://github.com/mckaymatt/cookiecutter-pypackage-rust-cross-platform-publish) project template.
