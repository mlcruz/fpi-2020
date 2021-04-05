import numpy as np
import cv2 as cv
import math

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

title = "Trabalho 3 - Brilho e contraste"
cv.namedWindow(title)

cv.createTrackbar("Brilho", title , 0, 256, on_trackbar)
cv.createTrackbar("Contraste", title , 0, 100, on_trackbar)
cv.createTrackbar("Negativo", title , 0, 1, on_trackbar)

while True:
    # Capture frame-by-frame
    ret, frame = cap.read()

    brilho = cv.getTrackbarPos("Brilho", title)
    contraste = cv.getTrackbarPos("Contraste", title)
    negativo = cv.getTrackbarPos("Negativo", title)

    # if frame is read correctly ret is True
    if not ret:
        print("Can't receive frame (stream end?). Exiting ...")
        break
    
    scaled = cv.convertScaleAbs(frame, alpha=(1+ (contraste / 10.0)), beta=brilho)

    if(negativo == 1):
        scaled = cv.bitwise_not(scaled)

    cv.imshow("Original", frame)
    cv.imshow(title, scaled)


    if len(sys.argv) > 1:
        out.write(scaled)


    if cv.waitKey(1) == ord('q'):
        break


# When everything done, release the capture
cap.release()
cv.destroyAllWindows()