package main

import (
	"fmt"
	"time"

	"github.com/bytedance/sonic"
)

type Employee struct {
	name   string
	age    int
	salary float32
	// title  JobTitle
}

func main_sonic() string {
	x := Employee{}
	s := `{"name":"Peter","age":28,"salary":95000.5,"title":2}`
	y := sonic.Unmarshal([]byte(s), &x)
	_ = y
	return x.name
}

func main() {
	for ii := 0; ii < 10000; ii++ { //preheat
	}
	N := 100000
	now := time.Now()
	for ii := 0; ii < N; ii++ {
		_ = main_sonic()
	}

	d := float64(time.Since(now))
	fmt.Println("total in ms", d/1e6)
	// fmt.Println("avg in us", d/float64(N)/1e3)
}
