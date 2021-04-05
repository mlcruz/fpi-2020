import numpy as np
import cv2 as cv
import math

cap = cv.VideoCapture(0)

def on_trackbar(val):
    None

if not cap.isOpened():
    print("Cannot open camera")
    exit()

title = "Trabalho 3 - Sobel"
cv.namedWindow(title)

cv.createTrackbar("Dx", title , 0, 4, on_trackbar)
cv.createTrackbar("Dy", title , 0, 4, on_trackbar)

cv.setTrackbarPos("Dx", title, 1)

while True:
    # Capture frame-by-frame
    ret, frame = cap.read()

    dx = cv.getTrackbarPos("Dx", title)
    dy = cv.getTrackbarPos("Dy", title)

    # if frame is read correctly ret is True
    if not ret:
        print("Can't receive frame (stream end?). Exiting ...")
        break
    
    canny = cv.Sobel(frame, cv.CV_64F, dx, dy, ksize=5)

    cv.imshow("Original", frame)
    cv.imshow(title, canny)

    if cv.waitKey(1) == ord('q'):
        break


# When everything done, release the capture
cap.release()
cv.destroyAllWindows()