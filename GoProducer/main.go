package main

import (
	"fmt"
	"strconv"
	"time"

	MQTT "github.com/eclipse/paho.mqtt.golang"
)

func main() {
	// a new message arrives every 20 milliseconds
	chTicks := make(chan int, 128)
	// true: start running, false: stop
	chRunning := make(chan bool, 128)
	go func() {
		tick := 0
		for {
			chTicks <- tick
			tick += 1
			time.Sleep(20 * time.Millisecond)
		}
	}()
	server := "tcp://localhost:6788"
	connOpts := MQTT.NewClientOptions().AddBroker(server).SetClientID("go-producer")
	client := MQTT.NewClient(connOpts)
	token := client.Connect()
	token.Wait()
	if token.Error() != nil {
		panic(token.Error())
	}
	fmt.Println("Connected")

	client.Subscribe("control", byte(1), func(client MQTT.Client, message MQTT.Message) {
		msg := string(message.Payload()[:])
		if msg == "start" {
			chRunning <- true
		} else if msg == "stop" {
			chRunning <- false
		}
	})

	currIdx := 0
	running := false
	for {
		select {
		case <-chTicks:
			if running {
				msg := strconv.Itoa(currIdx)
				client.Publish("data", byte(0), false, msg)
				currIdx += 1
			}
		case running = <-chRunning:
			if running {
				currIdx = 0
			}
		}
	}

	for {
		time.Sleep(2000 * time.Millisecond)
	}
}
