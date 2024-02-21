[lib](https://pypi.org/project/screenton-maker-rs/)
```py
import cv2
import numpy as np
from screenton_maker import Screenton

img = cv2.cvtColor(cv2.imread("1708268835.555137.png"), cv2.COLOR_RGB2GRAY).astype(np.float32) / 255
s = Screenton(7) #Screenton(7,lx_plus=3,ly_plus=3))  automatic values lx_plus = dot_size/2, ly_plus = dot_size/2
img = s.run(img)
cv2.imwrite("2.png", img * 255)
```
