package utils

import (
	"github.com/prometheus/client_golang/prometheus"
)

var (
	// TeamsMeetingStatus is a gauge metric that indicates whether a Teams meeting is in progress
	TeamsMeetingStatus = prometheus.NewGaugeVec(prometheus.GaugeOpts{
		Name: "teams_meeting_status",
		Help: "Indicates whether a Teams meeting is in progress",
	}, []string{"user"})

	// TotalTeamsMeetingTime is a counter metric that indicates the total time of Teams meetings
	TotalTeamsMeetingTime = prometheus.NewCounterVec(prometheus.CounterOpts{
		Name: "total_teams_meeting_time",
		Help: "Total time of Teams meetings",
	}, []string{"user"})
)

func init() {
	prometheus.MustRegister(TeamsMeetingStatus)
	prometheus.MustRegister(TotalTeamsMeetingTime)
}
