# -*- coding: utf-8 -*-


class GeocoderException(Exception):
    """Base class for all the reverse geocoder module exceptions."""


class InitializationError(GeocoderException):
    """Catching this error will catch all initialization-related errors."""


class CsvReadError(InitializationError):
    """Could not open the locations csv file."""


class CsvParseError(InitializationError):
    """Could not parse the locations csv file."""
