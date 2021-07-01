<template>
  <header>
    <burger-button :is-active="isActive" @toggle="emitToggle" />
    <div class="logo-container">
      <nuxt-link :to="localePath({ name: 'index' })">
        <img src="~/assets/images/logos/aragog-logo-bleu-2.svg" alt="aragog-icon" />
      </nuxt-link>
    </div>
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
      <a href="https://aragog.rs/book" target="_blank">
        <a-button :invert-color="true">{{ $t('menu.quick-start') }}</a-button>
      </a>
      <a href="https://gitlab.com/qonfucius/aragog" target="_blank">
        <a-button :shadow="true">Gitlab</a-button>
      </a>
    </div>
  </header>
</template>

<script lang="ts">
import Vue from 'vue';

import BurgerButton from '~/components/elements/burger-button';
import AButton from '~/components/elements/button';


export default Vue.extend({
  name: 'Header',
  props: {
    isActive: {
      type: Boolean,
      default: false,
    },
  },
  components: {
    BurgerButton,
    AButton
  },
  methods: {
    emitToggle(): void {
      this.$emit('toggle');
    },
    isLinkActive(routeName: string): boolean {
      return this.localePath({ name: routeName }) === this.$route.path;
    },
  },
});
</script>

<style scoped lang="scss">
@import 'assets/css/_vars.scss';

header {
  box-shadow: 0 10px 6px #53535321;
  min-height: 80px;
  .burger-button {
    float: left;
  }
  .logo-container {
    width: 200px;
    position: absolute;
    transform: translate(-50%);
    left: 50%;
  }
  .links,
  .buttons {
    display: none;
  }
}

@media screen and(min-width: $tablet) {
  header {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    grid-auto-rows: auto;
    grid-gap: $size-extra-large;
    .logo-container {
      place-self: center start;
      position: relative;
    }
    .links {
      display: block;
      ul {
        li {
          display: inline-block;
          padding-right: $size-large;
          a {
            color: $purple;
            text-decoration: none;
            &.active {
              font-weight: bold;
            }
          }
        }
      }
    }
    .buttons {
      display: block;
      .button {
        margin-left: $size-normal;
      }
    }
    div {
      align-content: center;
      place-self: center;
    }
  }
}
</style>

<i18n src="./menu.i18n.yml" lang="yaml" />
