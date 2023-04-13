package main

import (
	"fmt"
	"reflect"
	"sync/atomic"
	"time"
)

var count int32
var total int32

type event struct {
	args    []interface{}
	event   chan struct{}
	handler interface{}
}

func onEvent(sig chan struct{}, t time.Time) {
	d := time.Since(t)
	atomic.AddInt32(&count, 1)
	atomic.AddInt32(&total, int32(d))

	sig <- struct{}{}
}

func main() {

	queue := make(chan event, 200)
	defer close(queue)

	go func(q chan event) {
		// 循环读取事件
		for ev := range q {
			v := reflect.ValueOf(ev.handler)
			if v.Kind() != reflect.Func {
				panic("not a function")
			}

			vArgs := make([]reflect.Value, len(ev.args))
			for i, arg := range ev.args {
				vArgs[i] = reflect.ValueOf(arg)
			}
			v.Call(vArgs)
		}
	}(queue)

	const COUNT = 100000

	var sig = make(chan struct{}, COUNT)

	for i := 0; i < COUNT; i++ {
		// 发事件
		func(args ...interface{}) {
			var ev = event{args: args, handler: onEvent}
			queue <- ev
		}(sig, time.Now())

		time.Sleep(time.Microsecond)
	}

	for i := 0; i < COUNT; i++ {
		<-sig
	}

	fmt.Println("total:", total, "average:", time.Duration(total)/time.Duration(count), count)
}