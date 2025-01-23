import requests
import time

# Ziel-URL der Website
url = "https://example.com"

# Anzahl der Requests, die gesendet werden sollen
request_count = 3000 # Ändere dies nach Bedarf

# Wartezeit zwischen den Requests (in Sekunden)
delay = 2

for i in range(request_count):
    try:
        response = requests.get(url)
        print(f"Request {i+1}: Status Code {response.status_code}")
    except requests.exceptions.RequestException as e:
        print(f"Fehler bei Request {i+1}: {e}")
    time.sleep(delay)

print("Fertig!")
