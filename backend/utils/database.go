package utils

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/redis/go-redis/v9"
)

var (
	rdb *redis.Client
	ctx = context.Background()

	usersKey = fmt.Sprintf("%s_users", APP_NAME)
)

// InitializeDB initializes the database connection
func InitializeDB(databaseURL string, databasePort string, databasePassword string, databaseName int, ssl bool) {
	log.WithField("component", "database").
		WithField("databaseURL", databaseURL).
		WithField("databasePort", databasePort).
		WithField("databaseName", databaseName).
		WithField("ssl", ssl).Info("Initializing database connection")
	rdb = redis.NewClient(&redis.Options{
		Addr:     databaseURL + ":" + databasePort,
		Password: databasePassword,
		DB:       databaseName,
	})
}

func getKey(key string) (string, error) {
	val, err := rdb.Get(ctx, key).Result()
	if err != nil {
		return "", err
	}
	return val, nil
}

func setKey(key string, value string) error {
	err := rdb.Set(ctx, key, value, 0).Err()
	if err != nil {
		return err
	}
	return nil
}

func (meetingStatus MeetingStatus) encodeJSON() string {
	json, _ := json.Marshal(meetingStatus)
	return string(json)
}

func (meetingStatus *MeetingStatus) decodeJSON(jsonString string) error {
	err := json.Unmarshal([]byte(jsonString), meetingStatus)
	if err != nil {
		return err
	}
	return nil
}

func GetUsers() (UserStatus, error) {
	val, err := getKey(usersKey)
	if err != nil {
		log.WithField("component", "updateTeamsStatus").Errorf("Failed to get users: %s", err)
		return UserStatus{}, err
	}

	type usersType struct {
		Users []string `json:"users"`
	}

	users := usersType{}
	if err = json.Unmarshal([]byte(val), &users); err != nil {
		log.WithField("component", "updateTeamsStatus").Errorf("Failed to unmarshal users: %s", err)
		return UserStatus{}, err
	}

	result := UserStatus{Users: make(map[string]MeetingStatus)}

	for _, user := range users.Users {
		userStatusKey := fmt.Sprintf("%s_%s_status", APP_NAME, user)
		userStatusVal, err := getKey(userStatusKey)
		if err != nil {
			log.WithField("component", "updateTeamsStatus").Errorf("Failed to get user status: %s", err)
			return UserStatus{}, err
		}
		meetingStatus := MeetingStatus{}
		if err = meetingStatus.decodeJSON(userStatusVal); err != nil {
			log.WithField("component", "updateTeamsStatus").Errorf("Failed to decode user status: %s", err)
			return UserStatus{}, err
		}
		result.Users[user] = meetingStatus
	}
	return result, nil
}

func SetUsers(userStatus UserStatus) error {
	// Add user name to the Redis database
	usersList := make([]string, 0, len(userStatus.Users))
	for user := range userStatus.Users {
		usersList = append(usersList, user)
	}

	// Update the users in the Redis database
	type data struct {
		Users []string `json:"users"`
	}
	usersData := data{Users: usersList}
	users, err := json.Marshal(usersData)
	if err != nil {
		log.WithField("component", "updateTeamsStatus").Errorf("Failed to marshal users: %s", err)
		return err
	}

	if err := setKey(usersKey, string(users)); err != nil {
		log.WithField("component", "updateTeamsStatus").Errorf("Failed to set users: %s", err)
		return err
	}
	return nil
}

func SetUserStatus(key string, meetingStatus MeetingStatus) error {
	if err := setKey(key, meetingStatus.encodeJSON()); err != nil {
		log.WithField("component", "updateTeamsStatus").Errorf("Failed to set user status: %s", err)
		return err
	}
	return nil
}
