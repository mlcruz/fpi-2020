import numpy as np
import cv2 as cv
import math

cap = cv.VideoCapture(0)

def on_trackbar(val):
    None

if not cap.isOpened():
    print("Cannot open camera")
    exit()

title = "Trabalho 3 - Resize"
cv.namedWindow(title)

while True:
    # Capture frame-by-frame
    ret, frame = cap.read()

    # if frame is read correctly ret is True
    if not ret:
        print("Can't receive frame (stream end?). Exiting ...")
        break

    resized = cv.resize(frame, None, fx=0.5, fy=.5)

    cv.imshow("Original", frame)
    cv.imshow(title, resized)

    if cv.waitKey(1) == ord('q'):
        break


# When everything done, release the capture
cap.release()
cv.destroyAllWindows()