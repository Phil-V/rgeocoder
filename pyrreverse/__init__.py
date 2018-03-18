# -*- coding: utf-8 -*-
from __future__ import absolute_import

from ._pyrreverse import reverse_geocode

__all__ = ['reverse_geocode', 'reverse_geocode_py']


def reverse_geocode_py(lon, lat):
    return reverse_geocode(lon, lat)
