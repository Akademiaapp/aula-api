from datetime import datetime
import dateHelper
import httpx
import json

class aulaHandler:
    def __init__(self, session: httpx.Client):
        self.session = session
        self.profileInfo = self.requestProfileInfo()
        self.id = self.profileInfo["institutionProfiles"][0]["id"]
        self.institutionCode = self.profileInfo["institutionProfiles"][0]["institutionCode"]
        
        self.token = self.session.cookies["Csrfp-Token"]
        
        # print("DisplayName: ", self.profileInfo["displayName"], ", Id: ", self.id, ", token", self.token)
        pass

    def getCalenderEvents(self):
        
        start_date, end_date = dateHelper.get_current_week_dates()
        
        data = {
                "instProfileIds": [self.id],
                "start": start_date,
                "end": end_date,
                }


    
        r = self.session.post("https://www.aula.dk/api/v18/?method=calendar.getEventsByProfileIdsAndResourceIds", json=data, headers={"Csrfp-Token": self.token})
        rData = json.loads(r.text)["data"]
 
        # json_object = json.dumps(rData, indent=4)
        # Writing to sample.json for testing purposes
        # with open("sample.json", "w") as outfile:
        #     outfile.write(json_object)
        return rData
        
    def requestProfileInfo(self):
        r = self.session.get("https://www.aula.dk/api/v18/?method=profiles.getProfilesByLogin")
        info = json.loads(r.text)
        return info["data"]["profiles"][0]
        