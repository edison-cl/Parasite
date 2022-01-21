import requests
import time
while 1:
    print(requests.get("http://127.0.0.1:1000/api/leader/beat").status_code)
    time.sleep(0.15)