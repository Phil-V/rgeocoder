# -*- coding: utf-8 -*-

class GeocoderException(Exception):
    """Base class for all the reverse geocoder module exceptions."""

class InitializationError(GeocoderException):
    """Catching this error will catch all initialization-related errors."""
