<script setup>
const props = defineProps({
  shortcut_url: {
    type: String,
    required: true
  }
})

const openShortcut = () => {
  window.open(props.shortcut_url, '_blank')
}
</script>

<template>
  <div class="profile-card-ctr">
    <button class="profile-card__button button--blue js-shortcut-btn" @click="openShortcut">
      Get iOS Shortcut
    </button>
  </div>
</template>

<style scoped lang="scss">
.profile-card {
  &-ctr {
    display: flex;
    justify-content: center;
    align-items: center;
    margin-top: 40px;

    @media screen and (max-width: 576px) {
      flex-wrap: wrap;
    }
  }

  &__button {
    background: none;
    border: none;
    font-family: 'Quicksand', sans-serif;
    font-weight: 700;
    font-size: 19px;
    margin: 15px 35px;
    padding: 15px 40px;
    min-width: 201px;
    border-radius: 50px;
    min-height: 55px;
    color: #fff;
    cursor: pointer;
    backface-visibility: hidden;
    transition: all 0.3s;

    @media screen and (max-width: 768px) {
      min-width: 170px;
      margin: 15px 25px;
    }

    @media screen and (max-width: 576px) {
      min-width: inherit;
      margin: 0;
      margin-bottom: 16px;
      width: 100%;
      max-width: 300px;

      &:last-child {
        margin-bottom: 0;
      }
    }

    &:focus {
      outline: none !important;
    }

    @media screen and (min-width: 768px) {
      &:hover {
        transform: translateY(-5px);
      }
    }

    &:first-child {
      margin-left: 0;
    }

    &:last-child {
      margin-right: 0;
    }

    &.button--blue {
      background: linear-gradient(45deg, #1da1f2, #0e71c8);
      box-shadow: 0px 4px 30px rgba(19, 127, 212, 0.4);

      &:hover {
        box-shadow: 0px 7px 30px rgba(19, 127, 212, 0.75);
      }
    }

    &.button--orange {
      background: linear-gradient(45deg, #d5135a, #f05924);
      box-shadow: 0px 4px 30px rgba(223, 45, 70, 0.35);

      &:hover {
        box-shadow: 0px 7px 30px rgba(223, 45, 70, 0.75);
      }
    }

    &.button--gray {
      box-shadow: none;
      background: #dcdcdc;
      color: #142029;
    }
  }
}
</style>
