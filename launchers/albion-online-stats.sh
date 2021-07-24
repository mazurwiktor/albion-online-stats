#!/bin/bash

python -m venv python_env
source python_env/bin/activate
python -m pip install aostats
start aostats