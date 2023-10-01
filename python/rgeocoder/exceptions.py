# -*- coding: utf-8 -*-


class GeocoderException(Exception):
    """Base class for all the `rgeocoder` module exceptions."""


class InitializationError(GeocoderException):
    """Catch-all for `rgeocoder.ReverseGeocoder` initialization errors."""


class CsvReadError(InitializationError):
    """Could not open the locations csv file."""


class CsvParseError(InitializationError):
    """Could not parse the locations csv file."""
