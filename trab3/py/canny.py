import numpy as np
import cv2 as cv
import math

cap = cv.VideoCapture(0)


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

    if cv.waitKey(1) == ord("q"):
        break


# When everything done, release the capture
cap.release()
cv.destroyAllWindows()