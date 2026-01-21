import ctypes
import os
import sys

# Path to the shared library
lib_path = os.path.abspath('build/cde/lib/csa/libcsa.so')

print(f"Attempting to load: {lib_path}")

try:
    # Load the library
    libcsa = ctypes.CDLL(lib_path)
    print("Successfully loaded libcsa.so")
except OSError as e:
    print(f"Failed to load libcsa.so: {e}")
    sys.exit(1)

# Symbols to check
symbols = [
    '_DtCm_rpc_open_calendar',   # The C wrapper we modified
    # 'rs_rpc_open_calendar'     # The Rust function (might be hidden, but good to check if exported)
]

print("\nChecking for symbols:")
for sym_name in symbols:
    try:
        # verify symbol exists
        func = getattr(libcsa, sym_name)
        print(f"[OK] Symbol found: {sym_name}")
        print(f"     Address: {hex(ctypes.cast(func, ctypes.c_void_p).value)}")
    except AttributeError:
        print(f"[FAIL] Symbol NOT found: {sym_name}")
        sys.exit(1)

print("\nVerification Successful")
