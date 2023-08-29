#!/usr/bin/env python

from setuptools import setup

from pathlib import Path
this_directory = Path(__file__).parent
long_description = (this_directory / "README.md").read_text()

setup(
    name='cashu-sdk',
    version='0.0.1',
    description="High level client library.",
    long_description=long_description,
    long_description_content_type='text/markdown',
    include_package_data=True,
    zip_safe=False,
    packages=['cashu_sdk'],
    package_dir={'cashu_sdk': './src/cashu-sdk'},
    url="https://github.com/thesimplekid/cashu-crab",
    author="thesimplekid <tsk@thesimplekid.com>",
    license="BSD-3-Clause",
    # This is required to ensure the library name includes the python version, abi, and platform tags
    # See issue #350 for more information
    has_ext_modules=lambda: True,
)

