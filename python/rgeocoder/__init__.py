# -*- coding: utf-8 -*-
"""
*rgeocoder* is a lightweight reverse geocoding module for Python
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
import typing
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
    """Initialize the geocoder and load the dataset into memory.

    Args:
        path (str, optional): path to a custom dataset
        lazy (bool): lazy load until the first query (True, default)
            or load the dataset immediately on initialization (False)

    Raises:
        `rgeocoder.exceptions.InitializationError`:
            catch-all for initialization-related errors.
        `rgeocoder.exceptions.CsvReadError`:
            if the csv file cannot be read.
        `rgeocoder.exceptions.CsvParseError`:
            if the contents of the csv file cannot be parsed.
    """

    def __init__(self, path: typing.Optional[str] = None, lazy: bool = True):
        if path is not None:
            self._path = path
        else:
            self._path = join(dirname(__file__), 'cities.csv')
        if not lazy:
            self._initialize()

    def nearest(self, lat: float, lon: float) -> typing.Optional['Location']:
        """Find the location closest to the provided coordinates.

        Args:
            lat (float): latitude
            lon (float): longitude

        Returns:
            :class:`Location` object representing a dataset entry.
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
    """A location found in the dataset.

    Attributes:
        lat (float): latitude
        lon (float): longitude
        name (str): name
        admin1 (str): First-level administrative division
        admin2 (str): Second-level administrative division
        cc (str): ISO-3166 2-letter country code

    Note:
        `admin1` and `admin2` are not included for every location
        in the dataset and will sometimes return an empty string.
    """

    def __init__(self, lat, lon, name, admin1, admin2, cc):
        self.lat: float = lat
        self.lon: float = lon
        self.name: str = name
        self.admin1: str = admin1
        self.admin2: str = admin2
        self.cc: str = cc

    def __repr__(self):
        return '<Location [{lat:.4f}, {lon:.4f}]>'.format(
            lat=self.lat,
            lon=self.lon,
        )
