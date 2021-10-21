#!/bin/bash


FOUND_PYTHON="none"

for app in python3 python
do
    if [ -x "$(command -v $app)" ]; then
        FOUND_PYTHON=$app
        break
    fi

done

echo "Found python: $FOUND_PYTHON"

echo "Creating python virtual environment"
python -m venv python_env

source python_env/bin/activate

echo "Installing albion online stats"
python -m pip install aostats 
python -m pip install --upgrade aostats

aostats