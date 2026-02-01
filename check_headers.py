
import os
import glob

directory = "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile"
files = glob.glob(os.path.join(directory, "*.c"))

bad_files = []

for filepath in files:
    with open(filepath, "r") as f:
        content = f.read()
        
    p_composite = content.find("#include <X11/CompositeP.h>")
    p_core = content.find("#include <X11/CoreP.h>")
    p_intrinsic = content.find("#include <X11/Intrinsic.h>")
    p_xlib = content.find("#include <X11/Xlib.h>")
    
    # If CompositeP or CoreP is present...
    if p_composite != -1 or p_core != -1:
        # Determine the earliest "private" header
        p_private = p_composite if p_composite != -1 else p_core
        if p_composite != -1 and p_core != -1:
            p_private = min(p_composite, p_core)
            
        # Check if Intrinsic.h is present and after private
        if p_intrinsic != -1 and p_intrinsic > p_private:
            bad_files.append(filepath)
            continue
            
        # Check if Xlib.h is present and after private
        if p_xlib != -1 and p_xlib > p_private:
            bad_files.append(filepath)
            continue

print("Files with incorrect header order:")
for f in bad_files:
    print(f)
