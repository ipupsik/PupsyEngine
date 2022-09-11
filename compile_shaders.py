import subprocess as sp
import os

for sub_dir, dirs, files in os.walk("shaders/src"):
    for file in files:
        filepath = sub_dir + os.sep + file

        if filepath.endswith(".frag") or filepath.endswith(".vert"):
            print("Compiling", file)
            sp.run(["glslc", filepath, "-o", "shaders/spv/" + file + ".spv"])