# -*- coding: utf-8 -*-
from __future__ import absolute_import
import sys
from ._pyrreverse import RustReverseGeocoder
from ._errors import _CsvReadError

__all__ = ['ReverseGeocoder', 'InitializationError']


class GeocoderException(Exception):
    """Base class for all the reverse geocoder module exceptions."""

class InitializationError(GeocoderException):
    """Catching this error will catch all initialization-related errors."""


class ReverseGeocoder(object):
    """"""

    def __init__(self, lazy=True):
        if not lazy:
            self._initialize()

    def _initialize(self):
        self._rust_geocoder = RustReverseGeocoder('foo')
        # except OSError:
        #     raise InitializationError('Could not open the locations file.')
        # except
        #     raise InitializationError('Could not parse the CSV file.')
        # if e.__class__ == '_pyrreverse._InitializationError':
        #     raise InitializationError('Could not initialize the kdtree.')
        # raise

    def find(self, lat, lon):
        """Find the location closest to the coordinates.

        Returns a :class:`ReverseGeocoderResult` object representing a dataset entry.
        """
        result = self._geocoder.find(lat, lon)
        if result:
            return ReverseGeocoderResult(*result)
        raise NotImplementedError()

    @property
    def _geocoder(self):
        try:
            return self._rust_geocoder
        except AttributeError:
            self._initialize()
            return self._rust_geocoder


class ReverseGeocoderResult(object):
    """Reverse geocoder dataset entry.

    See http://download.geonames.org/export/dump/readme.txt for more
    information on the dataset.
    """

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
