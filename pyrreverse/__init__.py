# -*- coding: utf-8 -*-
from __future__ import absolute_import

from collections import namedtuple
from ._pyrreverse import reverse_geocode

__all__ = ['reverse_geocode', 'find']

RGeocoderResult = namedtuple('RGeocoderResult', (
    'lat',
    'lon',
    'name',
    'admin1',
    'admin2',
    'admin3',
))

def find(lat, lon):
    """"""
    return RGeocoderResult(*reverse_geocode(lat, lon))
