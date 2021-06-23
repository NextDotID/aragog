<template>
  <button :class="classNames" @click="toggleButtonState">
    <span class="bar bar--1" />
    <span class="bar bar--2" />
  </button>
</template>

<script lang="ts">
import Vue from 'vue';

interface ClassNames {
  'burger-button': boolean;
  active: boolean;
}

export default Vue.extend({
  name: 'BurgerButton',
  props: {
    isActive: {
      type: Boolean,
      default: false,
    },
  },
  computed: {
    classNames(): ClassNames {
      return {
        'burger-button': true,
        active: this.isActive,
      };
    },
  },
  methods: {
    toggleButtonState(): void {
      this.$emit('toggle');
    },
  },
});
</script>

<style scoped lang="scss">
@import 'assets/css/vars';

.burger-button {
  position: relative;
  $size: 60px;
  $offsetY: 8px;

  height: $size;
  width: $size;
  cursor: pointer;
  background-color: transparent;

  display: flex;
  justify-content: center;
  align-items: center;

  border: none;
  outline: none;
  border-radius: 50%;

  transition: all 1s;
  margin-top: $size-normal;
  margin-left: $size-normal;

  z-index: 6000 !important;

  .bar {
    position: absolute;

    width: 40px;
    margin: auto;
    height: 2px;
    background-color: $purple;
    transition: transform 1s ease;

    &.bar--1 {
      transform: translate3d(0, -$offsetY, 0);
    }

    &.bar--2 {
      transform: translate3d(0, $offsetY, 0);
    }
  }

  &:hover {

    background-color: $white;

    .bar--1 {
      transform: translate3d(0, -$offsetY, 0) rotate(-180deg);
    }

    .bar--2 {
      transform: translate3d(0, $offsetY, 0) rotate(-180deg);
    }

    &.active {

      .bar {
        background-color: $purple;
      }

      .bar--1 {
        transform: rotate(-495deg);
      }

      .bar--2 {
        transform: rotate(-45deg);
      }
    }
  }

  &.active {
    .bar {
      transition: transform 600ms ease;
      background-color: $white;
    }

    .bar--1 {
      transform: rotate(-675deg);
    }

    .bar--2 {
      transform: rotate(-225deg);
    }
  }
}

@media screen and(min-width: $tablet) {
  .burger-button {
    display: none;
  }
}
</style>
