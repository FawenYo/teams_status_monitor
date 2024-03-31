package utils

import (
	"os"

	"github.com/sirupsen/logrus"
)

var (
	log = logrus.WithField("component", "main")
)

func init() {
	logrus.SetFormatter(&logrus.JSONFormatter{})
	logLevel := os.Getenv("LOG_LEVEL")
	if logLevel == "" {
		logLevel = "info"
	}
	if level, err := logrus.ParseLevel(logLevel); err != nil {
		log.WithField("component", "main").Errorf("Failed to parse log level: %s", err)
		return
	} else {
		logrus.SetLevel(level)
	}
}

func GetLogger() *logrus.Entry {
	return log
}
