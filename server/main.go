package main

import (
	"fmt"
	"log"
	"net"
)

func worker(connection *net.UDPConn, jobs <- chan packet) {
	for p := range jobs {
		fmt.Printf("Handling message from %s, with game data size: %d\n", p.clientAddress, len(p.data))
		_, err := connection.WriteToUDP(p.data, p.clientAddress)
		if err != nil {
			fmt.Printf("Failed handling message: %s", err)
		}
	}
}

type packet struct {
	clientAddress *net.UDPAddr
	data []byte
}

func main() {
	udpAddress, err := net.ResolveUDPAddr("udp", "localhost:8080")
	if err != nil {
		log.Fatal("Couldn't resolve address: ", err)
	}

	connection, err := net.ListenUDP("udp", udpAddress)
	if err != nil {
		log.Fatal("Couldn't connect to address: ", err)
	}

	defer connection.Close()

	jobs := make(chan packet, 100)

	for range 4 {
		go worker(connection, jobs)
	}

	fmt.Printf("Server listening on: %s\n", udpAddress)

	buffer := make([]byte, 1024)
	for {
		n, clientAddress, err := connection.ReadFromUDP(buffer)
		if err != nil {
			log.Printf("Read error: %s", err)
			continue
		}

		data := make([]byte, 1024)
		copy(data, buffer[:n])

		jobs <- packet{ data: buffer[:n], clientAddress: clientAddress }
	}
}
