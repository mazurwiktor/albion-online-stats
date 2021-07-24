from setuptools import setup, find_packages
from src.utils import version

install_requires = [
    "requests==2.26.0",
    "pyside2==5.15.2",
    "toml==0.10.2",
    "pillow==8.3.1",
    "pyaoaddons==0.2.7.dev"
]

setup(
    name='aostats',
    version=version.get_version(),
    packages=find_packages('.', include=('src*')),
    package_data={'': ['**/*.png']},
    include_package_data=True,
    install_requires=install_requires,
    entry_points={
        'console_scripts': [
            'aostats = src.__main__:run'
        ]
    })
