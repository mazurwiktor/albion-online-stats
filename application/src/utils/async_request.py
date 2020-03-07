import threading
import queue
import requests
import collections


Item = collections.namedtuple('Item', 'key value')


_queue = queue.Queue()
_requests = {}
_requests_threads = {}


def update_loop():
    while True:
        item = _queue.get()

        _requests[item.key] = item.value
        _requests_threads[item.key].join()
        del _requests_threads[item.key]


_update_thread = threading.Thread(target=update_loop, daemon=True)
_update_thread.start()


def _async_req(url):
    req = requests.get(url)

    _queue.put(Item(key=url, value=req))


def get(url):

    if url in _requests:
        return _requests[url]

    if url in _requests_threads:
        return None

    rt = threading.Thread(target=_async_req, args=(url,))
    _requests_threads[url] = rt
    rt.start()

    return None
