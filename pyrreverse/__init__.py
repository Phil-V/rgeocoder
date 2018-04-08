# -*- coding: utf-8 -*-
"""
Reverse Geocoder module

pyrreverse is a lightweight reverse geocoding module for Python
implemented in Rust.

Basic usage:
   >>> from pyrreverse import ReverseGeocoder()
   >>> rg = ReverseGeocoder()
   >>> r = rg.find(50.844992, 4.349990)  # lat, lon
   >>> r.name == 'Brussels' and r.cc == 'BE'
   True

See http://download.geonames.org/export/dump/readme.txt for more
information on the dataset.
"""
from __future__ import absolute_import
from os.path import join, dirname
# Guard against panic in the rust library if an absolute import of the
# module is not possible. We want the python interpreter to handle this
# instead.
# As a result, from foo.pyrreverse import ReverseGeocoder will not work
# and throw ImportError.
from pyrreverse.exceptions import InitializationError as _
from pyrreverse._pyrreverse import RustReverseGeocoder as _RustReverseGeocoder


__all__ = ['ReverseGeocoder', 'ReverseGeocoderResult']


class ReverseGeocoder(object):
    """"""

    def __init__(self, path=None, lazy=True):
        if path is not None:
            self._path = path
        else:
            self._path = join(dirname(__file__), 'cities.csv')
        if not lazy:
            self._initialize()

    def find(self, lat, lon):
        """Find the location closest to the coordinates.

        Returns a :class:`ReverseGeocoderResult` object representing a dataset entry.
        """
        result = self._geocoder.find(lat, lon)
        if result:
            return ReverseGeocoderResult(*result)

    def _initialize(self):
        self._rust_geocoder = _RustReverseGeocoder(self._path)

    @property
    def _geocoder(self):
        try:
            return self._rust_geocoder
        except AttributeError:
            self._initialize()
            return self._rust_geocoder


class ReverseGeocoderResult(object):
    """Reverse geocoder dataset entry."""

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
        return '<ReverseGeocoderResult [{lat:.4f}, {lon:.4f}]>'.format(
            lat=self.lat,
            lon=self.lon,
        )
