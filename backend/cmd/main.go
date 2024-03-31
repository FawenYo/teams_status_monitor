package main

import (
	"fmt"
	"teams_meeting_monitor_backend/middlewares"
	"teams_meeting_monitor_backend/utils"
	"time"

	"github.com/chenjiandongx/ginprom"
	"github.com/gin-gonic/gin"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

const (
	shortcut_url = "https://www.icloud.com/shortcuts/fe4504fce4464b3b83cfc1c52882cb4f"
)

var (
	log        = utils.GetLogger()
	userStatus = utils.UserStatus{Users: make(map[string]utils.MeetingStatus)}
)

func init() {
	// Initialize the database connection
	utils.InitializeConfig()
	// Get all users from the Redis database
	var err error
	if userStatus, err = utils.GetUsers(); err != nil {
		log.WithField("component", "init").Errorf("Failed to get users: %s", err)
		userStatus = utils.UserStatus{Users: make(map[string]utils.MeetingStatus)}
	}
}

func main() {
	// Run the meeting metrics update in a goroutine
	go meetingMetricsUpdate()
	// Check if the user is in a meeting every 10 minutes
	go checkMeetingStatus()

	// Create a new router
	r := gin.New()
	r.Use(middlewares.LoggingMiddleware())
	r.Use(ginprom.PromMiddleware(nil))
	// Register the `/metrics` route.
	r.GET("/metrics", ginprom.PromHandler(promhttp.Handler()))

	// Teams monitor endpoint
	r.GET("/api/monitor/teams", getTeamsStatus)
	r.POST("/api/monitor/teams", updateTeamsStatus)
	// Health check endpoint
	r.GET("/healthz", utils.HealthHandler)

	// Start the server
	err := r.Run(":8000")
	if err != nil {
		log.Fatal(err)
	}
}

func getTeamsStatus(c *gin.Context) {
	// Get username from query parameter
	username := c.Query("user")

	if username == "" {
		c.JSON(400, gin.H{"error": "user query parameter is required"})
		return
	}

	// Check if user is in a meeting
	userIconURL := "https://cdn.iconscout.com/icon/free/png-256/avatar-370-456322.png"
	meetingDuration := int64(0)
	inMeeting := false

	if val, ok := userStatus.Users[username]; ok {
		userIconURL = val.UserIconURL
		inMeeting = val.InMeeting
		if inMeeting {
			meetingDuration = time.Now().Unix() - val.MeetingStartTime
		}
	}
	meetingDurationStr := meetingDurationProcess(meetingDuration)

	c.JSON(200, gin.H{
		"status": "success",
		"data": gin.H{
			"user":             username,
			"user_icon_url":    userIconURL,
			"in_meeting":       inMeeting,
			"meeting_duration": meetingDurationStr,
		},
		"shortcut_url": shortcut_url,
	})
}

func updateTeamsStatus(c *gin.Context) {
	// Validate the JSON
	type updateJSON struct {
		MeetingStatus bool   `json:"meeting_status"`
		User          string `json:"user"`
		UserIconURL   string `json:"user_icon_url"`
	}

	var json updateJSON
	if err := c.ShouldBindJSON(&json); err != nil {
		c.JSON(400, gin.H{"error": err.Error()})
		return
	}

	log.WithField("component", "updateTeamsStatus").Debugf("Received JSON: %+v", json)
	username := json.User
	userIconURL := json.UserIconURL

	meetingStatus := utils.MeetingStatus{
		Username:    username,
		UserIconURL: userIconURL,
	}
	if val, ok := userStatus.Users[username]; ok {
		meetingStatus = val
	} else {
		// Initialize the Prometheus metric
		utils.TeamsMeetingStatus.WithLabelValues(username).Set(0)
		utils.TotalTeamsMeetingTime.WithLabelValues(username).Add(0)
	}
	// Update user icon URL
	meetingStatus.UserIconURL = userIconURL
	// Update last sync time
	meetingStatus.LastSyncTime = time.Now().Unix()

	if json.MeetingStatus {
		if !meetingStatus.InMeeting {
			meetingStatus.MeetingStartTime = time.Now().Unix()
		}
		meetingStatus.InMeeting = true
	} else {
		meetingStatus = meetingPostHandler(meetingStatus)
	}

	// Update user status
	userStatus.Users[username] = meetingStatus
	// Update user status in the Redis database
	log.WithField("component", "updateTeamsStatus").Info("Updating users")
	if err := utils.SetUsers(userStatus); err != nil {
		log.WithField("component", "updateTeamsStatus").Errorf("Failed to set users: %s", err)
		c.JSON(500, gin.H{"error": "Failed to set users"})
		return
	}

	// Encode the user status to JSON and store it in the Redis database
	log.WithField("component", "updateTeamsStatus").Infof("Updating user status: %s", username)
	if err := utils.SetUserStatus(fmt.Sprintf("%s_%s_status", utils.APP_NAME, username), meetingStatus); err != nil {
		log.WithField("component", "updateTeamsStatus").Errorf("Failed to set users: %s", err)
		c.JSON(500, gin.H{"error": "Failed to set users"})
		return
	}
}

func meetingPostHandler(meetingStatus utils.MeetingStatus) utils.MeetingStatus {
	// Calculate meeting duration
	meetingDuration := time.Now().Unix() - meetingStatus.MeetingStartTime
	meetingDurationStr := meetingDurationProcess(meetingDuration)
	log.WithField("component", "meetingPostHandler").WithField("user", meetingStatus.Username).Infof("Meeting duration: %s", meetingDurationStr)

	// Update meeting status
	meetingStatus.InMeeting = false
	meetingStatus.MeetingStartTime = 0
	meetingStatus.TotalMeetingTime += meetingDuration

	// Update Prometheus metric
	utils.TeamsMeetingStatus.WithLabelValues(meetingStatus.Username).Set(0)
	return meetingStatus
}

func meetingDurationProcess(seconds int64) string {
	// Convert seconds to hours:minutes:seconds
	hours := seconds / 3600
	remainder := seconds - hours*3600
	minutes := remainder / 60
	seconds = remainder - minutes*60

	return fmt.Sprintf("%02d:%02d:%02d", hours, minutes, seconds)
}

func meetingMetricsUpdate() {
	for {
		for user, status := range userStatus.Users {
			if status.InMeeting {
				meetingDuration := time.Now().Unix() - status.MeetingStartTime
				utils.TeamsMeetingStatus.WithLabelValues(user).Set(float64(meetingDuration))
				utils.TotalTeamsMeetingTime.WithLabelValues(user).Add(1)
			}
		}
		time.Sleep(1 * time.Second)
	}
}

func checkMeetingStatus() {
	for {
		for _, status := range userStatus.Users {
			if status.InMeeting {
				durationFromLastSync := time.Now().Unix() - status.LastSyncTime
				// If the user has not synced the status for 10 minutes, consider the user is not in a meeting
				if durationFromLastSync > 600 {
					meetingPostHandler(status)
				}
			}
		}
		time.Sleep(1 * time.Second)
	}
}
