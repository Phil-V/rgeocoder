# -*- coding: utf-8 -*-
from __future__ import absolute_import

import pytest
import pyrreverse

def test_can_reverse_geocode():
    geocoder = pyrreverse.ReverseGeocoder()
    result = geocoder.find(43.25338, 2.17808)
    assert 'Alzonne' in result
    result = geocoder.find(43.25338, 2.17808)
    assert 'Alzonne' in result
    result = geocoder.find(-6.16394, 39.19793)
    assert 'Zanzibar' in result
