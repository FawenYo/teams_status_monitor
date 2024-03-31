<script setup>
import { ref } from 'vue'
import ProfileImage from '@/components/ProfileImage.vue'
import ProfileName from '@/components/ProfileName.vue'
import MeetingStatus from '@/components/MeetingStatus.vue'
import MeetingDuration from '@/components/MeetingDuration.vue'
import ShortcutButton from '@/components/ShortcutButton.vue'

// Get URL parameters
const urlParams = new URLSearchParams(window.location.search)

// Define reactive variables
let userPhoto = ref('https://placehold.co/150')
let userName = ref(urlParams.get('user'))
let meetingStatus = ref('meeting status')
let meetingStatusColor = ref('green')
let meetingDuration = ref('00:00:00')
let shortcutUrl = ref('')

// Fetch data from the API
function fetchData() {
  fetch(`/api/monitor/teams?user=${userName.value}`)
    .then((response) => response.json())
    .then((data) => {
      userName.value = data.data.user
      userPhoto.value = data.data.user_icon_url
      meetingStatus.value = data.data.in_meeting ? 'In Meeting' : 'Not in Meeting'
      meetingStatusColor.value = data.data.in_meeting ? 'red' : 'green'
      meetingDuration.value = data.data.meeting_duration
      shortcutUrl.value = data.shortcut_url
    })
    .catch((error) => {
      console.error('There was an error!', error)
    })
}

function incrementMeetingDuration() {
  if (meetingStatus.value === 'In Meeting') {
    // Split the current duration into hours, minutes, and seconds
    let [hours, minutes, seconds] = meetingDuration.value.split(':').map(Number)

    // Increment the seconds
    seconds++

    // Check if seconds exceed 59, then increment minutes and reset seconds
    if (seconds > 59) {
      minutes++
      seconds = 0
    }

    // Check if minutes exceed 59, then increment hours and reset minutes
    if (minutes > 59) {
      hours++
      minutes = 0
    }

    // Ensure double digits for hours, minutes, and seconds
    const pad = (num) => num.toString().padStart(2, '0')

    // Update the meetingDuration with the new time
    meetingDuration.value = `${pad(hours)}:${pad(minutes)}:${pad(seconds)}`
  }
}

// Fetch data on component mount
fetchData()
// Update meeting status every minute
setInterval(fetchData, 60000)
// Increment meeting duration every second
setInterval(incrementMeetingDuration, 1000)
</script>

<template>
  <div class="wrapper">
    <div class="profile-card js-profile-card">
      <ProfileImage :profile_img_url="userPhoto" />

      <div class="profile-card__cnt js-profile-cnt">
        <ProfileName :user_name="userName" />
        <MeetingStatus :meeting_status="meetingStatus" :status_color="meetingStatusColor" />
        <MeetingDuration :meeting_duration="meetingDuration" />
        <ShortcutButton :shortcut_url="shortcutUrl" />
      </div>
    </div>
  </div>
</template>

<style lang="scss">
@import url('https://fonts.googleapis.com/css?family=Quicksand:400,500,700&subset=latin-ext');

* {
  box-sizing: border-box;
}

body {
  font-family: 'Quicksand', sans-serif;
  color: #324e63;
}

a,
a:hover {
  text-decoration: none;
}

.icon {
  display: inline-block;
  width: 1em;
  height: 1em;
  stroke-width: 0;
  stroke: currentColor;
  fill: currentColor;
}

.wrapper {
  width: 100%;
  width: 100%;
  height: auto;
  min-height: 100vh;
  padding: 50px 20px;
  padding-top: 100px;
  display: flex;
  background-image: linear-gradient(-20deg, #ff2846 0%, #6944ff 100%);

  display: flex;
  background-image: linear-gradient(-20deg, #ff2846 0%, #6944ff 100%);

  @media screen and (max-width: 768px) {
    height: auto;
    min-height: 100vh;
    padding-top: 100px;
  }
}

.profile-card {
  width: 100%;
  min-height: 460px;
  margin: auto;
  box-shadow: 0px 8px 60px -10px rgba(13, 28, 39, 0.6);
  background: #fff;
  border-radius: 12px;
  max-width: 700px;
  position: relative;

  &.active {
    .profile-card__cnt {
      filter: blur(6px);
    }

    .profile-card-message,
    .profile-card__overlay {
      opacity: 1;
      pointer-events: auto;
      transition-delay: 0.1s;
    }

    .profile-card-form {
      transform: none;
      transition-delay: 0.1s;
    }
  }

  &__img {
    width: 150px;
    height: 150px;
    margin-left: auto;
    margin-right: auto;
    transform: translateY(-50%);
    border-radius: 50%;
    overflow: hidden;
    position: relative;
    z-index: 4;
    box-shadow:
      0px 5px 50px 0px rgb(108, 68, 252),
      0px 0px 0px 7px rgba(107, 74, 255, 0.5);

    @media screen and (max-width: 576px) {
      width: 120px;
      height: 120px;
    }

    img {
      display: block;
      width: 100%;
      height: 100%;
      object-fit: cover;
      border-radius: 50%;
    }
  }

  &__cnt {
    margin-top: -35px;
    text-align: center;
    padding: 0 20px;
    padding-bottom: 40px;
    transition: all 0.3s;
  }

  &__name {
    font-weight: 700;
    font-size: 24px;
    color: #6944ff;
    margin-bottom: 15px;
  }

  &__txt {
    font-size: 18px;
    font-weight: 500;
    color: #324e63;
    margin-bottom: 15px;

    strong {
      //color: #ff2846;
      font-weight: 700;
    }
  }

  &-form {
    box-shadow: 0 4px 30px rgba(15, 22, 56, 0.35);
    max-width: 80%;
    margin-left: auto;
    margin-right: auto;
    height: 100%;
    background: #fff;
    border-radius: 10px;
    padding: 35px;
    transform: scale(0.8);
    position: relative;
    z-index: 3;
    transition: all 0.3s;

    @media screen and (max-width: 768px) {
      max-width: 90%;
      height: auto;
    }

    @media screen and (max-width: 576px) {
      padding: 20px;
    }

    &__bottom {
      justify-content: space-between;
      display: flex;

      @media screen and (max-width: 576px) {
        flex-wrap: wrap;
      }
    }
  }

  textarea {
    width: 100%;
    resize: none;
    height: 210px;
    margin-bottom: 20px;
    border: 2px solid #dcdcdc;
    border-radius: 10px;
    padding: 15px 20px;
    color: #324e63;
    font-weight: 500;
    font-family: 'Quicksand', sans-serif;
    outline: none;
    transition: all 0.3s;

    &:focus {
      outline: none;
      border-color: #8a979e;
    }
  }

  &__overlay {
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    left: 0;
    pointer-events: none;
    opacity: 0;
    background: rgba(22, 33, 72, 0.35);
    border-radius: 12px;
    transition: all 0.3s;
  }
}
</style>
