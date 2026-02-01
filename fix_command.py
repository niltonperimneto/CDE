
import os

filepath = "/home/niltonperimneto/cdesktopenv-code/cde/programs/dtfile/Command.c"

with open(filepath, "r") as f:
    lines = f.readlines()

new_lines = []
skip = False
fixed_block_written = False

header_block_start_marker = "#include <X11/CompositeP.h>"
header_block_end_marker = "#include <X11/Xlib.h>"

fixed_headers = [
    "#include <X11/Xlib.h>\n",
    "#include <X11/Intrinsic.h>\n",
    "#include <X11/IntrinsicP.h>\n",
    "#include <X11/CoreP.h>\n",
    "#include <X11/CompositeP.h>\n",
    "#include <X11/Shell.h>\n"
]

for line in lines:
    if line.strip() == header_block_start_marker:
        skip = True
        if not fixed_block_written:
            new_lines.extend(fixed_headers)
            fixed_block_written = True
    
    if not skip:
        new_lines.append(line)
        
    if line.strip() == header_block_end_marker:
        skip = False

with open(filepath, "w") as f:
    f.writelines(new_lines)
