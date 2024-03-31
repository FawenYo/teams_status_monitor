package utils

import (
	"os"
	"path/filepath"
	"reflect"
)

const (
	APP_NAME = "teams_meeting_monitor"
)

func readConfig(configPath string) (string, error) {
	result, err := os.ReadFile(configPath)
	if err != nil {
		log.WithField("component", "config").Errorf("Failed to read config file: %s", err)
		return "", err
	}
	return string(result), nil
}

func initDB() {
	// Initialize the database connection
	var err error
	dataBaseConfig := DataBaseConfig{}
	configPath := os.Getenv("CONFIG_PATH")
	secretPath := os.Getenv("SECRET_PATH")

	configKeys := []string{"DBHost", "DBPort"}

	// Use reflection to set the values dynamically
	v := reflect.ValueOf(&dataBaseConfig).Elem()
	for _, key := range configKeys {
		field := v.FieldByName(key)
		if !field.IsValid() {
			log.WithField("component", "config").Errorf("Invalid field name: %s", key)
			return
		}

		filePath := filepath.Join(configPath, key)
		value, err := readConfig(filePath)
		if err != nil {
			log.WithField("component", "config").Errorf("Failed to read config file: %s", err)
			return
		}
		field.SetString(value)
	}

	if dataBaseConfig.DBPassword, err = readConfig(filepath.Join(secretPath, "DBPassword")); err != nil {
		log.WithField("component", "config").Errorf("Failed to read config file: %s", err)
		return
	}
	InitializeDB(dataBaseConfig.DBHost, dataBaseConfig.DBPort, dataBaseConfig.DBPassword, dataBaseConfig.DBName, dataBaseConfig.SSL)
}

func InitializeConfig() {
	initDB()
}
