from os import path
from setuptools import setup, find_packages
from src.utils import version

install_requires = [
    "requests==2.26.0",
    "pyside2==5.15.2",
    "toml==0.10.2",
    "pillow==8.3.1",
    "pyaoaddons==0.2.7.dev"
]

this_directory = path.abspath(path.dirname(__file__))
with open(path.join(this_directory, 'README.md'), encoding='utf-8') as f:
    long_description = f.read()

    setup(
        name='aostats',
        author="Wiktor Mazur",
        version=version.get_version(),
        author_email="wiktormazur1@gmail.com",
        url="https://github.com/mazurwiktor/albion-online-stats",
        long_description=long_description,
        packages=find_packages('.', include=('src*')),
        package_data={'': ['**/*.png']},
        include_package_data=True,
        install_requires=install_requires,
        entry_points={
            'console_scripts': [
                'aostats = src.__main__:run'
            ]
        })
