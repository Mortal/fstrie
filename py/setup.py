# setup.py based on https://github.com/getsentry/symbolic
import os
import re
import atexit
import shutil
import zipfile
import tempfile
import subprocess
from setuptools import setup, find_packages
from distutils.command.sdist import sdist


MODULE_NAME = 'fstrie'
_version_re = re.compile(r'^version\s*=\s*"(.*?)"\s*$(?m)')


DEBUG_BUILD = os.environ.get('FSTRIE_DEBUG') == '1'

with open('README', 'r') as f:
    readme = f.read()


if os.path.isfile('../Cargo.toml'):
    with open('../Cargo.toml') as f:
        version = _version_re.search(f.read()).group(1)
else:
    with open('version.txt') as f:
        version = f.readline().strip()


def write_version():
    with open('version.txt', 'wb') as f:
        f.write(('%s\n' % version).encode())


class CustomSDist(sdist):
    def run(self):
        write_version()
        sdist.run(self)


def build_native(spec):
    cmd = ['cargo', 'build']
    if not DEBUG_BUILD:
        cmd.append('--release')
        target = 'release'
    else:
        target = 'debug'

    # Step 0: find rust sources
    rust_path = '..'

    # Step 1: build the rust library
    build = spec.add_external_build(
        cmd=cmd,
        path=rust_path
    )

    spec.add_cffi_module(
        module_path='%s._lowlevel' % MODULE_NAME,
        dylib=lambda: build.find_dylib(MODULE_NAME, in_path='target/%s' % target),
        header_filename=lambda: build.find_header('%s.h' % MODULE_NAME, in_path='include'),
        rtld_flags=['NOW', 'NODELETE']
    )


setup(
    name=MODULE_NAME,
    version=version,
    packages=find_packages(),
    author='Mathias Rav',
    license='GPL3+',
    author_email='m@git.strova.dk',
    description='A Python library for searching filesystem-backed tries.',
    long_description=readme,
    include_package_data=True,
    zip_safe=False,
    platforms='any',
    install_requires=[
        'milksnake',
    ],
    setup_requires=[
        'milksnake',
    ],
    milksnake_tasks=[
        build_native,
    ],
    cmdclass={
        'sdist': CustomSDist,
    }
)

