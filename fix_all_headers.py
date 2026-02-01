
import os
import glob
import re

directory = "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile"
files = [
    "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile/MultiView.c",
    "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile/Filter.c",
    "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile/Desktop.c",
    "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile/FileOp.c",
    "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile/FileMgr.c",
    "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile/Find.c",
    "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile/ModAttr.c",
    "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile/Menu.c",
    "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile/Help.c",
    "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile/Prefs.c",
    "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile/ChangeDir.c",
    "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile/File.c",
    "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile/SharedProcs.c",
    "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile/Main.c"
]

def get_priority(line):
    if "Xlib.h" in line: return 1
    if "Intrinsic.h" in line: return 2
    if "IntrinsicP.h" in line: return 3
    if "CoreP.h" in line: return 4
    if "CompositeP.h" in line: return 5
    if "Shell.h" in line: return 6
    if "ShellP.h" in line: return 7
    return 100 # Others

for filepath in files:
    print(f"Processing {filepath}...")
    with open(filepath, "r") as f:
        lines = f.readlines()
        
    x11_lines = []
    first_index = -1
    indices_to_remove = []
    
    for i, line in enumerate(lines):
        if line.strip().startswith("#include <X11/"):
            if first_index == -1:
                first_index = i
            x11_lines.append(line)
            indices_to_remove.append(i)
            
    if not x11_lines:
        continue
        
    # Sort lines
    x11_lines.sort(key=get_priority)
    
    # Construct new content
    new_content = []
    for i, line in enumerate(lines):
        if i == first_index:
            new_content.extend(x11_lines)
            
        if i not in indices_to_remove:
            new_content.append(line)
            
    with open(filepath, "w") as f:
        f.writelines(new_content)
        
    print(f"Fixed {filepath}")
