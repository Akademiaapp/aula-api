
import urllib.parse as urlparse
from unilogin import login    
from aulaHandler import aulaHandler
import json
# with open("user.json") as f:
#     user = json.load(f)
# login = Unilogin(user["username"], user["password"])

# print(login.login("https://www.aula.dk/auth/login.php?type=unilogin"))

with open("user.json") as f:
     user = json.load(f)
     
session = login(user["username"], user["password"])
print(session.cookies)
handler = aulaHandler(session=session)

print(handler.getCalenderEvents())

# r = session.get("https://www.aula.dk/api/v18/?method=municipalConfiguration.getSameAdministrativeAuthorityInstitutions&institutionCode=373012")
# print(r.text)

