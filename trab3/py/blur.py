import numpy as np
import cv2 as cv
import math

cap = cv.VideoCapture(0)

def on_trackbar(val):
    None

if not cap.isOpened():
    print("Cannot open camera")
    exit()

title = "Trabalho 3 - GaussianBlur"
cv.namedWindow(title)
cv.createTrackbar("Kernel size", title , 1, 21, on_trackbar)


while True:
    # Capture frame-by-frame
    ret, frame = cap.read()


    k_size = cv.getTrackbarPos("Kernel size", title)

    fixed_size = k_size

    if(fixed_size % 2 == 0):
        fixed_size = fixed_size + 1
        cv.setTrackbarPos("Kernel size", title, fixed_size)

    print(fixed_size)


    # if frame is read correctly ret is True
    if not ret:
        print("Can't receive frame (stream end?). Exiting ...")
        break
    
    blur = cv.GaussianBlur(frame,(fixed_size,fixed_size),0)

    cv.imshow(title, blur)

    if cv.waitKey(1) == ord('q'):
        break


# When everything done, release the capture
cap.release()
cv.destroyAllWindows()