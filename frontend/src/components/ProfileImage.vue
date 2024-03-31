<script setup>
defineProps({
  profile_img_url: {
    type: String,
    required: true
  }
})
</script>

<template>
  <div class="profile-card__img">
    <!-- Corrected src binding here -->
    <img :src="profile_img_url" alt="profile card" />
  </div>
</template>
