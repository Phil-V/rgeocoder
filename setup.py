#!/usr/bin/env python
# -*- coding: utf-8 -*-

from setuptools import setup, Distribution
try:
    from setuptools_rust import RustExtension, Binding
except ImportError:
    import subprocess
    print("\nsetuptools_rust is required before install - https://pypi.python.org/pypi/setuptools-rust")
    print("attempting to install with pip...")
    print(subprocess.check_output(["pip", "install", "setuptools_rust"]))
    from setuptools_rust import RustExtension, Binding

with open('README.rst') as readme_file:
    readme = readme_file.read()

with open('HISTORY.rst') as history_file:
    history = history_file.read()

requirements = []

test_requirements = [
    'pytest>=2.9.2',
    'pytest-runner>=2.0'
]

setup(
    name='rgeocoder',
    version='0.1.3',
    description="A lightweight offline reverse geocoder implemented in Rust.",
    long_description=readme + '\n\n' + history,
    author="Phil V.",
    author_email='philippe@arcadian.be',
    url='https://github.com/Phil-V/rgeocoder',
    packages=[
        'rgeocoder',
    ],
    package_dir={'rgeocoder':
                 'rgeocoder'},
    include_package_data=True,
    package_data={'rgeocoder': ['cities.csv']},
    install_requires=requirements,
    license="MIT license",
    zip_safe=False,
    rust_extensions=[
        RustExtension(
            'rgeocoder._rgeocoder',
            'rgeocoder/rust/Cargo.toml',
            debug=False,
            binding=Binding.PyO3
        )],
    keywords='rgeocoder',
    classifiers=[
        'Development Status :: 3 - Alpha',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: MIT License',
        'Natural Language :: English',
        "Programming Language :: Python :: 2",
        'Programming Language :: Python :: 2.7',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.5',
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Rust',
    ],
    test_suite='tests',
    tests_require=test_requirements,
    setup_requires=['setuptools_rust',
    'pytest-runner>=2.0',
    ]
)
