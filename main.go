package main

import (
	"log"
	"net/http"
	"os"

	"github.com/gin-gonic/gin"
)

func main() {
	port := os.Getenv("PORT")

	if port == "" {
		log.Fatal("$PORT must be set")
	}

	r := gin.Default()
	r.Use(func(c *gin.Context) {
		log.Println("Host:", c.Request.URL.Hostname())

	})

	r.Use(gin.WrapF(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Vary", "Accept-Encoding")
		w.Header().Set("Cache-Control", "public, max-age=7776000")
	}))

	r.Static("/", "./static")

	if err := r.Run(":" + port); err != nil {
		log.Fatal(err)
	}
}
