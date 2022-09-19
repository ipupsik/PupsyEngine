import subprocess as sp
import os
import time

for sub_dir, dirs, files in os.walk("shaders/src"):
    for file in files:
        filepath = sub_dir + os.sep + file

        if filepath.endswith(".frag") or filepath.endswith(".vert"):
            start_time = time.time()
            sp.run(["glslc", filepath, "-o", "shaders/spv/" + file + ".spv"])
            print("Compiled ", file, " for ", (time.time() - start_time), "seconds")