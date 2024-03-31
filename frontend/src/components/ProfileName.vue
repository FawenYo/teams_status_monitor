<script setup>
defineProps({
  user_name: {
    type: String,
    required: true
  }
})
</script>

<template>
  <div class="profile-card__name">
    {{ user_name }}
  </div>
</template>
