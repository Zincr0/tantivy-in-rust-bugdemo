import ctypes
from ctypes import cdll
import json


def main():
    lib = cdll.LoadLibrary("target/release/libfailgosample.so")

    lib.filtra.argtypes = (ctypes.c_char_p,)
    lib.filtra.restype = ctypes.c_void_p

    lib.freeFiltra.argtypes = (ctypes.c_void_p,)

    post1 = {"body": "hello fake test"}
    post2 = {"body": "bye fake test"}

    fakeSearch = {"search": "fake search", 
                "posts": [post1, post2]}

    try:
        ptr = lib.filtra(json.dumps(fakeSearch).encode("utf-8"))
        respuesta = ctypes.cast(ptr, ctypes.c_char_p).value
    finally:
        lib.freeFiltra(ptr)
    print("finished with no errors.")



if __name__ == "__main__":
    main()
