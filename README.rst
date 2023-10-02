rgeocoder
=========

|badgepypi| |badgedocs| 

.. |badgepypi| image:: https://img.shields.io/pypi/v/rgeocoder.svg
        :target: https://pypi.python.org/pypi/rgeocoder
        :alt: PyPI version

.. |badgedocs| image:: https://img.shields.io/badge/docs-latest-blue
        :target: https://phil-v.github.io/rgeocoder/
        :alt: Documentation

A lightweight offline reverse geocoder implemented in Rust with
`pyo3 <https://github.com/PyO3/pyo3>`_ Python bindings.


Basic usage
-----------

.. code-block:: sh

    pip install rgeocoder

.. code-block:: python

    >>> from rgeocoder import ReverseGeocoder
    >>> rg = ReverseGeocoder()
    >>> r = rg.nearest(41.891929, 12.511331)  # lat, lon
    >>> print(r.name, r.cc)
    'Rome IT'
    >>> print(r.lat, r.lon)
    41.89193 12.51133

Some locations include the first-
and second-level administrative divisions:

.. code-block:: python

    >>> print(r.admin1)
    'Latium'
    >>> print(r.admin2)
    'Citta metropolitana di Roma Capitale'

See `<http://download.geonames.org/export/dump/readme.txt>`_ for more
information on the dataset.


License
-------

`MIT License`_

.. _MIT License: LICENSE


Credits
-------

Rust implementation of the algorithm originally based on code from
`llambda/rust-reverse-geocoder <https://github.com/llambda/rust-reverse-geocoder>`_.

Inspired by and meant to act as an alternative to
`thampiman/reverse-geocoder <https://github.com/thampiman/reverse-geocoder>`_.
