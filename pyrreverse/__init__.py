# -*- coding: utf-8 -*-
from __future__ import absolute_import

from ._pyrreverse import reverse_geocode

__all__ = ['reverse_geocode', 'ReverseGeocoder']


class ReverseGeocoder(object):
    """"""

    def __init__(self, lazy=True):
        """"""
        if not lazy:
            # load the kdtree into mem
            pass

    def find(self, lat, lon):
        """"""
        return reverse_geocode(lat, lon)
