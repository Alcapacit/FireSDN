package main

import (
        "fmt"
        "log"
        "net"
        "time"
)

func main() {
        log.Println("[Routing Server] Starting server on port 9000")
        listener, err := net.Listen("tcp", ":9000")
        if err != nil {
                log.Fatal(err)
        }
        defer listener.Close()

        log.Println("[Routing Server] Listening for device connections")

        for {
                conn, err := listener.Accept()
                if err != nil {
                        log.Println("Connection error:", err)
                        continue
                }

                go handleConnection(conn)
        }
}

func handleConnection(conn net.Conn) {
        defer conn.Close()
        buf := make([]byte, 4096)
        n, err := conn.Read(buf)
        if err != nil {
                log.Println("Read error:", err)
                return
        }

        request := string(buf[:n])
        log.Printf("[Routing Server] Received metrics: %s\n", request)

        // Logic to determine best route based on metrics
        // In a real scenario, this would check available VPN nodes
        bestNode := "us-east-1" 
        response := fmt.Sprintf(`{"route":"%s", "status":"optimized"}`, bestNode)
        conn.Write([]byte(response))
}