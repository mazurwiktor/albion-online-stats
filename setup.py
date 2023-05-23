from os import path
from setuptools import setup, find_packages
from src.version import version

install_requires = [
    "requests==2.31.0",
    "PySide6==6.2.0",
    "toml==0.10.2",
    "pillow==8.4.0",
    "pyaoaddons==0.2.8"
]

this_directory = path.abspath(path.dirname(__file__))
with open(path.join(this_directory, 'README.md'), encoding='utf-8') as f:
    long_description = f.read()

    setup(
        name='aostats',
        author="Wiktor Mazur",
        version=version,
        author_email="wiktormazur1@gmail.com",
        url="https://github.com/mazurwiktor/albion-online-stats",
        long_description=long_description,
        long_description_content_type='text/markdown',
        packages=find_packages('.', include=('src*')),
        include_package_data=True,
        install_requires=install_requires,
        entry_points={
            'console_scripts': [
                'aostats = src.__main__:run'
            ]
        })
