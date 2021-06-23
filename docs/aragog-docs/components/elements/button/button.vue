<template>
  <button type="button" class="button" :class="classNames" :disabled="disabled">
    <slot />
  </button>
</template>

<script lang="ts">
import Vue from 'vue';

export default Vue.extend({
  props: {
    disabled: {
      type: Boolean,
      default: false,
    },
    shadow: {
      type: Boolean,
      default: false,
    },
    invertColor: {
      type: Boolean,
      default: false,
    },
    large: {
      type: Boolean,
      default: false,
    }
  },
  computed: {
    classNames() {
      return {
        disabled: this.disabled,
        active: !this.disabled,
        shadow: this.shadow,
        invertColor: this.invertColor,
        large: this.large,
      };
    },
  },
});
</script>

<style scoped lang="scss">
@use "assets/css/vars";

.button {
  position: relative;
  margin: vars.$size-extra-large auto;
  background-color: vars.$white;
  color: vars.$purple;
  border: 1px solid transparent;
  border-radius: 100px;
  padding: vars.$size-extra-small vars.$size-extra-large;
  transition: padding 0.2s;
  font-size: vars.$size-normal;
  cursor: pointer;
  overflow: hidden;
  z-index: 1;
  &:after {
    position: relative;
    right: 25px;
    opacity: 0;
    transition: all 0.2s;
    content: '';
  }
  &:hover {
    padding: vars.$size-extra-small vars.$size-extra-large * 1.5 vars.$size-extra-small vars.$size-extra-large !important;
    transition: padding 0.2s;
    &:after {
      position: absolute;
      right: 15px;
      opacity: 1;
      transition: all 0.2s;
      content: ' \2192';
    }
  }
  &.large {
    padding: vars.$size-small vars.$size-large * 2;
    font-size: vars.$size-medium;
    &:after {
      right: 40px;
    }
    &:hover {
      padding: vars.$size-small vars.$size-large * 2.5 vars.$size-small vars.$size-large * 2 !important;
      &:after {
        right: 20px;
      }
    }
  }
  &.invertColor {
    background-color: vars.$purple;
    color: vars.$white;
  }
  &.shadow {
    box-shadow: 0 3px 10px #6A6AE639;
  }
  &.disabled {
    border: 1px solid vars.$white;
    color: vars.$white;
    cursor: default;
  }
}
</style>
