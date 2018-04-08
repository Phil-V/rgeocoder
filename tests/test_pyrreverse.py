# -*- coding: utf-8 -*-
from __future__ import absolute_import

import pytest
import pyrreverse
import pyrreverse.exceptions


@pytest.fixture(scope="module")
def geocoder():
    return pyrreverse.ReverseGeocoder()

@pytest.fixture()
def invalid_csv(tmpdir_factory):
    csv = tmpdir_factory.mktemp('data').join('locations.csv')
    csv.write(
        'foo, bar, foobar\n'
        'a, b, c'
    )
    return csv

@pytest.fixture()
def empty_csv(tmpdir_factory):
    """csv file with no rows"""
    csv = tmpdir_factory.mktemp('data').join('empty.csv')
    csv.write(
        'foo, bar, foobar\n'
    )
    return csv

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

def test_cant_open_file():
    with pytest.raises(pyrreverse.exceptions.CsvReadError):
        pyrreverse.ReverseGeocoder(path='foo.csv', lazy=False)

def test_cant_parse_file(invalid_csv):
    with pytest.raises(pyrreverse.exceptions.CsvParseError):
        pyrreverse.ReverseGeocoder(path=str(invalid_csv), lazy=False)

def test_empty_csv(empty_csv):
    rg = pyrreverse.ReverseGeocoder(path=str(empty_csv), lazy=False)
    assert rg.find(43.25338, 2.17808) is None
