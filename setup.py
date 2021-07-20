from setuptools import setup, find_packages


install_requires = [
    "requests==2.23.0",
    "pyside2==5.14.2",
    "toml==0.10.1",
    "pillow==7.2.0",
    "pyaoaddons==0.2.5"
]

setup(
    name='aostats',
    version='0.11.11',
    packages=find_packages('.', include=('src*')),
    install_requires=install_requires,
    entry_points={
        'console_scripts': [
            'aostats = src.__main__:run'
        ]
    })
