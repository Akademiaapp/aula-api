import httpx
import json
from bs4 import BeautifulSoup


def postForm(prevR, data, session):
    soup = BeautifulSoup(prevR.text, 'html.parser')
    action = soup.find("form").get("action")

    r = session.post(action, data=data, follow_redirects=True)
    return r

def unilogin(username, password):
    session = httpx.Client()
    session.headers['user-agent'] = 'Mozilla/5.0'
    resp = session.get("https://www.aula.dk/auth/login.php?type=unilogin", follow_redirects=False)

    resp = session.get(resp.headers["location"], follow_redirects=False)
    
    href = resp.headers["location"]
    r = session.get(href, follow_redirects=False)

    r = postForm(r, {"selectedIdp": "uni_idp"}, session)

    r = postForm(r, {"username": username}, session)
    r = postForm(r, {"password": password, "username": ""}, session)
    html = BeautifulSoup(r.text, 'html.parser')

    payload = {html.find("input", {"name": "SAMLResponse"}).get("name"): html.find("input", {"name": "SAMLResponse"}).get("value"),
                    html.find("input", {"name": "RelayState"}).get("name"): html.find("input", {"name": "RelayState"}).get("value")}
    # https://broker.unilogin.dk/auth/realms/broker/broker/uni_idp/endpoint

    r = session.post("https://broker.unilogin.dk/auth/realms/broker/broker/uni_idp/endpoint", data=payload, follow_redirects=True)

    r = postForm(r, {}, session)

    soup = BeautifulSoup(r.text, 'html.parser')
    action = soup.find("form", {"name": "saml-post-binding"}).get("action")
    
    html = BeautifulSoup(r.text, 'html.parser')
    payload = {html.find("input", {"name": "SAMLResponse"}).get("name"): html.find("input", {"name": "SAMLResponse"}).get("value"),
                    html.find("input", {"name": "RelayState"}).get("name"): html.find("input", {"name": "RelayState"}).get("value")}
    # https://broker.unilogin.dk/auth/realms/broker/broker/uni_idp/endpoint

    r = session.post(action, data=payload, follow_redirects=True)
    
    return session