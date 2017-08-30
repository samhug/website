package main

import (
	"log"
	"net/http"
	"os"

	"github.com/gin-gonic/gin"
	"fmt"
)

var PRIMARY_HOST = "sa.m-h.ug"

func main() {
	port := os.Getenv("PORT")

	if port == "" {
		log.Fatal("$PORT must be set")
	}

	r := gin.Default()
	r.Use(func(c *gin.Context) {
		if c.Request.Host != PRIMARY_HOST {
			c.Redirect(http.StatusMovedPermanently, fmt.Sprintf("https://%s%s", PRIMARY_HOST, c.Request.URL))
		}
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
