<template>
  <nav :class="classNames">
    <div class="links">
      <ul>
        <li>
          <nuxt-link :to="localePath({ name: 'index' })" :class="{ active: isLinkActive('index') }"
          >
            {{ $t('menu.home') }}
          </nuxt-link>
        </li>
        <li>
          <nuxt-link :to="localePath({ name: 'community' })" :class="{ active: isLinkActive('community') }">
            {{ $t('menu.community') }}
          </nuxt-link>
        </li>
      </ul>
    </div>
    <div class="buttons">
      <ul>
        <li>
          <a href="https://qonfucius.gitlab.io/aragog/" target="_blank">
            {{ $t('menu.quick-start') }}
          </a>
        </li>
        <li>
          <a href="https://gitlab.com/qonfucius/aragog" target="_blank">
            Gitlab
          </a>
        </li>
      </ul>
    </div>
  </nav>
</template>

<script lang="ts">
import Vue from 'vue';


export default Vue.extend({
  name: 'MenuMobile',
  props: {
    isActive: {
      type: Boolean,
      default: false,
    },
  },
  computed: {
    classNames() {
      return {
        'mobile-menu': true,
        active: this.isActive,
      };
    },
  },
  methods: {
    isLinkActive(routeName: string): boolean {
      return this.localePath({ name: routeName })  === this.$route.path;
    },
  },
});
</script>

<style scoped lang="scss">
@import 'assets/css/vars';

.mobile-menu {
  position: fixed;
  z-index: 5000;
  top: 0;
  left: -100%;
  height: 100%;
  width: 100%;
  background-color: $purple;
  color: $white;
  transition: all 1s;

  .links {
    margin-top: $size-extra-large;
  }

  .links, .buttons {
    padding-left: 10%;
    ul {
      list-style-type: none;
      li {
        a {
          color: $white;
          text-decoration: none;
          font-weight: bold;
          &.active {
            color: #3636e5 !important;
          }
        }
      }
    }
  }

  &.active {
    left: 0;
    transition: all 1s;

    ul {
      opacity: 1;
    }
  }

  ul {
    opacity: 0;
    margin-top: 100px;
    transition: all 0.5s;
    font-size: $size-large;

    .navigation-item {
      margin-bottom: $size-normal;
      margin-left: 10%;
      &.active {
        display: block;
        align-items: center;
        border: 2px solid $purple;
        border-left: none;
        border-right: none;
        text-decoration: none;
        margin-left: 0;
        padding: 10px 0 10px 10%;
        width: 100%;
        margin-bottom: $size-normal * 2;
        &:after {
          margin: 0;
          height: 0;
        }
      }
    }
  }
}

@media screen and(min-width: $mobile) {
  .menu-mobile {
    display: none;

    ul {
      font-size: $size-extra-large;
    }
  }
}

@media screen and(min-width: $tablet) {
  .menu-mobile {
    display: none;
  }
}
</style>

<i18n src="./menu.i18n.yml" lang="yaml" />
