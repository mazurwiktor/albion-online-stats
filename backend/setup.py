#!/usr/bin/env python
import sys


from setuptools import setup

try:
    from setuptools_rust import RustExtension, Binding
except ImportError:
    import subprocess

    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension, Binding

setup_requires = ["setuptools-rust>=0.10.1", "wheel"]
install_requires = []

setup(
    name="aostats",
    packages=["aostats"],
    rust_extensions=[RustExtension("libaostats", 'Cargo.toml', binding=Binding.RustCPython)],
    install_requires=install_requires,
    setup_requires=setup_requires,
    include_package_data=True,
    version="6.6.6",
    zip_safe=False,
)