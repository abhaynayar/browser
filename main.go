package main

import (
    "fmt"
    "net"
    "strings"
)

func main() {
    url := "http://example.org/index.html"
    if !strings.HasPrefix(url, "http://") {
        panic("Unsupported scheme")
    }

    url = url[len("http://"):]
    host := strings.SplitN(url,"/",2)[0]
    path := "/" + strings.SplitN(url,"/",2)[1]

    con, err := net.Dial("tcp", host + ":80")
    if err != nil {
        fmt.Println("Could not connect to host")
    }

    msg := "GET " + path + " HTTP/1.0\r\n" +
            "Host: " + host + "\r\n" +
            "\r\n";
    _, err = con.Write([]byte(msg))
    if err != nil {
        fmt.Println("Could not write to socket")
    }

    reply := make([]byte, 4096)
    _, err = con.Read(reply)
    if err != nil {
        fmt.Println("Could not read from socket")
    }

    response := strings.Split(string(reply), "\r\n")
    statusLine := response[0]
    //version := strings.SplitN(statusLine, " ", 3)[0]
    status := strings.SplitN(statusLine, " ", 3)[1]
    //explanation := strings.SplitN(statusLine, " ", 3)[2]

    if status != "200" {
        fmt.Println("Didn't receive 200 HTTP status code")
    }
}
