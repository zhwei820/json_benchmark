import json
import time

struct Employee {
	name   string
	age    int
	salary f32
	// title  JobTitle
}

fn main_json() {
	s := '{"name":"Peter","age":28,"salary":95000.5,"title":2}'
	y := json.decode(Employee, s) or { panic('err') }
	_ = y
}

fn main() {
	for ii := 0; ii < 10000; ii++ {
	}

	t := time.now()
	n:=100000
	for _ in 0 .. n {
		main_json()
	}
	d:=time.since(t)
	mill_sec:=d.milliseconds()
	println("total $mill_sec in ms",)
}


