package utils

type DataBaseConfig struct {
	DBHost     string
	DBPort     string
	DBPassword string
	DBName     int  `default:"0"`
	SSL        bool `default:"false"`
}

type UserStatus struct {
	Users map[string]MeetingStatus
}

type MeetingStatus struct {
	Username         string `json:"username"`
	UserIconURL      string `json:"user_icon_url"`
	InMeeting        bool   `json:"in_meeting"`
	MeetingStartTime int64  `json:"meeting_start_time"`
	LastSyncTime     int64  `json:"last_sync_time"`
	TotalMeetingTime int64  `json:"total_meeting_time"`
}
