from typing import Callable, Generator

from contextlib import contextmanager
from pathlib import Path
from time import sleep

from selenium.webdriver import Chrome, ChromeOptions, Keys
from selenium.webdriver.common.by import By
from selenium.webdriver.support.wait import WebDriverWait
from selenium.webdriver.support.select import Select
from pyperclip import copy

@contextmanager
def close_with[T](resource: T, closer: Callable[[T], None]) -> Generator[T, None, None]:
    try:
        yield resource
    finally:
        closer(resource)

DOWNLOADS_PATH = Path(r"C:\Users\Elijah Ang\Desktop\Projects\Coding\animal_crossing_taking_root\songs")

CHROME_OPTIONS = ChromeOptions()
CHROME_OPTIONS.add_argument("--incognito")
CHROME_OPTIONS.add_experimental_option(
    "prefs", 
    {
        "download.default_directory": f"{DOWNLOADS_PATH}",
        "savefile.default_directory": f"{DOWNLOADS_PATH}",
        "download.prompt_for_download": False,       # Disable the download prompt
        "directory_upgrade": True,                   # Allow directory upgrades
        "safebrowsing.enabled": True,                # Enable safe browsing for downloads
        "profile.default_content_settings.popups": 0 # Suppress all popup windows
    }
)

def download_song(hour : int, period: str) -> None:
    with close_with(
        Chrome(options=CHROME_OPTIONS), 
        lambda driver: driver.quit()
    ) as driver:
        driver.get("https://bandchampalbumdownloadermp3.com/")

        song_url = f"https://scruffymusic.bandcamp.com/track/{hour}-{period}"

        download_url_input = driver.find_element(by=By.ID, value="url")
        copy(song_url)
        download_url_input.send_keys(Keys.CONTROL + "v")

        wait_to_load = WebDriverWait(driver, timeout=5)
        wait_to_load.until(lambda driver: driver.current_url == "https://bandchampalbumdownloadermp3.com/single_track.php")

        download_button = driver.find_element(by=By.CSS_SELECTOR, value="a[album='Animal Crossing: Taking Root']")
        download_button.click()

        quality_selector = Select(driver.find_element(by=By.ID, value="quality"))
        quality_selector.select_by_value("ogg")

        song_download_path = DOWNLOADS_PATH / Path(f"{hour} {period.title()} - Scruffy.oga")

        wait_to_download = WebDriverWait(driver, timeout=60)
        wait_to_download.until(lambda _: song_download_path.exists())

        song_download_path.rename(song_download_path.with_stem(f"{hour}_{period}"))

songs = [(hour, period) for period in ("am", "pm") for hour in range(1, 12 + 1)]
songs = [(hour, period) for (hour, period) in songs if not (DOWNLOADS_PATH / Path(f"{hour}_{period}.oga")).exists()]

for (hour, period) in songs:
    download_song(hour, period)