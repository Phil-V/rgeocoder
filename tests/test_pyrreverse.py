# -*- coding: utf-8 -*-
from __future__ import absolute_import

import pytest
import pyrreverse


@pytest.fixture(scope="module")
def geocoder():
    return pyrreverse.ReverseGeocoder()

def test_can_reverse_geocode(geocoder):
    result = geocoder.find(43.25338, 2.17808)
    assert result.name == 'Alzonne'
    result = geocoder.find(43.25338, 2.17808)
    assert result.name == 'Alzonne'
    result = geocoder.find(-6.16394, 39.19793)
    assert result.name == 'Zanzibar'

def test_result(geocoder):
    result = geocoder.find(12.42167,42.89556)
    assert result.name == 'Alaili Dadda`'
    assert result.admin1 == 'Obock'
    assert result.admin2 == ''
    assert result.cc == 'DJ'
    assert result.lat == 12.42167
    assert result.lon == 42.89556
    assert result.__repr__() == '<ReverseGeocoderResult [12.4217, 42.8956]>'

def test_wrong_type(geocoder):
    with pytest.raises(TypeError):
        geocoder.find('foo', 'bar')