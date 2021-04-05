import numpy as np
import cv2 as cv
import math
import sys

cap = cv.VideoCapture(0)


def on_trackbar(val):
    None


if not cap.isOpened():
    print("Cannot open camera")
    exit()

title = "Trabalho 3 - Sobel"
cv.namedWindow(title)

cv.createTrackbar("Gaussian", title, 0, 1, on_trackbar)
cv.createTrackbar("Gray", title, 0, 1, on_trackbar)


if len(sys.argv) > 1:
    ret, frame = cap.read()
    height, width, _ = frame.shape
    fourcc = cv.VideoWriter_fourcc(*"MJPG")
    out = cv.VideoWriter(sys.argv[1], fourcc, 20.0, (width, height))


while True:
    # Capture frame-by-frame
    ret, frame = cap.read()

    gauss = cv.getTrackbarPos("Gaussian", title)
    gray = cv.getTrackbarPos("Gray", title)

    img = frame

    if gray == 1:
        img = cv.cvtColor(img, cv.COLOR_BGR2GRAY)

    if gauss > 0 and gauss % 2 == 1:
        img = cv.GaussianBlur(img, (3, 3), 0)

    grad_x = cv.Sobel(img, cv.CV_64F, 1, 0)
    grad_y = cv.Sobel(img, cv.CV_64F, 0, 1)

    abs_grad_x = cv.convertScaleAbs(grad_x)
    abs_grad_y = cv.convertScaleAbs(grad_y)

    sobel = cv.addWeighted(abs_grad_x, 0.5, abs_grad_y, 0.5, 0)

    # if frame is read correctly ret is True
    if not ret:
        print("Can't receive frame (stream end?). Exiting ...")
        break

    cv.imshow("Original", frame)
    cv.imshow(title, sobel)

    if len(sys.argv) > 1:
        out.write(sobel)

    if cv.waitKey(1) == ord("q"):
        break


# When everything done, release the capture
cap.release()
cv.destroyAllWindows()