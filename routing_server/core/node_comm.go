package main

import (
	"fmt"
	"net"
)

func sendToNode(address string, payload []byte) {
	conn, err := net.Dial("udp", address)
	if err != nil {
		fmt.Println("Error connecting:", err)
		return
	}
	defer conn.Close()

	conn.Write(payload)
}