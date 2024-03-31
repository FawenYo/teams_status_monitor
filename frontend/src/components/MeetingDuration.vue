<script setup>
defineProps({
  meeting_duration: {
    type: String,
    required: true
  }
})
</script>

<template>
  <div class="profile-card-inf">
    <div class="profile-card-inf__item">
      <div class="profile-card-inf__title">
        {{ meeting_duration }}
      </div>
      <div class="profile-card-inf__txt">Meeting Duration</div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.profile-card-inf {
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
  align-items: flex-start;
  margin-top: 35px;

  &__item {
    padding: 10px 35px;
    min-width: 150px;

    @media screen and (max-width: 768px) {
      padding: 10px 20px;
      min-width: 120px;
    }
  }

  &__title {
    font-weight: 700;
    font-size: 27px;
    //color: #6944ff;
    color: #324e63;
  }

  &__txt {
    font-weight: 500;
    margin-top: 7px;
  }
}
</style>
