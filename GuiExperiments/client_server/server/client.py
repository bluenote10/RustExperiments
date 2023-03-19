import json
import pprint

import requests

routes = [
    "http://127.0.0.1:3000/",
    "http://127.0.0.1:3000/plain_string",
    "http://127.0.0.1:3000/binary_route",
]

for route in routes:
    print(f"\n{route}")
    response = requests.get(route)
    print(f" => {response.status_code} {response.reason}")
    print("headers:", json.dumps(dict(response.headers), indent=True))
    print("details:")
    pprint.pprint(response.__dict__)
    print("text:", response.text)

