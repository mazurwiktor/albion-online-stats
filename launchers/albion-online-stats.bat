@echo off

TITLE Albion online stats launcher

where /Q python3 && set FOUND_PYTHON=python3 && goto install
where /Q py && set FOUND_PYTHON=py && goto install
where /Q python && set FOUND_PYTHON=python && goto install

if not defined FOUND_PYTHON (goto python_not_found)

:install
    echo Found python: %FOUND_PYTHON%
    echo Installing virtual environment in current directory...
    %FOUND_PYTHON% -m venv python_env
    call .\python_env\Scripts\activate.bat

    for /f "tokens=* delims=" %%a in ('python --version') do (echo Python version: %%a)
    for /f "tokens=* delims=" %%a in ('python -m pip --version') do (echo Pip version: %%a)

    echo Installing newest version of aostats...
    python -m pip install --upgrade pip
    python -m pip install aostats
    python -m pip install --upgrade aostats

    goto execute

:execute
    start pythonw .\python_env\Scripts\aostats-script.py
    goto end

:python_not_found
    echo Error: python not found in the system
    python
    pause

:end
