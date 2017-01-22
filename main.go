package main

import (
	"log"
	"os"

	"github.com/gin-gonic/gin"
)

func main() {
	port := os.Getenv("PORT")

	if port == "" {
		log.Fatal("$PORT must be set")
	}

	router := gin.Default()
	router.Static("/", "./static")
	if err := router.Run(":" + port); err != nil {
		log.Fatal(err)
	}
}
