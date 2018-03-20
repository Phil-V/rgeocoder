# -*- coding: utf-8 -*-
from __future__ import absolute_import

import pytest
import pyrreverse

def test_can_reverse_geocode():
    result = pyrreverse.ReverseGeocoder().find(43.25338, 2.17808)
    assert 'Alzonne' in result
