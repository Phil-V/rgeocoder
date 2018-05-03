#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
test_rgeocoder
----------------------------------

Tests for `rgeocoder` module.
"""

from __future__ import absolute_import, unicode_literals
import pytest
import rgeocoder
import rgeocoder.exceptions


@pytest.fixture(scope="module")
def geocoder():
    return rgeocoder.ReverseGeocoder()


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
    result = geocoder.nearest(43.25338, 2.17808)
    assert result.name == 'Alzonne'
    result = geocoder.nearest(43.25338, 2.17808)
    assert result.name == 'Alzonne'
    result = geocoder.nearest(-6.16394, 39.19793)
    assert result.name == 'Zanzibar'


def test_result(geocoder):
    result = geocoder.nearest(12.42167, 42.89556)
    assert result.name == 'Alaili Dadda`'
    assert result.admin1 == 'Obock'
    assert result.admin2 == ''
    assert result.cc == 'DJ'
    assert result.lat == 12.42167
    assert result.lon == 42.89556
    assert result.__repr__() == '<Location [12.4217, 42.8956]>'


def test_wrong_type(geocoder):
    with pytest.raises(TypeError):
        geocoder.nearest('foo', 'bar')


def test_cant_open_file():
    with pytest.raises(rgeocoder.exceptions.CsvReadError):
        rgeocoder.ReverseGeocoder(path='foo.csv', lazy=False)


def test_cant_parse_file(invalid_csv):
    with pytest.raises(rgeocoder.exceptions.CsvParseError):
        rgeocoder.ReverseGeocoder(path=str(invalid_csv), lazy=False)


def test_empty_csv(empty_csv):
    rg = rgeocoder.ReverseGeocoder(path=str(empty_csv))
    assert rg.nearest(43.25338, 2.17808) is None
