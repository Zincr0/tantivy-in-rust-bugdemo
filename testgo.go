package main

// #cgo CFLAGS: -g -Wall -I${SRCDIR}
// #cgo LDFLAGS: ${SRCDIR}/target/x86_64-unknown-linux-musl/release/libfailgosample.a
// #include "libfailgosample.h"
import "C"
import (
	"encoding/json"
	"fmt"
	"unsafe"
)

type FakeSearchItem struct {
	Body string `json:"body"`
}

type FakeSearch struct {
	SearchText string           `json:"search"`
	Posts      []FakeSearchItem `json:"items"`
}

func doFakeSearch(FakeSearch FakeSearch) ([]string, error) {

	resultIds := []string{}

	jsonData, err := json.Marshal(FakeSearch)
	if err != nil {
		return resultIds, err
	}

	fakeSearchPointer := C.CString(string(jsonData))

	resultPointer := C.filtra(fakeSearchPointer)

	_ = C.GoString(resultPointer)

	C.free(unsafe.Pointer(fakeSearchPointer))
	C.freeFiltra(resultPointer)

	return resultIds, nil
}

func main() {
	textItems := []FakeSearchItem{}

	post1 := FakeSearchItem{Body: "hello fake test"}
	post2 := FakeSearchItem{Body: "bye fake test"}

	textItems = append(textItems, post1)
	textItems = append(textItems, post2)

	consulta := FakeSearch{SearchText: "fake search text", Posts: textItems}

	_, errf := doFakeSearch(consulta)
	if errf != nil {
		fmt.Printf("error! : %v \n", errf)
	}

	fmt.Println("done")
}
