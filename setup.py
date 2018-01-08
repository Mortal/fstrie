# setup.py based on https://github.com/getsentry/milksnake
from setuptools import setup


def build_native(spec):
    build = spec.add_external_build(
        cmd=['cargo', 'build', '--release'],
        path='./rust',
    )

    spec.add_cffi_module(
        module_path='fstrie._native',
        dylib=lambda: build.find_dylib('fstrie', in_path='target/release'),
        header_filename=lambda: build.find_header('fstrie.h', in_path='include'),
    )


setup(
    name='fstrie',
    version='0.1.0',
    packages=['fstrie'],
    author='Mathias Rav',
    license='GPL3+',
    author_email='m@git.strova.dk',
    description='A Python library for searching filesystem-backed tries.',
    # long_description=readme,
    include_package_data=True,
    zip_safe=False,
    platforms='any',
    install_requires=['milksnake'],
    setup_requires=['milksnake'],
    milksnake_tasks=[
        build_native,
    ],
)

