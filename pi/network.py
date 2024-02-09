

from requests import post, get
import time
import ifcfg
import os
from led import LoadingData
import logging
from utils import async_wrap


IFACE = "wlan0"  # hard coded for debian raspberry pi

VOUCHER = ""

def login_to_captive_portal():
    payload = {
        "by": "voucher",
        "voucher": VOUCHER
    }

    logging.debug("Hitting captive portal")
    response = post(
        url=f"https://inform.infra.darkwolf.io:8443/guest/s/default/login?t={int(time.time() * 1000)}",
        data=payload
    )
    logging.debug(f"Captive portal response: {response.status_code} {response.content}")


def is_internet_connected():
    try:
        response = get("https://pypi.org", timeout=5)
        return response.status_code == 200
    except Exception:
        return False


def get_connected_network_interface():
    interfaces = ifcfg.interfaces()
    if IFACE in interfaces and interfaces[IFACE]["inet"] is not None:
        return interfaces[IFACE]
    for interface in interfaces:
        if interfaces[interface]["inet"] is not None and interfaces[interface]["inet"] != "127.0.0.1":
            return interfaces[interface] # best guess
    return None


@async_wrap
def try_connect(loading_data: LoadingData):
    connected_network_interface = get_connected_network_interface()
    if connected_network_interface is None:
        return None
    loading_data.line1 = 'Connected'
    loading_data.line2 = connected_network_interface['inet']
    loading_data.line3 = 'Awaiting internet'
    logging.debug(f"Connected as {connected_network_interface['inet']}, awaiting internet")
    time.sleep(1)
    ssid = os.popen("iwgetid -r").read()
    if not is_internet_connected():
        login_to_captive_portal()
    if not is_internet_connected():
        return None
    logging.debug(f"Found internet at SSID {ssid}")
    loading_data.line3 = f'SSID: {ssid}'
    return ssid

