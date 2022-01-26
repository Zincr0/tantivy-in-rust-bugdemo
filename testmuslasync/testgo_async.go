package main

/*
#cgo CFLAGS: -g -Wall -I${SRCDIR}
#cgo LDFLAGS: ${SRCDIR}/../target/x86_64-unknown-linux-musl/release/libfailgosample.a
#include "../libfailgosample.h"
*/
import "C"
import (
	"fmt"
)

func main() {
	C.justAnAsyncTest()
	fmt.Println("done without errors.")
}
