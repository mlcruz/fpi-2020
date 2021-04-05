import numpy as np
import cv2 as cv
import math
import sys


cap = cv.VideoCapture(0)

if len(sys.argv) > 1:
    ret, frame = cap.read()
    height, width, _ = frame.shape
    fourcc = cv.VideoWriter_fourcc(*"MJPG")
    out = cv.VideoWriter(sys.argv[1], fourcc, 20.0, (width, height))

def on_trackbar(val):
    None


if not cap.isOpened():
    print("Cannot open camera")
    exit()

title = "Trabalho 3 - Canny"
cv.namedWindow(title)

cv.createTrackbar("Min Val", title, 100, 500, on_trackbar)
cv.createTrackbar("Max Val", title, 100, 500, on_trackbar)
cv.createTrackbar("Gaussian", title, 0, 7, on_trackbar)


while True:
    # Capture frame-by-frame
    ret, frame = cap.read()

    min_val = cv.getTrackbarPos("Min Val", title)
    max_val = cv.getTrackbarPos("Max Val", title)

    gauss = cv.getTrackbarPos("Gaussian", title)

    if(gauss % 2 == 0 and gauss > 0):
        cv.setTrackbarPos("Gaussian", title, gauss + 1)

    # if frame is read correctly ret is True
    if not ret:
        print("Can't receive frame (stream end?). Exiting ...")
        break

    if(gauss > 0 and gauss % 2 == 1):
        canny = cv.Canny(cv.GaussianBlur(frame, (gauss, gauss), 0), min_val, max_val)
    else:
        canny = cv.Canny(frame, min_val, max_val)


    cv.imshow("Original", frame)
    cv.imshow(title, canny)

    if len(sys.argv) > 1:
        out.write(canny)


    if cv.waitKey(1) == ord("q"):
        break


# When everything done, release the capture
cap.release()
out.release()
cv.destroyAllWindows()