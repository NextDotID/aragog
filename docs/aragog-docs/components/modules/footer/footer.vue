<template>
  <div class="footer">
    <ul>
      <li>
        <a href="https://qongzi.com" target="_blank">Qongzi</a>
      </li>
      <li>
        <a href="https://qonfucius.com" target="_blank">Qonfucius</a>
      </li>
      <li>
        <nuxt-link :to="localePath({ name: 'legal-notices' })" :class="classNames">
          {{ $t('legal-notices') }}
        </nuxt-link>
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import Vue from 'vue';

interface State {
  classNames: {
    link: boolean;
    'is-active': boolean;
  };
}

export default Vue.extend({
  name: 'MenuFooter',
  data(): State {
    return {
      classNames: {
        link: true,
        // @ts-ignore
        'is-active': this.isLinkActive('legal-notices'),
      },
    };
  },
  methods: {
    isLinkActive(routeName: string): boolean {
      return routeName === this.$route.path;
    },
  },
});
</script>

<style scoped lang="scss">
@import 'assets/css/vars';

.footer {
  background-color: $white;
  margin: $size-normal 0;
  padding: 0 $size-large;
  text-align: center;

  ul {
    list-style-type: none;
    padding: 0;
    margin: 0;
    li {
      padding-top: $size-normal;
      &:first-child {
        padding-top: 0;
      }
    }
  }

  a {
    color: $black;
    text-decoration: none;
    font-weight: 100;
  }
  .is-active {
    font-weight: bold;
  }
}

@media screen and(min-width: $mobile) {
  .footer {
    margin: $size-extra-large 0;
    text-align: left;
    ul {
      li {
        display: inline-block;
        padding: 0 $size-large;

        &:last-child {
          position: absolute;
          left: 70%;
        }
      }
    }
  }
}

@media screen and(min-width: $tablet) {
  .footer {
    margin: $size-extra-large 0;
    text-align: left;
    ul {
      li {
        display: inline-block;
        padding: 0 $size-large;
        &:last-child {
          position: absolute;
          left: 50%;
          transform: translateX(-50%);
        }
      }
    }
  }
}
</style>

<i18n src="./footer.i18n.yml" lang="yaml" />
