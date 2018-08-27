package main

import (
	"net"
	"time"
	"encoding/binary"
)

func main()  {
	conn, _ := net.Dial("tcp", "localhost:12345")

	name := "haha"
	binary.Write(conn, binary.LittleEndian, uint16(len(name)))
	for i:=0; i< 1000; i++ {

	conn.Write([]byte(name))
	}

	time.Sleep(time.Second)
}