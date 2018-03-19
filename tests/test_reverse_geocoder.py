# -*- coding: utf-8 -*-
from __future__ import absolute_import

import pytest
import pyrreverse

def test_can_reverse_geocode():
    result = pyrreverse.reverse_geocode(5, 50.)
    assert result == 'foobar'
