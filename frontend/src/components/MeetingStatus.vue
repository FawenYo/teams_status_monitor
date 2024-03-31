<script setup>
defineProps({
  meeting_status: {
    type: String,
    required: true
  },
  status_color: {
    type: String,
    required: true
  }
})
</script>

<template>
  <div class="profile-card__txt">
    Meeting status:
    <strong :style="{ color: status_color }">{{ meeting_status }}</strong>
  </div>
</template>
