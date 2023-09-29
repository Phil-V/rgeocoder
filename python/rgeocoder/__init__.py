# -*- coding: utf-8 -*-
"""
Reverse Geocoder module

rgeocoder is a lightweight reverse geocoding module for Python
implemented in Rust.

Basic usage:
   >>> from rgeocoder import ReverseGeocoder
   >>> rg = ReverseGeocoder()
   >>> r = rg.nearest(50.844992, 4.349990)  # lat, lon
   >>> r.name == 'Brussels' and r.cc == 'BE'
   True

The default dataset includes cities with a population greater than 1000.
See http://download.geonames.org/export/dump/readme.txt for more
information on the dataset.
"""

from __future__ import absolute_import, unicode_literals
from os.path import join, dirname
# Guard against panic in the rust library if an absolute import of the
# module is not possible. We want the python interpreter to handle this
# instead.
# As a result, from foo.rgeocoder import ReverseGeocoder will not work
# and throw ImportError.
from rgeocoder.exceptions import InitializationError as _  # noqa: F401
from rgeocoder._rgeocoder import RustReverseGeocoder as _RustReverseGeocoder

__version__ = '0.1.3'
__all__ = ['ReverseGeocoder', 'Location']


class ReverseGeocoder(object):
    """Reverse geocode coordinates into city and country names.

    :param path: (optional) a path to a custom dataset.
    :type path: string
    :param lazy: (optional) wait for the first query to load the dataset
        (default) or load it immediately on init (if set to False)
    :type lazy: bool
    """

    def __init__(self, path=None, lazy=True):
        if path is not None:
            self._path = path
        else:
            self._path = join(dirname(__file__), 'cities.csv')
        if not lazy:
            self._initialize()

    def nearest(self, lat, lon):
        """Find the location closest to the coordinates.

        Returns a :class:`Location` object representing
        a dataset entry.
        """
        result = self._geocoder.find(lat, lon)
        if result:
            return Location(*result)

    def _initialize(self):
        self._rust_geocoder = _RustReverseGeocoder(self._path)

    @property
    def _geocoder(self):
        try:
            return self._rust_geocoder
        except AttributeError:
            self._initialize()
            return self._rust_geocoder


class Location(object):
    """A location found in the dataset."""

    def __init__(self, lat, lon, name, admin1, admin2, cc):
        #: Float of the location's latitude
        self.lat = lat
        #: Float of the location's longitude
        self.lon = lon
        #: The location name
        self.name = name
        #: First-level administrative division
        self.admin1 = admin1
        #: Second-level administrative division
        self.admin2 = admin2
        #: Country code
        self.cc = cc

    def __repr__(self):
        return '<Location [{lat:.4f}, {lon:.4f}]>'.format(
            lat=self.lat,
            lon=self.lon,
        )
