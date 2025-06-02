rgeocoder
=========

|badgepypi| |badgedocs|

.. |badgepypi| image:: https://img.shields.io/pypi/v/rgeocoder.svg
        :target: https://pypi.python.org/pypi/rgeocoder
        :alt: PyPI version

.. |badgedocs| image:: https://img.shields.io/badge/docs-latest-blue
        :target: https://phil-v.github.io/rgeocoder/
        :alt: Documentation

A lightweight offline reverse geocoding module for Python implemented in Rust.


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

Some locations include first- and second-level administrative divisions:

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


Acknowledgments
---------------

Initially forked from `llambda/rust-reverse-geocoder <https://github.com/llambda/rust-reverse-geocoder>`_
for the Rust implementation of the algorithm.

Inspired by and meant to act as a dependency-free alternative to NumPy/SciPy-based
`thampiman/reverse-geocoder <https://github.com/thampiman/reverse-geocoder>`_.
