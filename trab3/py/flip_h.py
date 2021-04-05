import numpy as np
import cv2 as cv
import math

cap = cv.VideoCapture(0)


def on_trackbar(val):
    None


if not cap.isOpened():
    print("Cannot open camera")
    exit()

title = "Trabalho 3 - Flip horizontal"
cv.namedWindow(title)

while True:
    # Capture frame-by-frame
    ret, frame = cap.read()

    # if frame is read correctly ret is True
    if not ret:
        print("Can't receive frame (stream end?). Exiting ...")
        break

    cv.imshow("Original", frame)
    cv.imshow(title, cv.flip(frame, 1))

    if cv.waitKey(1) == ord("q"):
        break


# When everything done, release the capture
cap.release()
cv.destroyAllWindows()