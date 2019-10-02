import os
import requests

_version = "0.5.3"

latest_url = "https://github.com/mazurwiktor/albion-online-stats/releases/latest"


def latest_version():
    req = requests.get(latest_url)
    if req.status_code == 200:
        return os.path.basename(req.url)
    return None


def current_version():
    return _version
