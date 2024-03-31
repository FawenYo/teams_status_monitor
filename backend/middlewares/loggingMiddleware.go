package middlewares

import (
	"time"

	"github.com/gin-gonic/gin"
	"github.com/sirupsen/logrus"
)

var (
	whitelist = []string{
		"/healthz",
		"/metrics",
	}
)

func LoggingMiddleware() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		// Starting time
		startTime := time.Now()

		// Processing request
		ctx.Next()

		// End Time
		endTime := time.Now()

		// execution time
		latencyTime := endTime.Sub(startTime)

		// Request method
		reqMethod := ctx.Request.Method

		// Request route
		reqUri := ctx.Request.RequestURI

		// status code
		statusCode := ctx.Writer.Status()

		// log only when path is not in whitelist
		for _, path := range whitelist {
			if path == reqUri {
				return
			}
		}

		logrus.WithFields(logrus.Fields{
			"method":  reqMethod,
			"uri":     reqUri,
			"status":  statusCode,
			"latency": latencyTime,
		}).Info("http request")

		ctx.Next()
	}
}
