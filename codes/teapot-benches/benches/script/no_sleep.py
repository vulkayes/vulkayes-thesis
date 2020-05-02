import time
import pyautogui

while True:
	print("Moving mouse by 10");
	pyautogui.moveRel(0, 10);
	time.sleep(30);
	print("Moving mouse by -10");
	pyautogui.moveRel(0, -10);
	time.sleep(30);

